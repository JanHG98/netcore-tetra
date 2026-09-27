#!/usr/bin/env python3
"""No downloads or compilation on the Pi; identity and SD expansion only."""
from pathlib import Path
import subprocess


def main():
    subprocess.run(['ssh-keygen', '-A'], check=True)
    # The stock Pi init_resize may already have expanded the partition.
    # growpart's NOCHANGE result is harmless; resize2fs also handles that case.
    try:
        source = subprocess.check_output(['findmnt', '-n', '-o', 'SOURCE', '/'], text=True).strip()
        device = Path(source).resolve()
        part = Path('/sys/class/block') / device.name
        number = (part / 'partition').read_text().strip()
        parent = part.resolve().parent.name
        if parent and number == '2' and device.is_block_device():
            result = subprocess.run(['growpart', '/dev/' + parent, number], capture_output=True, text=True)
            if result.returncode not in (0, 1) or (result.returncode == 1 and 'NOCHANGE:' not in result.stdout):
                raise RuntimeError(result.stderr or result.stdout)
            subprocess.run(['resize2fs', str(device)], check=True)
        else:
            raise RuntimeError('Rootpartition konnte nicht eindeutig bestimmt werden')
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as exc:
        # Keep SSH, discovery and the preinstalled station usable for recovery.
        print('SD-Erweiterung prüfen; das vorhandene Dateisystem bleibt nutzbar: ' + str(exc), flush=True)
    Path('/var/lib/netcore-firstboot.done').write_text('Identity initialized; no network build required.\n')


if __name__ == '__main__':
    main()
