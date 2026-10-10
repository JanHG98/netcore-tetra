# Leitstelle v5.14.0 – Directory-API Pull

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room v5.14.0 – Directory-API Pull. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room v5.14.0 – Directory-API Pull

Dieses Update macht genau das, was eigentlich gemeint war: Der Control-Room-LXC holt Namen aus der bestehenden NetCore Directory API.

## Warum v5.13 noch nicht gereicht hat

v5.13 hat den Import-Endpunkt gebaut, aber wenn niemand diesen Endpunkt befüllt, bleibt `/api/directory` leer.

Dein LXC lieferte:

```json
{
  "hide_infrastructure": true
}
```

Damit gibt es keine Namen für die UI.

## Neu in v5.14

Der LXC fragt jetzt automatisch die bestehende NetCore Directory API ab:

Standard:

```text
http://127.0.0.1:8095
```

Genutzte Endpunkte:

```text
GET /api/devices
GET /api/basestations
GET /api/groups
GET /api/device-groups
GET /api/status
```

Daraus baut der Control Room automatisch:

```text
/api/directory
/api/directory/resolved
```

Die Windows UI nutzt dann `/api/directory/resolved`.

## Anderer Host / Port

Wenn dein Directory Server nicht lokal auf dem Control-Room-LXC läuft, setze im systemd-Service:

```ini
Environment=NETCORE_DIRECTORY_API=http://10.0.1.25:8095
```

oder passend:

```ini
Environment=NETCORE_DIRECTORY_API=http://<DIRECTORY-IP>:8095
```

Danach:

```bash
systemctl daemon-reload
systemctl restart netcore-control-room
```

## LXC Update

```bash
cd /opt/netcore/flowstation

systemctl stop netcore-control-room || true

cargo clean -p netcore-control-room
cargo build --release \
  -p netcore-control-room \
  -p netcore-control-room-operator

systemctl daemon-reload
systemctl start netcore-control-room
```

## Diagnose

```bash
curl -u admin:DEIN_PASSWORT http://127.0.0.1:9010/api/directory/upstream | jq
curl -u admin:DEIN_PASSWORT http://127.0.0.1:9010/api/directory/resolved | jq
```

Wenn `/api/directory/upstream` `ok: true` und `resolved_subscriber_count > 0` zeigt, muss die UI Namen anzeigen.

Wenn `ok: false`, dann erreicht der Control Room den Directory Server nicht. Dann stimmt Host/Port/Service nicht.

## Windows

Die UI-Version ist:

```text
Native UI v5.14.0 · Directory-API Pull
```
