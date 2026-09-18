"""Bounded side effects and strict Asterisk status parsing for the SIP router."""
from __future__ import annotations

import copy
import queue
import re
import threading


def registration_status(output: str, registration_id: str) -> str:
    for line in output.splitlines():
        fields = line.strip().split()
        if not fields or fields[0].split('/', 1)[0].rstrip(':') != registration_id:
            continue
        for field in fields[1:]:
            status = field.lower().strip('(),')
            if status in {'registered', 'unregistered', 'rejected', 'stopped'}:
                return status
    return 'absent'


def available_contact(output: str) -> bool:
    # Ignore the column heading and unreachable/unknown contacts. A registration
    # existing in the AoR alone does not establish SIP reachability.
    return any(re.match(r'^\s*Contact:\s+[^<\s]+', line)
               and re.search(r'\bAvail\b', line)
               for line in output.splitlines())


class SideEffects:
    """One bounded FIFO; persistence coalesces separately from optional events.

    Neither a broken broker nor slow storage holds the routing/state mutex.
    Queue overflow is observable and never backpressures call control.
    """
    def __init__(self, app, capacity=512):
        self.app = app
        self.jobs = queue.Queue(maxsize=max(1, capacity))
        self.dirty = threading.Event()
        self.stopping = threading.Event()
        self.dropped = 0
        self.failures = 0
        self.last_error = None
        self.worker = threading.Thread(target=self.run, name='sip-side-effects', daemon=True)
        self.worker.start()

    def submit(self, path, record, mqtt=False):
        try:
            self.jobs.put_nowait((path, copy.deepcopy(record), mqtt))
        except queue.Full:
            self.dropped += 1

    def run(self):
        import json
        while not self.stopping.is_set() or not self.jobs.empty() or self.dirty.is_set():
            if self.dirty.is_set():
                self.dirty.clear()
                try:
                    self.app.persist_snapshot()
                except Exception as error:
                    self.failures += 1
                    self.last_error = str(error)
            try:
                path, record, mqtt = self.jobs.get(timeout=.1)
            except queue.Empty:
                continue
            try:
                path.parent.mkdir(parents=True, exist_ok=True)
                with path.open('a', encoding='utf-8') as handle:
                    handle.write(json.dumps(record, ensure_ascii=False, separators=(',', ':')) + '\n')
                if mqtt:
                    prefix = str(self.app.config.mqtt.get('topic_prefix', 'netcore/v1')).rstrip('/')
                    topic = f"{prefix}/events/{record['event_type'].replace('.', '/')}"
                    if not self.app.publish_mqtt(topic, record):
                        raise RuntimeError('MQTT publish failed')
            except Exception as error:
                self.failures += 1
                self.last_error = str(error)
            finally:
                self.jobs.task_done()

    def close(self, timeout=3):
        self.stopping.set()
        self.worker.join(timeout)

    def status(self):
        return {'queued': self.jobs.qsize(), 'dropped': self.dropped,
                'failures': self.failures, 'last_error': self.last_error}
