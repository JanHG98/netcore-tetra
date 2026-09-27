#!/usr/bin/env python3
"""Render a runtime config on service start. No dependency on a running agent."""
import argparse
from copy import deepcopy
import os
from pathlib import Path
import sys
import tomllib
from bindings import resolve_config
from common import atomic_write, json_file, toml_dump


def merge_runtime_edits(current, previous_source, previous_runtime):
    """Keep WebUI edits across restart; explicit changes to the source take priority."""
    result = deepcopy(current)
    absent = object()
    for key in previous_source.keys() | previous_runtime.keys():
        old = previous_source.get(key, absent)
        edited = previous_runtime.get(key, absent)
        now = current.get(key, absent)
        if all(isinstance(x, dict) for x in (old, edited, now)):
            result[key] = merge_runtime_edits(now, old, edited)
        elif old != edited and now == old:
            if edited is absent:
                result.pop(key, None)
            else:
                result[key] = deepcopy(edited)
    return result


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--service', required=True)
    p.add_argument('--config', required=True)
    p.add_argument('--cache', default='/var/lib/netcore-discovery/endpoints.json')
    p.add_argument('--runtime', required=True)
    p.add_argument('command', nargs=argparse.REMAINDER)
    a = p.parse_args()
    command = a.command[1:] if a.command[:1] == ['--'] else a.command
    if not command:
        p.error('missing executable')
    try:
        original = Path(a.config).read_text()
        data = tomllib.loads(original)
        snapshot = Path(a.runtime).with_name('source.toml')
        if snapshot.exists() and Path(a.runtime).exists():
            try:
                data = merge_runtime_edits(data, tomllib.loads(snapshot.read_text()),
                                          tomllib.loads(Path(a.runtime).read_text()))
            except (OSError, ValueError) as exc:
                print(f'Discovery: ignoring invalid previous runtime config: {exc}', file=sys.stderr)
        data, changes = resolve_config(a.service, data, json_file(a.cache, {}))
        atomic_write(a.runtime, toml_dump(data))
        atomic_write(snapshot, original)
        # Keep the existing TBS known-good fallback; never regenerate/overwrite it.
        fallback = Path(a.config + '.fallback')
        if fallback.is_file() and not Path(a.runtime + '.fallback').exists():
            atomic_write(a.runtime + '.fallback', fallback.read_text())
        command = [a.runtime if x == a.config else x for x in command]
        print(f'Discovery: {len(changes)} dependency fields resolved', file=sys.stderr)
    except (OSError, ValueError) as exc:
        print(f'Discovery: original configuration retained: {exc}', file=sys.stderr)
    os.execv(command[0], command)


if __name__ == '__main__':
    main()
