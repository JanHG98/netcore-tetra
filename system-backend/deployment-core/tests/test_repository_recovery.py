"""Exercise abandoned and live Git locks in a real managed checkout."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch
import json

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from deploy import Repository


class RecoveryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.origin = self.root / 'origin'
        self.state = self.root / 'state'
        subprocess.run(['git', 'init', '-b', 'main', str(self.origin)], check=True, capture_output=True)
        (self.origin / 'source.txt').write_text('committed source\n')
        subprocess.run(['git', '-C', str(self.origin), 'add', '.'], check=True, capture_output=True)
        subprocess.run(['git', '-C', str(self.origin), '-c', 'user.name=Test',
                        '-c', 'user.email=test@example.invalid', 'commit', '-m', 'source'],
                       check=True, capture_output=True)
        self.cfg = dict(repository=str(self.origin), state_dir=str(self.state))
        self.repo = Repository(self.cfg)
        self.log = []
        self.sha = self.repo.resolve('main', self.log.append)
        self.lock = self.repo.path / '.git/index.lock'

    def tearDown(self):
        self.temp.cleanup()

    def stale_lock(self):
        self.lock.write_bytes(b'partial index from interrupted checkout')
        stamp = time.time() - 600
        os.utime(self.lock, (stamp, stamp))

    def test_locked_tree_is_preserved_while_fresh_checkout_is_used(self):
        self.stale_lock()
        (self.repo.path / 'source.txt').write_text('partial checkout\n')
        (self.repo.path / 'operator-note.txt').write_text('keep this')
        marker = self.state / 'deployed-subscriber-core.json'
        marker.write_text('{"commit":"previous"}')
        previous = self.repo.path
        source = self.repo.checkout(self.sha, self.log.append)
        self.assertEqual((source / 'source.txt').read_text(), 'committed source\n')
        archived = [previous]
        self.assertNotEqual(previous, source)
        self.assertEqual((archived[0] / '.git/index.lock').read_bytes(), b'partial index from interrupted checkout')
        self.assertEqual((archived[0] / 'operator-note.txt').read_text(), 'keep this')
        self.assertEqual((archived[0] / 'source.txt').read_text(), 'partial checkout\n')
        self.assertEqual(marker.read_text(), '{"commit":"previous"}')
        self.assertFalse((source / '.git/index.lock').exists())

    def test_live_git_keeps_its_original_tree_and_lock(self):
        self.stale_lock()
        previous = self.repo.path
        child = subprocess.Popen(['git', '-c',
            'alias.hold=!sleep 2; printf untouched > writer-finished', 'hold'], cwd=previous)
        try:
            source = self.repo.checkout(self.sha, self.log.append)
            self.assertNotEqual(source, previous)
            child.wait(timeout=10)
            self.assertEqual(child.returncode, 0)
            self.assertEqual((previous / 'writer-finished').read_text(), 'untouched')
            self.assertTrue(self.lock.exists())
        finally:
            child.wait(timeout=10)

    def test_fresh_lock_is_preserved_as_well(self):
        self.lock.write_bytes(b'new index')
        previous = self.repo.path
        with self.lock.open('rb') as active_descriptor:
            source = self.repo.checkout(self.sha, self.log.append)
            self.assertNotEqual(source, previous)
            self.assertEqual(active_descriptor.read(), b'new index')
        self.assertEqual(self.lock.read_bytes(), b'new index')

    def test_failed_clone_keeps_previous_selection_and_files(self):
        self.stale_lock()
        previous = self.repo.path
        with patch('deploy.run', side_effect=RuntimeError('network unavailable')):
            with self.assertRaisesRegex(RuntimeError, 'network unavailable'):
                self.repo.checkout(self.sha, self.log.append)
        self.assertEqual(self.repo.path, previous)
        self.assertTrue(self.lock.exists())
        self.assertFalse((self.state / 'source-location.json').exists())
        self.assertEqual(list(self.state.glob('source-new-*')), [])

    def test_new_selection_survives_agent_restart(self):
        self.stale_lock()
        source = self.repo.checkout(self.sha, self.log.append)
        other = Repository(self.cfg)
        self.assertEqual(other.checkout(self.sha, self.log.append), source)
        self.assertTrue(self.lock.exists())

    def test_selector_cannot_escape_the_managed_state(self):
        (self.state / 'source-location.json').write_text(json.dumps({'directory': '../elsewhere'}))
        with self.assertRaisesRegex(RuntimeError, 'Ungültiger verwalteter Git-Cache'):
            self.repo.checkout(self.sha, self.log.append)

    def test_process_mutex_excludes_another_repository_instance(self):
        other = Repository(self.cfg)
        with self.repo.guard():
            with self.assertRaisesRegex(RuntimeError, 'anderer NetCore-Prozess'):
                other.checkout(self.sha, self.log.append)

    def test_unrelated_local_edits_are_still_rejected(self):
        self.repo.checkout(self.sha, self.log.append)
        (self.repo.path / 'source.txt').write_text('local edit\n')
        with self.assertRaisesRegex(RuntimeError, 'local changes'):
            self.repo.checkout(self.sha, self.log.append)
        self.assertEqual((self.repo.path / 'source.txt').read_text(), 'local edit\n')
        self.assertFalse((self.state / 'source-location.json').exists())


if __name__ == '__main__':
    unittest.main()
