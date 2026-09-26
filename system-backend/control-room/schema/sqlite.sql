-- NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Leitstellenfunktionen und Bedienoberflächen.
-- NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

-- NetCore Control Room SQLite schema v2.
-- The service auto-applies this schema on startup; this file is documentation/reference.

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS node_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id TEXT NOT NULL,
    station_name TEXT,
    site TEXT,
    connected_at TEXT NOT NULL,
    disconnected_at TEXT,
    protocol_version TEXT,
    stack_version TEXT,
    raw_hello TEXT
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_node_sessions_node_time ON node_sessions(node_id, connected_at DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    node_id TEXT NOT NULL,
    seq INTEGER,
    event_type TEXT NOT NULL,
    event_json TEXT NOT NULL
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_events_node_time ON events(node_id, timestamp DESC);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_events_type_time ON events(event_type, timestamp DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS commands (
    command_id TEXT PRIMARY KEY,
    target_node_id TEXT NOT NULL,
    operator_id TEXT,
    issued_at TEXT NOT NULL,
-- Was: Ändert vorhandene Datensätze.
-- Warum: Gespeicherter Zustand bleibt dadurch mit dem aktuellen Betrieb synchron.
    updated_at TEXT NOT NULL,
    status TEXT NOT NULL,
    target_entity_json TEXT,
    message TEXT,
    command_json TEXT NOT NULL,
    responses_json TEXT NOT NULL DEFAULT '[]'
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_commands_node_time ON commands(target_node_id, updated_at DESC);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_commands_status_time ON commands(status, updated_at DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS sds_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id TEXT NOT NULL,
    station_name TEXT,
    timestamp TEXT NOT NULL,
    direction TEXT NOT NULL,
    source_issi INTEGER NOT NULL,
    dest_issi INTEGER NOT NULL,
    is_group INTEGER NOT NULL DEFAULT 0,
    protocol_id INTEGER NOT NULL,
    text TEXT NOT NULL
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_sds_node_time ON sds_log(node_id, timestamp DESC);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_sds_source_time ON sds_log(source_issi, timestamp DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS locations (
    node_id TEXT NOT NULL,
    station_name TEXT,
    issi INTEGER NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    source TEXT NOT NULL,
-- Was: Ändert vorhandene Datensätze.
-- Warum: Gespeicherter Zustand bleibt dadurch mit dem aktuellen Betrieb synchron.
    updated_at TEXT NOT NULL,
    raw_text TEXT,
    PRIMARY KEY (node_id, issi)
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_locations_time ON locations(updated_at DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS emergencies (
    node_id TEXT NOT NULL,
    station_name TEXT,
    source_issi INTEGER NOT NULL,
    dest_ssi INTEGER NOT NULL,
    active INTEGER NOT NULL,
    raised_at TEXT NOT NULL,
    cleared_at TEXT,
    PRIMARY KEY (node_id, source_issi, raised_at)
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_emergencies_active_time ON emergencies(active, raised_at DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS auth_tokens (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    role TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
-- Was: Ändert vorhandene Datensätze.
-- Warum: Gespeicherter Zustand bleibt dadurch mit dem aktuellen Betrieb synchron.
    updated_at TEXT NOT NULL,
    last_used_at TEXT,
    expires_at TEXT,
    created_by TEXT
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_auth_tokens_role ON auth_tokens(role, enabled);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_auth_tokens_created ON auth_tokens(created_at DESC);

-- Was: Legt eine neue Datenbanktabelle und ihre Felder an.
-- Warum: Die Anwendung benötigt eine feste Struktur, damit Daten dauerhaft und eindeutig gespeichert werden.
CREATE TABLE IF NOT EXISTS auth_users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    password_salt TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
-- Was: Ändert vorhandene Datensätze.
-- Warum: Gespeicherter Zustand bleibt dadurch mit dem aktuellen Betrieb synchron.
    updated_at TEXT NOT NULL,
    last_login_at TEXT,
    created_by TEXT
);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_auth_users_role ON auth_users(role, enabled);
-- Was: Legt einen Suchindex für ausgewählte Felder an.
-- Warum: Häufige Abfragen werden dadurch auch bei wachsenden Datenmengen schnell ausgeführt.
CREATE INDEX IF NOT EXISTS idx_auth_users_updated ON auth_users(updated_at DESC);

-- Was: Schreibt einen neuen Datensatz in die Datenbank.
-- Warum: Der neue Zustand muss dauerhaft gespeichert und nach Neustarts wieder verfügbar sein.
INSERT OR IGNORE INTO schema_migrations(version) VALUES (1);
-- Was: Schreibt einen neuen Datensatz in die Datenbank.
-- Warum: Der neue Zustand muss dauerhaft gespeichert und nach Neustarts wieder verfügbar sein.
INSERT OR IGNORE INTO schema_migrations(version) VALUES (2);
-- Was: Schreibt einen neuen Datensatz in die Datenbank.
-- Warum: Der neue Zustand muss dauerhaft gespeichert und nach Neustarts wieder verfügbar sein.
INSERT OR IGNORE INTO schema_migrations(version) VALUES (3);
