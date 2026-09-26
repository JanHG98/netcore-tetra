"""Persistent incidents, provider aliases and the per-radio delivery ledger."""
import json
import sqlite3
import threading
from contextlib import contextmanager
from datetime import datetime, timezone
from pathlib import Path


def timestamp(value=None):
    if value is None:
        return datetime.now(timezone.utc).timestamp()
    if isinstance(value, (int, float)):
        return float(value)
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("Zeitangaben benötigen eine Zeitzone")
    return parsed.timestamp()


def iso(value=None):
    return datetime.fromtimestamp(timestamp(value), timezone.utc).isoformat()


class Store:
    def __init__(self, path):
        if str(path) != ":memory:":
            Path(path).parent.mkdir(parents=True, exist_ok=True)
        self.lock = threading.RLock()
        self.db = sqlite3.connect(path, check_same_thread=False, timeout=15)
        self.db.row_factory = sqlite3.Row
        self.db.execute("PRAGMA journal_mode=WAL")
        self.db.execute("PRAGMA synchronous=FULL")
        self.db.execute("PRAGMA foreign_keys=ON")
        self.db.executescript("""
            CREATE TABLE IF NOT EXISTS alerts (
                id TEXT PRIMARY KEY, source TEXT NOT NULL, data TEXT NOT NULL,
                removed INTEGER NOT NULL DEFAULT 0, updated_at REAL NOT NULL);
            CREATE TABLE IF NOT EXISTS aliases (
                alias TEXT PRIMARY KEY, alert_id TEXT NOT NULL REFERENCES alerts(id));
            CREATE TABLE IF NOT EXISTS deliveries (
                alert_id TEXT NOT NULL REFERENCES alerts(id), issi INTEGER NOT NULL,
                idempotency_key TEXT NOT NULL UNIQUE, request_json TEXT NOT NULL,
                state TEXT NOT NULL, router_id TEXT, last_error TEXT,
                attempts INTEGER NOT NULL DEFAULT 0, next_attempt REAL NOT NULL DEFAULT 0,
                suppressed INTEGER NOT NULL DEFAULT 0,
                expires_at REAL NOT NULL, updated_at REAL NOT NULL,
                PRIMARY KEY(alert_id, issi));
            CREATE TABLE IF NOT EXISTS metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);
        """)

    @contextmanager
    def transaction(self):
        with self.lock:
            with self.db:
                yield self.db

    def meta(self, key, default=None):
        with self.lock:
            row = self.db.execute("SELECT value FROM metadata WHERE key=?", (key,)).fetchone()
            return json.loads(row[0]) if row else default

    def set_meta(self, key, value):
        with self.transaction() as db:
            db.execute("INSERT OR REPLACE INTO metadata VALUES (?,?)", (key, json.dumps(value)))

    def ingest(self, records, complete=False, now=None):
        """An update keeps its original incident and delivery history, including across restarts.

        Aliases can join two previously unknown reference chains. Their histories are
        merged before any new radio matching; previously attempted deliveries win.
        """
        now = timestamp(now)
        seen = set()
        with self.transaction() as db:
            for record in records:
                record = dict(record)
                aliases = {"nina:" + str(a) for a in record.get("aliases", [record["id"]])}
                aliases.add("nina:" + record["incident_id"])
                existing = set()
                for alias in aliases:
                    row = db.execute("SELECT alert_id FROM aliases WHERE alias=?", (alias,)).fetchone()
                    if row:
                        existing.add(row[0])
                alert_id = sorted(existing)[0] if existing else "nina:" + record["incident_id"]
                candidates = [record]
                for existing_id in existing:
                    prior = db.execute("SELECT data FROM alerts WHERE id=?", (existing_id,)).fetchone()
                    if prior:
                        candidates.append(json.loads(prior[0]))
                record = max(candidates, key=lambda item: timestamp(item["sent_at"]))
                for old_id in existing - {alert_id}:
                    # Keep every original key and router ID for reconciliation.
                    # The alias union, rather than rewriting delivery IDs, prevents
                    # a pending row from replacing a previously attempted send.
                    db.execute("UPDATE aliases SET alert_id=? WHERE alert_id=?", (alert_id, old_id))
                    db.execute("UPDATE alerts SET removed=1 WHERE id=?", (old_id,))
                    seen.discard(old_id)
                old = db.execute("SELECT data FROM alerts WHERE id=?", (alert_id,)).fetchone()
                record["id"] = alert_id
                seen.add(alert_id)
                if not old or timestamp(record["sent_at"]) >= timestamp(json.loads(old[0])["sent_at"]):
                    db.execute("INSERT INTO alerts VALUES (?,?,?,0,?) ON CONFLICT(id) DO UPDATE SET data=excluded.data, removed=0, updated_at=excluded.updated_at",
                               (alert_id, "nina", json.dumps(record), now))
                for alias in aliases:
                    db.execute("INSERT OR REPLACE INTO aliases VALUES (?,?)", (alias, alert_id))
                members = db.execute("""SELECT d.* FROM deliveries d LEFT JOIN aliases a ON a.alias=d.alert_id
                    WHERE COALESCE(a.alert_id,d.alert_id)=? AND d.suppressed=0""", (alert_id,)).fetchall()
                by_radio = {}
                for row in members:
                    by_radio.setdefault(row["issi"], []).append(row)
                for rows in by_radio.values():
                    rows.sort(key=lambda r: (r["state"] == "accepted", r["attempts"] > 0 or bool(r["router_id"]), r["updated_at"]), reverse=True)
                    for loser in rows[1:]:
                        db.execute("UPDATE deliveries SET suppressed=1 WHERE alert_id=? AND issi=?", (loser["alert_id"], loser["issi"]))
            if complete:
                for row in db.execute("SELECT id FROM alerts WHERE source='nina'").fetchall():
                    if row[0] not in seen:
                        db.execute("UPDATE alerts SET removed=1 WHERE id=?", (row[0],))
                db.execute("INSERT OR REPLACE INTO metadata VALUES ('nina_success',?)", (json.dumps(now),))

    def add_manual(self, record):
        with self.transaction() as db:
            db.execute("INSERT INTO alerts VALUES (?,?,?,0,?)", (record["id"], "manual", json.dumps(record), timestamp()))

    def remove_manual(self, alert_id):
        with self.transaction() as db:
            row = db.execute("SELECT source FROM alerts WHERE id=?", (alert_id,)).fetchone()
            if row is None:
                raise KeyError(alert_id)
            if row[0] != "manual":
                raise ValueError("Nur eigene Meldungen können gelöscht werden")
            db.execute("UPDATE alerts SET removed=1 WHERE id=?", (alert_id,))

    def alerts(self):
        with self.lock:
            return [{**json.loads(row["data"]), "removed": bool(row["removed"])}
                    for row in self.db.execute("SELECT * FROM alerts ORDER BY updated_at DESC")]

    def deliveries(self, public=False):
        with self.lock:
            rows = [dict(r) for r in self.db.execute("SELECT * FROM deliveries ORDER BY updated_at DESC")]
        if public:
            for row in rows:
                row.pop("request_json")
                row.pop("idempotency_key")
        return rows

    def enqueue(self, alert_id, issi, request, expires_at, now):
        with self.transaction() as db:
            if db.execute("""SELECT 1 FROM deliveries d LEFT JOIN aliases a ON a.alias=d.alert_id
                WHERE COALESCE(a.alert_id,d.alert_id)=? AND d.issi=? LIMIT 1""", (alert_id, issi)).fetchone():
                return
            db.execute("""INSERT OR IGNORE INTO deliveries
                (alert_id,issi,idempotency_key,request_json,state,expires_at,updated_at)
                VALUES (?,?,?,?,'pending',?,?)""",
                (alert_id, issi, request["idempotency_key"], json.dumps(request, sort_keys=True), expires_at, now))

    def canonical_id(self, alert_id):
        with self.lock:
            row = self.db.execute("SELECT alert_id FROM aliases WHERE alias=?", (alert_id,)).fetchone()
            return row[0] if row else alert_id

    def alert(self, alert_id):
        with self.lock:
            row = self.db.execute("SELECT * FROM alerts WHERE id=?", (alert_id,)).fetchone()
            return {**json.loads(row["data"]), "removed": bool(row["removed"])} if row else None

    def renew_unsubmitted(self, row, request, expires_at, now):
        """Caller must first establish no router key exists and the old deadline passed."""
        with self.transaction() as db:
            db.execute("""UPDATE deliveries SET request_json=?,expires_at=?,state='pending',next_attempt=0,updated_at=?
                WHERE alert_id=? AND issi=? AND router_id IS NULL AND suppressed=0""",
                (json.dumps(request, sort_keys=True), expires_at, now, row["alert_id"], row["issi"]))

    def update_delivery(self, row, **values):
        allowed = {"state", "router_id", "last_error", "attempts", "next_attempt", "updated_at"}
        if not values or set(values) - allowed:
            raise ValueError("Invalid delivery update")
        with self.transaction() as db:
            db.execute("UPDATE deliveries SET " + ",".join(k + "=?" for k in values) + " WHERE alert_id=? AND issi=?",
                       (*values.values(), row["alert_id"], row["issi"]))

    def close(self):
        with self.lock:
            self.db.close()
