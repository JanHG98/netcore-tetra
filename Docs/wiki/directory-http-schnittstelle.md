# HTTP-Schnittstelle des NetCore Directory

Die API wird von Weboberfläche, Basisstation und optionalen Hilfswerkzeugen genutzt. Der aktuelle Python-Handler enthält keine eigene Authentisierung. Die Beispiele verwenden Platzhalter; Netzgrenze und gegebenenfalls einen vorgeschalteten gesicherten Zugang separat prüfen.

## Basis-Endpunkte

| Methode | Endpunkt | Zweck |
|---|---|---|
| `GET` | `/api/health` | Dienstzustand |
| `GET` | `/api/export` | vollständiger Datenexport |
| `POST` | `/api/import` | Datenimport |
| `GET`, `POST`, `PUT`, `PATCH`, `DELETE` | `/api/devices` | Geräte |
| `GET`, `POST`, `PUT`, `PATCH`, `DELETE` | `/api/basestations` | Basisstationen |
| `GET`, `POST`, `PUT`, `PATCH`, `DELETE` | `/api/groups` | Gruppen |
| `GET`, `POST`, `PUT`, `PATCH`, `DELETE` | `/api/device-groups` | Gerätegruppen/Statusgruppen |
| `GET`, `POST`, `PUT`, `PATCH`, `DELETE` | `/api/status` | Statusmeldungen |
| `GET` | `/api/status-group-members?issi=<ISSI>` | Statusgruppen eines Geräts und Mitglieder |

Je nach Client stehen Alias-Endpunkte für `status-groups` oder `vehicles` zur Verfügung.

## RadioID-kompatible Abfragen

```text
GET /api/dmr/user/?id=<ISSI>
GET /api/dmr/repeater/?id=<ISSI>
```

Diese Routen erleichtern die Wiederverwendung bestehender Lookup-Logik.

## Export

```bash
curl -fsS http://<DIRECTORY-IP>:8095/api/export \
  -o netcore-directory-export.json
```

Exportdateien können personenbezogene oder betriebliche Informationen enthalten und sind entsprechend zu schützen.

## Import

Vor jedem Import Datenbank und bestehenden Export sichern. Beispiel:

```bash
curl -fsS -X POST \
  -H 'Content-Type: application/json' \
  --data-binary @netcore-directory-import.json \
  http://<DIRECTORY-IP>:8095/api/import
```

## Fehlercodes

- `2xx` – erfolgreich
- `400` – ungültige Nutzdaten oder fehlende Pflichtfelder
- `404` – Datensatz oder Route nicht gefunden
- Schreib-/Importfehler werden im aktuellen Handler überwiegend als `400` mit Fehlertext beantwortet.
- `POST`, `PUT` und `PATCH` verwenden denselben Upsert-Pfad. Eine vorhandene ID kann überschrieben werden; ein garantierter `409`-Konfliktschutz besteht nicht.

## Praxisregel

API-Clients sollten Timeouts setzen und einen Directory-Ausfall nicht mit einem Ausfall der RF-Basisstation gleichsetzen. Namensauflösung darf den kritischen Funkpfad nicht blockieren.

## Weiterführend

[Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) · [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) · [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md)

`PATCH` verwendet denselben Upsert-Pfad wie `POST` und `PUT`. Bei vorhandenen Datensätzen bleiben nicht übergebene Textfelder erhalten; `visible` und bei Gerätegruppen `status_sync` werden ohne explizite Angabe jedoch auf wahr gesetzt. Diese Schalter deshalb bei Teilupdates ausdrücklich mitsenden. Vor Importen und Serienänderungen Export und Datenbanksicherung erstellen.

## Quellen zur Pflege dieser Seite

[HTTP-Routen und Upsert](../../system-backend/directory/netcore-directory.py).
