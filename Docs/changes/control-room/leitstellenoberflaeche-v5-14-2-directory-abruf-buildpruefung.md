# Leitstelle v5.14.2 – Directory Pull Verified Buildkorrektur

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room v5.14.2 – Directory Pull Verified Buildfix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room v5.14.2 – Directory Pull Verified Buildfix

Dieses Paket ist der harte Gegencheck für den Fehler:

```text
error[E0599]: no method named `len` found for enum `serde_json::Value`
```

## Was geprüft wurde

Im Paket ist kein `resolved.len()` mehr enthalten.

Stattdessen wird überall gezählt mit:

```rust
resolved.as_object().map(|object| object.len()).unwrap_or(0)
```

Außerdem enthält `bins/netcore-control-room/src/http.rs` den Marker:

```rust
V5_14_2_NO_RESOLVED_LEN_MARKER
```

## Wichtig

Wenn der Compiler danach immer noch exakt diese Zeile meldet:

```text
"resolved_subscriber_count": resolved.len(),
```

dann baut Cargo nicht aus dem entpackten v5.14.2-Stand, sondern aus einem alten Arbeitsbaum.

## LXC komplett sauber entpacken

```bash
cd /opt/netcore

systemctl stop netcore-control-room || true

rm -rf /opt/netcore/flowstation
mkdir -p /opt/netcore/flowstation

cd /opt/netcore/flowstation
unzip -o /root/netcore-control-room-v5-14-2-directory-pull-verified-files.zip
```

Falls du in ein bestehendes Git-Repo entpackst:

```bash
cd /opt/netcore/flowstation
unzip -o /root/netcore-control-room-v5-14-2-directory-pull-verified-files.zip
```

## Vor dem Build prüfen

```bash
cd /opt/netcore/flowstation

grep -R "resolved\.len()" -n bins/netcore-control-room system-backend/control-room || echo "OK: kein resolved.len mehr"
grep -R "V5_14_2_NO_RESOLVED_LEN_MARKER" -n bins/netcore-control-room/src/http.rs
```

Der erste Befehl darf keine Fundstelle zeigen.
Der zweite muss den Marker zeigen.

## Build

```bash
cd /opt/netcore/flowstation

cargo clean -p netcore-control-room
cargo build --release \
  -p netcore-control-room \
  -p netcore-control-room-operator

systemctl daemon-reload
systemctl start netcore-control-room
```

## Directory testen

```bash
curl -u admin:DEIN_PASSWORT http://127.0.0.1:9010/api/directory/resolved | jq
```
