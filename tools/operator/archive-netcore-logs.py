#!/usr/bin/env python3
"""Archive closed Linux logs on a verified NAS, then release their local bytes."""
from __future__ import annotations

import argparse
from contextlib import contextmanager
from datetime import datetime
import fcntl
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import socket
import stat
import subprocess
import sys
import tempfile
import time
import uuid
from zoneinfo import ZoneInfo

DEFAULT_CONFIG = Path("/etc/netcore/log-archive.json")
STATE = Path("/var/lib/netcore-log-archive")
PROGRAM = Path("/opt/netcore-log-archive/archive-netcore-logs.py")
RSYSLOG_RULE = Path("/etc/logrotate.d/rsyslog")
GLOBAL_ROTATION = Path("/etc/logrotate.conf")
SYSTEMD_UNITS = Path("/etc/systemd/system")
NETWORK_FILESYSTEMS = {"nfs", "nfs4", "cifs", "smb3"}
ROTATED = re.compile(
    r".+(?:\.\d+(?:\.(?:gz|xz|bz2|zst))?"
    r"|-\d{8}(?:-\d{6})?(?:\.(?:gz|xz|bz2|zst))?"
    r"|\.log\.\d{4}-\d{2}-\d{2}(?:\.gz)?)$"
)
JOURNAL = re.compile(r"(?:system|user-\d+)@[0-9a-f-]+\.journal~?$")
CHUNK = 1024 * 1024


class Progress:
    def __init__(self, source, phase, total):
        self.source, self.phase, self.total = str(source), phase, total
        self.last = 0.0

    def __call__(self, completed):
        now = time.monotonic()
        if now - self.last >= 5 or completed == self.total:
            print(json.dumps({"phase": self.phase, "source": self.source,
                              "bytes": completed, "total_bytes": self.total,
                              "percent": round(100 * completed / max(1, self.total), 1)}),
                  file=sys.stderr, flush=True)
            self.last = now


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def signature(info):
    return (info.st_dev, info.st_ino, info.st_size,
            info.st_mtime_ns, info.st_ctime_ns)


def unslash(value):
    return re.sub(r"\\([0-7]{3})", lambda m: chr(int(m[1], 8)), value)


def network_mount(path, mountinfo=None):
    """Use the longest enclosing mount; a directory below /mnt is not a mount."""
    path = Path(path).absolute()
    for part in (path, *path.parents):
        require(not part.is_symlink(), "Symlink im NAS-Pfad: " + str(part))
    ancestor = path
    while not ancestor.exists():
        require(ancestor.parent != ancestor, "Kein erreichbarer Archivpfad")
        ancestor = ancestor.parent
    resolved = ancestor.resolve(strict=True)
    entries = (Path("/proc/self/mountinfo").read_text()
               if mountinfo is None else mountinfo)
    matches = []
    for line in entries.splitlines():
        left, sep, right = line.partition(" - ")
        fields, fs_fields = left.split(), right.split()
        if not sep or len(fields) < 5 or len(fs_fields) < 2:
            continue
        target = Path(unslash(fields[4]))
        if resolved == target or target in resolved.parents:
            matches.append((len(target.parts), target, fs_fields[0], fs_fields[1]))
    require(bool(matches), "Kein Mount zum Archivpfad gefunden")
    _, target, kind, source = max(matches, key=lambda item: item[0])
    require(kind in NETWORK_FILESYSTEMS,
            f"Archivziel liegt nicht auf NFS/SMB: {target} ({kind})")
    return {"target": str(target), "fstype": kind, "source": unslash(source)}


def sync_directory(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def safe_mkdir(path):
    """Never follow a pre-existing symlink on a shared archive."""
    path = Path(path)
    for part in reversed((path, *path.parents)):
        if not part.exists():
            part.mkdir(mode=0o750)
            sync_directory(part.parent)
        require(not part.is_symlink() and part.is_dir(),
                "Kein echtes Archivverzeichnis: " + str(part))


def atomic_json(path, value):
    safe_mkdir(path.parent)
    fd, temporary = tempfile.mkstemp(prefix=".netcore-receipt-", dir=path.parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as stream:
            json.dump(value, stream, ensure_ascii=False, indent=2)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, 0o640)
        os.replace(temporary, path)
        sync_directory(path.parent)
    finally:
        Path(temporary).unlink(missing_ok=True)


def file_is_open(path):
    """A rotated file can still have a writer until its daemon has reopened."""
    info = path.stat(follow_symlinks=False)
    for process in Path("/proc").iterdir():
        if not process.name.isdigit():
            continue
        try:
            descriptors = list((process / "fd").iterdir())
        except FileNotFoundError:
            continue
        except PermissionError as exc:
            raise RuntimeError("Prozess-FDs nicht vollständig prüfbar") from exc
        for descriptor in descriptors:
            try:
                opened = descriptor.stat()
            except (FileNotFoundError, ProcessLookupError):
                continue
            except PermissionError as exc:
                raise RuntimeError("Prozess-FD nicht prüfbar: " + str(descriptor)) from exc
            if (opened.st_dev, opened.st_ino) == (info.st_dev, info.st_ino):
                return True
    return False


def candidates(root):
    root = Path(root).resolve(strict=True)
    device = root.stat().st_dev
    for directory, subdirs, files in os.walk(root, followlinks=False):
        parent = Path(directory)
        subdirs[:] = [
            name for name in sorted(subdirs)
            if not (parent / name).is_symlink()
            and (parent / name).stat().st_dev == device
        ]
        for name in sorted(files):
            path = parent / name
            info = path.lstat()
            if not stat.S_ISREG(info.st_mode) or info.st_dev != device:
                continue
            relative = path.relative_to(root)
            is_journal = relative.parts[0] == "journal" and bool(JOURNAL.fullmatch(name))
            if is_journal or (relative.parts[0] != "journal" and ROTATED.fullmatch(name)):
                yield path, relative, is_journal


def digest_file(path, compressed, progress=None):
    digest = hashlib.sha256()
    length = 0
    require(not path.is_symlink(), "Symlink statt Archivdatei: " + str(path))
    with (gzip.open(path, "rb") if compressed else path.open("rb")) as stream:
        while block := stream.read(CHUNK):
            length += len(block)
            digest.update(block)
            if progress is not None:
                progress(length)
    return digest.hexdigest(), length


def archive_one(source, relative, archive, host, timezone, *, is_journal=False):
    """Commit and independently re-read the NAS copy before unlinking a closed log."""
    original = source.lstat()
    require(stat.S_ISREG(original.st_mode), "Quelle ist keine reguläre Datei")
    if file_is_open(source):
        return {"source": str(source), "status": "open", "released_bytes": 0}

    day = datetime.fromtimestamp(original.st_mtime, ZoneInfo(timezone))
    destination = archive / host / day.strftime("%Y/%m/%d") / relative.parent
    network_mount(archive)
    safe_mkdir(destination)
    compressed = not is_journal and source.suffix not in {".gz", ".xz", ".bz2", ".zst"}
    suffix = ".gz" if compressed else (".journal" if is_journal else source.suffix)
    temporary = destination / (
        f".netcore-log-{original.st_dev}-{original.st_ino}-"
        f"{original.st_mtime_ns}-{original.st_size}.part")
    if temporary.exists() or temporary.is_symlink():
        require(not temporary.is_symlink() and temporary.is_file(),
                "Ungültige frühere Teildatei im Archiv")
        require(not file_is_open(temporary), "Frühere NAS-Teildatei ist noch geöffnet")
        # A killed process may have left this exact source's incomplete copy.
        # It is never an archive receipt and may be restarted without accumulating
        # another giant partial copy on every retry.
        temporary.unlink()
    digest = hashlib.sha256()
    copied = 0
    progress = Progress(source, "copying_to_nas", original.st_size)
    try:
        source_fd = os.open(source, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(source_fd, "rb") as reader:
            require(signature(os.fstat(reader.fileno())) == signature(original),
                    "Quelldatei wurde vor dem Lesen ersetzt")
            with temporary.open("xb") as output:
                if compressed:
                    writer = gzip.GzipFile(filename="", mode="wb", compresslevel=1,
                                           fileobj=output, mtime=0)
                else:
                    writer = output
                try:
                    while block := reader.read(CHUNK):
                        copied += len(block)
                        digest.update(block)
                        writer.write(block)
                        progress(copied)
                finally:
                    if compressed:
                        writer.close()
                output.flush()
                os.fsync(output.fileno())
            require(signature(os.fstat(reader.fileno())) == signature(original),
                    "Quelldatei wurde beim Archivieren verändert")
        require(copied == original.st_size, "Unvollständige Quelldatei")
        expected = digest.hexdigest()
        require(digest_file(temporary, compressed,
                            Progress(source, "verifying_nas_copy", copied)) == (expected, copied),
                "NAS-Kopie stimmt nicht mit der Quelle überein")
        verified = temporary.stat()
        name = source.name[:100] + "." + expected + suffix
        final = destination / name
        if final.exists():
            require(digest_file(final, compressed) == (expected, copied),
                    "Vorhandene Archivdatei stimmt nicht überein")
            verified = final.stat()
            temporary.unlink()
        else:
            os.chmod(temporary, 0o640)
            # No overwrite: another process or changed shared directory must not
            # replace the verified copy between the existence check and publish.
            os.link(temporary, final)
            temporary.unlink()
            sync_directory(destination)
        receipt_path = final.with_name(final.name + ".json")
        receipt = {
            "schema": "netcore-log-archive-v1", "host": host,
            "source": str(source), "relative_source": str(relative),
            "source_mtime_ns": original.st_mtime_ns, "size_bytes": copied,
            "sha256": expected, "compression": "gzip" if compressed else "none",
            "archived_at": datetime.now(ZoneInfo(timezone)).isoformat(),
            "archive": str(final),
        }
        atomic_json(receipt_path, receipt)
        require(json.loads(receipt_path.read_text()) == receipt,
                "Archivbeleg nicht vollständig lesbar")
        network_mount(archive)
        # The same verified inode is published by a hard link, never overwritten.
        # Verify the published identity as well; avoid another complete read of
        # e.g. a 317 GB sparse/compressed syslog merely to check its new name.
        published = final.lstat()
        require(stat.S_ISREG(published.st_mode)
                and (published.st_dev, published.st_ino, published.st_size, published.st_mtime_ns)
                == (verified.st_dev, verified.st_ino, verified.st_size, verified.st_mtime_ns),
                "Veröffentlichte Archivdatei wurde verändert")
        require(signature(source.lstat()) == signature(original),
                "Quelle wurde vor der lokalen Freigabe verändert")
        if file_is_open(source):
            return {"source": str(source), "archive": str(final),
                    "status": "open", "released_bytes": 0}
        source.unlink()
        sync_directory(source.parent)
        return {"source": str(source), "archive": str(final),
                "sha256": expected, "status": "archived", "released_bytes": copied}
    finally:
        temporary.unlink(missing_ok=True)


def configuration(path):
    value = json.loads(path.read_text())
    require(value.get("schema") == "netcore-log-archive-config-v1", "Falsches Konfigurationsschema")
    require(value.get("source_root") == "/var/log", "Nur /var/log ist als Quelle erlaubt")
    require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]{0,79}", value.get("host", "")),
            "Ungültiger Hostname")
    require(value["host"] not in {".", ".."}, "Ungültiger Hostname")
    require(Path(value["archive_root"]).is_absolute(), "Archivpfad muss absolut sein")
    ZoneInfo(value["timezone"])
    return value


@contextmanager
def exclusive_lock(path):
    safe_mkdir(path.parent)
    fd = os.open(path, os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield
    finally:
        os.close(fd)


def run_command(arguments):
    completed = subprocess.run(arguments, text=True, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, timeout=120)
    require(completed.returncode == 0,
            "Befehl fehlgeschlagen: " + " ".join(arguments) + "\n" +
            completed.stderr[-2000:])
    return completed.stdout


def rotate_logs(force=False):
    rule = RSYSLOG_RULE
    if rule.is_file():
        arguments = ["logrotate"]
        if force:
            arguments.append("--force")
        # Same state file as the ordinary distribution logrotate run.
        run_command(arguments + [str(rule)])


def archive_pass(config, apply):
    archive = Path(config["archive_root"])
    mount = network_mount(archive)
    result = {"mount": mount, "host": config["host"], "files": [],
              "released_bytes": 0, "errors": [],
              "released_bytes_unit": "logical_uncompressed_source_bytes",
              "free_bytes_before": shutil.disk_usage(config["source_root"]).free}
    for source, relative, is_journal in candidates(Path(config["source_root"])):
        if not apply:
            result["files"].append({"source": str(source), "size_bytes": source.stat().st_size,
                                    "status": "open" if file_is_open(source) else "planned"})
            continue
        try:
            entry = archive_one(source, relative, archive, config["host"],
                                config["timezone"], is_journal=is_journal)
            result["files"].append(entry)
            result["released_bytes"] += entry["released_bytes"]
        except Exception as exc:
            result["errors"].append({"source": str(source), "error": str(exc)})
    result["free_bytes_after"] = shutil.disk_usage(config["source_root"]).free
    return result


def adapt_rsyslog_rule(text, global_text=""):
    """Keep the distro's reopen script; only this rule's retention becomes archival."""
    lines = text.splitlines()
    starts = [i for i, line in enumerate(lines) if line.strip() == "{"]
    ends = [i for i, line in enumerate(lines) if line.strip() == "}"]
    require(len(starts) == len(ends) == 1 and starts[0] < ends[0],
            "Unbekanntes rsyslog-logrotate-Layout; keine Änderung")
    require(any(line.strip() == "postrotate" for line in lines),
            "rsyslog-Rotation benötigt ein vorhandenes Reopen-Skript")
    require(not any(line.strip() in {"copytruncate", "copy"} for line in lines),
            "copy/copytruncate werden für verlustfreie Logarchivierung nicht unterstützt")
    # Ubuntu's /etc/logrotate.conf supplies su/create before including rsyslog.
    # This timer deliberately runs just the rsyslog rule, so make those inherited
    # defaults explicit. In particular, /var/log may be group-writable by syslog.
    defaults = {}
    for line in global_text.splitlines():
        value = line.split("#", 1)[0].strip()
        if value.startswith("include ") or value == "{":
            break
        if re.fullmatch(r"su\s+[A-Za-z0-9_-]+\s+[A-Za-z0-9_-]+", value):
            defaults["su"] = value
        if re.fullmatch(r"create(?:\s+[0-7]{3,4}(?:\s+[A-Za-z0-9_-]+(?:\s+[A-Za-z0-9_-]+)?)?)?", value):
            defaults["create"] = value
    body = lines[starts[0] + 1:ends[0]]
    inherited = []
    if not any(re.match(r"\s*su\s+", line) for line in body):
        inherited.append("    " + defaults.get("su", "su root root"))
    if not any(re.match(r"\s*(?:create|nocreate)(?:\s|$)", line) for line in body):
        inherited.append("    " + defaults.get("create", "create"))
    result, script = [], False
    for index, line in enumerate(lines):
        directive = line.strip().split(maxsplit=1)[0] if line.strip() else ""
        if directive in {"prerotate", "postrotate", "firstaction", "lastaction"}:
            script = True
        if not script and starts[0] < index < ends[0] and directive in {
            "rotate", "compress", "delaycompress", "nocompress", "maxsize", "size", "maxage",
            "dateext", "nodateext", "dateformat"
        }:
            continue
        result.append(line)
        if index == starts[0]:
            result.extend(["    rotate -1", "    maxsize 64M", "    nocompress",
                           "    dateext", "    dateformat -%Y%m%d-%H%M%S"])
            result.extend(inherited)
        if directive == "endscript":
            script = False
    return "\n".join(result) + "\n"


SERVICE = """[Unit]
Description=NetCore closed logs to NAS archive
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/python3 -B /opt/netcore-log-archive/archive-netcore-logs.py --apply
TimeoutStartSec=6h
UMask=0027
Nice=10
IOSchedulingClass=idle
"""
TIMER = """[Unit]
Description=Archive NetCore logs on NAS every minute

[Timer]
OnCalendar=*-*-* *:*:00
RandomizedDelaySec=10s
Persistent=true
Unit=netcore-log-archive.service

[Install]
WantedBy=timers.target
"""
ROTATION_SERVICE = """[Unit]
Description=Rotate large NetCore syslog files independently of NAS transfer
ConditionPathExists=/etc/logrotate.d/rsyslog

[Service]
Type=oneshot
ExecStart=/usr/sbin/logrotate /etc/logrotate.d/rsyslog
TimeoutStartSec=2min
"""
ROTATION_TIMER = TIMER.replace("Archive NetCore logs on NAS every minute",
                               "Rotate large NetCore syslog files every minute").replace(
                                   "netcore-log-archive.service", "netcore-log-rotate.service")


def install(args):
    require(os.geteuid() == 0, "Installation benötigt root")
    require(args.archive_root is not None, "--archive-root ist für Installation erforderlich")
    host = socket.gethostname()
    config = {"schema": "netcore-log-archive-config-v1", "source_root": "/var/log",
              "archive_root": str(Path(args.archive_root).absolute()),
              "host": host, "timezone": "Europe/Berlin"}
    mount = network_mount(Path(config["archive_root"]))
    # Parse before changing any file, including an existing installation.
    require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]{0,79}", host), "Ungültiger Hostname")
    rule_path = RSYSLOG_RULE
    old_rule = rule_path.read_text() if rule_path.is_file() else None
    global_text = GLOBAL_ROTATION.read_text() if GLOBAL_ROTATION.is_file() else ""
    new_rule = adapt_rsyslog_rule(old_rule, global_text) if old_rule is not None else None
    if not args.apply:
        return {"phase": "planned", "config": config, "mount": mount,
                "rsyslog_rotation": new_rule is not None,
                "archive": archive_pass(config, False)}
    safe_mkdir(STATE)
    with exclusive_lock(STATE / "archive.lock"):
        if new_rule is not None:
            backup = STATE / ("rsyslog-logrotate-" + uuid.uuid4().hex + ".before")
            shutil.copy2(rule_path, backup)
            rule_path.write_text(new_rule)
            try:
                run_command(["logrotate", "--debug", str(rule_path)])
            except Exception:
                shutil.copy2(backup, rule_path)
                raise
        safe_mkdir(PROGRAM.parent)
        temporary = PROGRAM.with_suffix(".py.new")
        shutil.copyfile(Path(__file__).resolve(), temporary)
        temporary.chmod(0o755)
        temporary.replace(PROGRAM)
        atomic_json(DEFAULT_CONFIG, config)
        unit_root = SYSTEMD_UNITS
        (unit_root / "netcore-log-archive.service").write_text(SERVICE)
        (unit_root / "netcore-log-archive.timer").write_text(TIMER)
        (unit_root / "netcore-log-rotate.service").write_text(ROTATION_SERVICE)
        (unit_root / "netcore-log-rotate.timer").write_text(ROTATION_TIMER)
        run_command(["systemctl", "daemon-reload"])
        rotate_logs(force=True)
        run_command(["systemctl", "enable", "--now", "netcore-log-rotate.timer",
                     "netcore-log-archive.timer"])
        report = {"phase": "installed", "config": config, "mount": mount,
                  "archive_status": "scheduled", "released_bytes": 0,
                  "status_command": "journalctl -fu netcore-log-archive.service -o cat"}
        atomic_json(STATE / "last-run.json", report)
    run_command(["systemctl", "start", "--no-block", "netcore-log-archive.service"])
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", type=Path, default=DEFAULT_CONFIG)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--install", action="store_true")
    parser.add_argument("--archive-root", type=Path)
    parser.add_argument("--rotate", action="store_true")
    args = parser.parse_args()
    try:
        if args.install:
            require(args.config == DEFAULT_CONFIG, "Installation verwendet die Standardkonfiguration")
            result = install(args)
        else:
            config = configuration(args.config)
            if args.apply:
                require(os.geteuid() == 0, "Lokale Freigabe benötigt root")
                with exclusive_lock(STATE / "archive.lock"):
                    # Drain old rotations before logrotate may rename them.
                    result = archive_pass(config, True)
                    if args.rotate and not result["errors"]:
                        rotate_logs()
                        second = archive_pass(config, True)
                        result["files"].extend(second["files"])
                        result["errors"].extend(second["errors"])
                        result["released_bytes"] += second["released_bytes"]
                    result["phase"] = "failed" if result["errors"] else "completed"
                    atomic_json(STATE / "last-run.json", result)
            else:
                result = archive_pass(config, False)
                result["phase"] = "planned"
        print(json.dumps(result, ensure_ascii=False, indent=2))
        return 1 if result.get("errors") else 0
    except Exception as exc:
        print(json.dumps({"phase": "failed", "error": str(exc)}, ensure_ascii=False),
              file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
