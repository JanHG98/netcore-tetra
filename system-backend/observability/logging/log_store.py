#!/usr/bin/env python3
"""Bounded rsyslog omprog sink, NMS preview and verified daily share archive.

Only the receiver touches active segments. Archive I/O never holds its lock.
Raw segments are the archive source; the SQLite outbox is a bounded UI preview.
"""
from __future__ import annotations

import argparse
import contextlib
import datetime as dt
import fcntl
import gzip
import hashlib
import ipaddress
import json
import os
from pathlib import Path
import re
import shutil
import sqlite3
import sys
import tempfile
import time
import urllib.request
import uuid

UTC = dt.timezone.utc
SEGMENT = re.compile(r"\d{8}T\d{6}-[0-9a-f]{32}\.jsonl\Z")


def atomic_json(path, value):
    path = Path(path)
    fd, name = tempfile.mkstemp(prefix="." + path.name, dir=path.parent)
    try:
        with os.fdopen(fd, "w") as out:
            json.dump(value, out, ensure_ascii=False)
            out.flush()
            os.fsync(out.fileno())
        os.replace(name, path)
        fsync_dir(path.parent)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def fsync_dir(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def load_config(path):
    cfg = json.loads(Path(path).read_text())
    for key in ("state_dir", "archive_mount"):
        if not Path(cfg[key]).is_absolute():
            raise ValueError(f"{key} must be absolute")
    for key in ("archive_directory", "collector_id"):
        if not re.fullmatch(r"[A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*", cfg[key]) or any(
                x in (".", "..") for x in cfg[key].split("/")):
            raise ValueError(f"Invalid {key}")
    for key in ("archive_days", "archive_max_bytes", "archive_min_free_bytes", "local_max_bytes",
                "local_min_free_bytes", "segment_bytes", "max_record_bytes", "preview_records"):
        if not isinstance(cfg[key], int) or cfg[key] <= 0:
            raise ValueError(f"{key} must be positive")
    if not 4096 <= cfg["max_record_bytes"] <= 65536 or cfg["preview_records"] > 5000 or cfg["max_record_bytes"] * cfg["preview_records"] > 96 * 1024 * 1024:
        raise ValueError("Record/preview limits exceed the supported disk budget")
    if cfg["segment_bytes"] > cfg["local_max_bytes"]:
        raise ValueError("Segment is larger than the local budget")
    for network in cfg["allowed_networks"]:
        ipaddress.ip_network(network)
    if cfg["nms_url"] != "http://127.0.0.1:8210":
        raise ValueError("NMS preview must use the local Observability endpoint")
    return cfg


class Store:
    def __init__(self, cfg):
        self.cfg = cfg
        self.root = Path(cfg["state_dir"])
        self.raw = self.root / "raw"
        self.raw.mkdir(parents=True, exist_ok=True)
        self.networks = [ipaddress.ip_network(n) for n in cfg["allowed_networks"]]
        try:
            self.inventory = {h["address"]: h["name"] for h in
                              json.loads(Path(cfg["inventory"]).read_text())["hosts"]}
        except (KeyError, OSError, ValueError):
            self.inventory = {}
        self.db = sqlite3.connect(self.root / "preview.sqlite", timeout=10)
        self.db.execute("PRAGMA journal_mode=WAL")
        self.db.execute("PRAGMA synchronous=FULL")
        self.db.execute("PRAGMA max_page_count=65536")  # 256 MiB maximum, incl. free pages
        self.db.execute("PRAGMA wal_autocheckpoint=64")
        self.db.executescript("""
            CREATE TABLE IF NOT EXISTS outbox (id INTEGER PRIMARY KEY AUTOINCREMENT, payload TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS counters (name TEXT PRIMARY KEY, value INTEGER NOT NULL);
            CREATE TABLE IF NOT EXISTS sources (ip TEXT PRIMARY KEY, name TEXT, last_seen TEXT);
        """)

    @contextlib.contextmanager
    def locked(self):
        with (self.root / "store.lock").open("a") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            yield

    def count(self, name, amount=1):
        self.db.execute("INSERT INTO counters VALUES (?, ?) ON CONFLICT(name) DO UPDATE SET value=value+excluded.value",
                        (name, amount))

    def seal(self):
        """Caller holds store.lock; only complete JSONL records become immutable."""
        for path in sorted(self.raw.glob("*.open")):
            with path.open("r+b") as source:
                source.seek(0, 2)
                size = source.tell()
                if size:
                    source.seek(max(0, size - self.cfg["max_record_bytes"] - 1))
                    tail = source.read()
                    end = tail.rfind(b"\n")
                    valid = size - len(tail) + end + 1 if end >= 0 else 0
                    source.truncate(valid)
                    if valid < size:
                        self.count("partial_bytes_recovered", size - valid)
                    source.flush()
                    os.fsync(source.fileno())
                else:
                    valid = 0
            if valid:
                path.rename(path.with_suffix(".jsonl"))
            else:
                path.unlink()
        fsync_dir(self.raw)

    def recover(self):
        with self.locked(), self.db:
            self.seal()

    def make_room(self, incoming):
        files = sorted(self.raw.glob("*.jsonl"))
        total = sum(p.stat().st_size for p in self.raw.iterdir() if p.suffix in (".open", ".jsonl"))
        free = shutil.disk_usage(self.root).free
        for old in files:
            if total + incoming <= self.cfg["local_max_bytes"] and free - incoming >= self.cfg["local_min_free_bytes"]:
                break
            size = old.stat().st_size
            old.unlink()
            self.count("unarchived_segments_dropped")
            self.count("unarchived_bytes_dropped", size)
            total -= size
            free = shutil.disk_usage(self.root).free
        return total + incoming <= self.cfg["local_max_bytes"] and free - incoming >= self.cfg["local_min_free_bytes"]

    def append(self, record):
        try:
            ip = str(ipaddress.ip_address(record["source_ip"]))
            if not any(ipaddress.ip_address(ip) in n for n in self.networks):
                raise ValueError("Source is outside allowed networks")
        except (KeyError, ValueError):
            with self.db:
                self.count("rejected_sources")
            return
        now = dt.datetime.now(UTC)
        truncated = len(str(record.get("message", ""))) > 8192
        record = {k: str(record.get(k, ""))[:256] for k in
                  ("timestamp", "hostname", "program", "severity", "facility", "source_ip", "transport")} | {
            "message": str(record.get("message", ""))[:8192],
            "received_at": now.isoformat(), "id": uuid.uuid4().hex,
            "inventory_name": self.inventory.get(ip, ip),
        }
        if truncated:
            record["truncated"] = True
        # Bound the encoded size too: JSON escapes and UTF-8 can expand characters.
        payload = json.dumps(record, ensure_ascii=False).encode() + b"\n"
        while len(payload) > self.cfg["max_record_bytes"]:
            record["message"] = record["message"][:len(record["message"]) // 2]
            record["truncated"] = True
            if not record["message"]:
                raise ValueError("Configured record limit is too small for metadata")
            payload = json.dumps(record, ensure_ascii=False).encode() + b"\n"
        with self.locked(), self.db:
            current = list(self.raw.glob("*.open"))
            path = current[0] if current else None
            if path and (not path.name.startswith(now.strftime("%Y%m%d")) or
                         path.stat().st_size + len(payload) > self.cfg["segment_bytes"]):
                self.seal()
                path = None
            if not self.make_room(len(payload)):
                self.seal()
                path = None
                if not self.make_room(len(payload)):
                    self.count("incoming_records_dropped")
                    return  # Explicit bounded-loss policy; never fill the root disk.
            if path is None:
                path = self.raw / (now.strftime("%Y%m%dT%H%M%S-") + uuid.uuid4().hex + ".open")
            new = not path.exists()
            with path.open("ab") as out:
                out.write(payload)
                out.flush()
                os.fsync(out.fileno())
            if new:
                fsync_dir(self.raw)
            self.count("received_records")
            self.db.execute("INSERT INTO sources VALUES (?, ?, ?) ON CONFLICT(ip) DO UPDATE SET name=excluded.name,last_seen=excluded.last_seen",
                            (ip, record["inventory_name"], record["received_at"]))
            # Preview failure/expiry does not remove the original archive record.
            self.db.execute("INSERT INTO outbox(payload) VALUES (?)", (payload.decode(),))
            size = self.db.execute("SELECT count(*) FROM outbox").fetchone()[0]
            if size > self.cfg["preview_records"]:
                self.db.execute("DELETE FROM outbox WHERE id IN (SELECT id FROM outbox ORDER BY id LIMIT ?)",
                                (size - self.cfg["preview_records"],))
                self.count("preview_records_expired", size - self.cfg["preview_records"])

    def status(self):
        files = list(self.raw.glob("*.jsonl")) + list(self.raw.glob("*.open"))
        total = 0
        for path in files:
            try:
                total += path.stat().st_size
            except FileNotFoundError:
                pass
        return {"updated_at": dt.datetime.now(UTC).isoformat(),
                "local_bytes": total, "local_max_bytes": self.cfg["local_max_bytes"],
                "free_bytes": shutil.disk_usage(self.root).free,
                "preview_pending": self.db.execute("SELECT count(*) FROM outbox").fetchone()[0],
                "counters": dict(self.db.execute("SELECT name,value FROM counters")),
                "sources": [{"ip": ip, "name": name, "last_seen": seen} for ip, name, seen in
                            self.db.execute("SELECT ip,name,last_seen FROM sources ORDER BY name")],
                "expected_hosts": self.inventory, "archive_mount": self.cfg["archive_mount"],
                "archive_directory": self.cfg["archive_directory"]}


def ingest(store, source=sys.stdin, output=sys.stdout):
    store.recover()
    print("OK", file=output, flush=True)  # omprog startup handshake
    for line in source:
        try:
            store.append(json.loads(line))
            print("OK", file=output, flush=True)  # after durable raw + preview commit
        except (ValueError, KeyError):
            with store.db:
                store.count("malformed_records")
            print("OK", file=output, flush=True)
        except Exception as exc:
            print(f"NetCore log sink: {exc}", file=sys.stderr, flush=True)
            print("ERR", file=output, flush=True)  # rsyslog retries, no successful ACK


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *args, **kwargs):
        raise ValueError("Redirect refused")


def forward_once(store):
    rows = store.db.execute("SELECT id,payload FROM outbox ORDER BY id LIMIT 100").fetchall()
    if not rows:
        return 0
    logs = []
    for _, payload in rows:
        r = json.loads(payload)
        level = "error" if r["severity"] in ("emerg", "alert", "crit", "err", "error") else (
            "warn" if r["severity"] in ("warning", "warn") else "debug" if r["severity"] == "debug" else "info")
        logs.append({"timestamp": r["received_at"], "service": r["program"] or "syslog",
                     "node": r["inventory_name"], "level": level, "message": r["message"][:4096] if r["message"].strip() else "[empty syslog message]",
                     "fields": {"syslog_id": r["id"], "source_ip": r["source_ip"],
                                "hostname": r["hostname"], "syslog_timestamp": r["timestamp"],
                                "facility": r["facility"], "transport": r["transport"]}})
    request = urllib.request.Request(store.cfg["nms_url"] + "/api/v1/logs/ingest",
                                     json.dumps({"records": logs}, ensure_ascii=False).encode(), {"Content-Type": "application/json"})
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
    with opener.open(request, timeout=5) as response:
        if response.status != 202 or json.loads(response.read(4096)).get("accepted") != len(logs):
            raise ValueError("NMS did not accept the complete preview batch")
    with store.db:
        store.db.executemany("DELETE FROM outbox WHERE id=?", [(row[0],) for row in rows])
    return len(rows)


def bridge(store):
    while True:
        error = None
        try:
            forward_once(store)
        except Exception as exc:
            error = str(exc)
        status = store.status()
        status["preview_error"] = error
        atomic_json(store.root / "receiver-status.json", status)
        time.sleep(5)


def open_share(cfg):
    """Require an actual network mount; retain its fd across an unmount/remount.

    Bind mounts of NFS/CIFS into an LXC retain their filesystem type. Never mkdir
    the mountpoint: an absent mount must not turn into a local archive directory.
    """
    mount = Path(cfg["archive_mount"])
    expected = str(mount)
    valid = []
    for line in Path("/proc/self/mountinfo").read_text().splitlines():
        left, right = line.split(" - ", 1)
        parts = left.split()
        unescape = lambda x: re.sub(r"\\([0-7]{3})", lambda m: chr(int(m[1], 8)), x)
        if unescape(parts[4]) == expected and right.split()[0] in ("nfs", "nfs4", "cifs"):
            valid.append(parts[2])
    if not valid:
        raise OSError(f"Network share is not mounted at {mount}")
    fd = os.open(mount, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    st = os.fstat(fd)
    if f"{os.major(st.st_dev)}:{os.minor(st.st_dev)}" not in valid:
        os.close(fd)
        raise OSError("Share changed during mount check")
    return fd


def subdir(parent, name):
    try:
        os.mkdir(name, 0o750, dir_fd=parent)
    except FileExistsError:
        pass
    return os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=parent)


def digest(source):
    result = hashlib.sha256()
    for data in iter(lambda: source.read(1024 * 1024), b""):
        result.update(data)
    return result.digest()


def archive_file(source, directory, name):
    """Delete decisions are made by caller, only after decompression/readback."""
    target = name + ".gz"
    source.seek(0)
    expected = digest(source)
    try:
        fd = os.open(target, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=directory)
    except FileNotFoundError:
        fd = None
    if fd is not None:
        with os.fdopen(fd, "rb") as existing, gzip.GzipFile(fileobj=existing) as gz:
            if digest(gz) != expected:
                raise OSError("Existing archive checksum mismatch")
        return
    partial = name + ".partial"
    fd = os.open(partial, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_NOFOLLOW, 0o640, dir_fd=directory)
    try:
        with os.fdopen(fd, "wb") as out:
            source.seek(0)
            with gzip.GzipFile(filename="", mode="wb", fileobj=out, mtime=0) as gz:
                shutil.copyfileobj(source, gz, 1024 * 1024)
            out.flush()
            os.fsync(out.fileno())
        verify = os.open(partial, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=directory)
        with os.fdopen(verify, "rb") as check, gzip.GzipFile(fileobj=check) as gz:
            if digest(gz) != expected:
                raise OSError("Archive readback checksum mismatch")
        os.replace(partial, target, src_dir_fd=directory, dst_dir_fd=directory)
        os.fsync(directory)
    except BaseException:
        try:
            os.unlink(partial, dir_fd=directory)
        except OSError:
            pass
        raise


def prune_archive(root_fd, cfg, incoming=0):
    """Retention is scoped to this collector's dated NetCore segment files."""
    entries = []
    today = dt.datetime.now(UTC).date()
    for name in os.listdir(root_fd):
        if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", name):
            continue
        date = dt.date.fromisoformat(name)
        folder = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=root_fd)
        try:
            for file in os.listdir(folder):
                if file.endswith(".partial") and SEGMENT.fullmatch(file[:-8]):
                    os.unlink(file, dir_fd=folder)  # only while holding archive.lock
                    continue
                if not file.endswith(".gz") or not SEGMENT.fullmatch(file[:-3]):
                    continue
                st = os.stat(file, dir_fd=folder, follow_symlinks=False)
                if (today - date).days >= cfg["archive_days"]:
                    os.unlink(file, dir_fd=folder)
                else:
                    entries.append((name, file, st.st_size))
        finally:
            os.close(folder)
    total = sum(x[2] for x in entries)
    for date, file, size in sorted(entries):
        free = os.fstatvfs(root_fd).f_bavail * os.fstatvfs(root_fd).f_frsize
        if total + incoming <= cfg["archive_max_bytes"] and free - incoming >= cfg["archive_min_free_bytes"]:
            break
        folder = os.open(date, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=root_fd)
        try:
            os.unlink(file, dir_fd=folder)
        finally:
            os.close(folder)
        total -= size
    fs = os.fstatvfs(root_fd)
    if total + incoming > cfg["archive_max_bytes"] or fs.f_bavail * fs.f_frsize - incoming < cfg["archive_min_free_bytes"]:
        raise OSError("Archive share has insufficient space within configured limits")


def archive(store, mount_opener=open_share):
    # Also serialize manual runs; an NFS stall must never hold store.lock.
    with (store.root / "archive.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return _archive(store, mount_opener)


def _archive(store, mount_opener):
    status = {"last_attempt": dt.datetime.now(UTC).isoformat(), "archived_segments": 0, "error": None}
    status_path = store.root / "archive-status.json"
    try:
        previous = json.loads(status_path.read_text())
        status["last_success"] = previous.get("last_success")
    except (OSError, ValueError):
        pass
    try:
        with store.locked(), store.db:
            store.seal()
        with contextlib.ExitStack() as stack:
            fd = mount_opener(store.cfg)
            stack.callback(os.close, fd)
            for component in (store.cfg["archive_directory"] + "/" + store.cfg["collector_id"]).split("/"):
                fd = subdir(fd, component)
                stack.callback(os.close, fd)
            prune_archive(fd, store.cfg)
            for path in sorted(store.raw.glob("*.jsonl")):
                if not SEGMENT.fullmatch(path.name):
                    continue
                try:
                    source = path.open("rb")
                except FileNotFoundError:  # receiver may have evicted it at the local limit
                    continue
                with source:
                    prune_archive(fd, store.cfg, os.fstat(source.fileno()).st_size + 1024 * 1024)
                    day = dt.datetime.strptime(path.name[:8], "%Y%m%d").date().isoformat()
                    folder = subdir(fd, day)
                    try:
                        archive_file(source, folder, path.name)
                    finally:
                        os.close(folder)
                with store.locked():
                    path.unlink(missing_ok=True)
                    fsync_dir(store.raw)
                status["archived_segments"] += 1
        status["last_success"] = dt.datetime.now(UTC).isoformat()
    except Exception as exc:
        status["error"] = str(exc)
    atomic_json(status_path, status)
    if status["error"]:
        raise OSError(status["error"])
    return status


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("ingest", "bridge", "archive", "check"))
    parser.add_argument("--config", default="/etc/netcore/syslog.json")
    args = parser.parse_args()
    cfg = load_config(args.config)
    if args.command == "check":
        print("Configuration valid; share availability is checked by the archive job.")
        return
    store = Store(cfg)
    {"ingest": ingest, "bridge": bridge, "archive": archive}[args.command](store)


if __name__ == "__main__":
    main()
