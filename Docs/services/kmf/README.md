# NetCore-Tetra KMF

**Quellen:** [system-backend/kmf](../../../system-backend/kmf) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Die **Key Management Facility** ist der zentrale Lifecycle-Dienst für TETRA-Netz- und Gruppenschlüssel (CCK/GCK/SCK). Dieses Paket setzt den Roadmap-Baustein nach dem Security Core um und verwaltet:

- Common Cipher Keys (**CCK**),
- Group Cipher Keys (**GCK**),
- Static Cipher Keys (**SCK**),
- Key-Versionen und Vorgänger-/Nachfolgerketten,
- Crypto Periods,
- Rotation,
- vorbereitete OTAR-Zustellungen,
- nodegebundene Transportprofile,
- verschlüsselte Backups,
- hashverkettetes Audit,
- eine eigene WebUI auf Port **8190**.

## Wichtige Sicherheitsgrenze

Die normale WebUI und Management-API liefern **niemals Rohschlüssel**. Das gilt auch für Audit, Metrics, OpenAPI, Status, Export und Fehlermeldungen.

OTAR-Claims enthalten Schlüsselmaterial ausschließlich als an das Ziel-Node gebundenen `SealedBlob`. Das nötige Bootstrap-Geheimnis wird als lokale Datei mit Modus `0600` erzeugt und nicht in einer API-Antwort ausgegeben.

## Open-Lab-Modus

Die aktuelle Testphase bleibt ausdrücklich offen:

- keine Benutzerkonten,
- keine Tokens,
- kein TLS,
- keine echte Identitätsprüfung bei der Vier-Augen-Freigabe.

Deshalb darf die KMF nur in einem isolierten Managementnetz laufen. Die Actor-Namen bei Freigaben sind im Open-Lab-Modus deklarativ; die technische Erzwingung verschiedener Namen ersetzt noch keine echte Authentisierung.

## Shadow und Authoritative

```toml
[policy]
operating_mode = "shadow"
```

`shadow` erzeugt Schlüssel, Rotationen, Jobs und Zustellungen, gibt aber keine Aktion an eine TBS Edge frei.

```toml
[policy]
operating_mode = "authoritative"
```

`authoritative` erlaubt vollständig freigegebenen und gequeueten OTAR-Aktionen, vom passenden Node über den Edge-Endpunkt beansprucht zu werden.

## Was dieses Paket bewusst noch nicht behauptet

- `lab_file_vault` ist kein HSM.
- `lab_sha256_stream_mac_v1` ist ein Integrations-Envelope, kein zertifiziertes Produktionsverfahren.
- Das Paket implementiert noch keine TETRA-TA-Algorithmen.
- Es kodiert noch keine D-OTAR-Air-Interface-PDUs.
- Es ersetzt keine produktive PKI, RBAC- oder Vier-Augen-Identitätsprüfung.

Die KMF liefert die sichere Control-Plane, Metadaten, Lifecycle- und Transporthülle. Der spätere Air-Interface-OTAR-Baustein setzt darauf auf.

## Schnellstart

```bash
sudo system-backend/kmf/install/install.sh
```

Danach:

```text
http://<KMF-LXC-IP>:8190/
```

## Verzeichnisstruktur

```text
system-backend/kmf/
├── config/kmf.example.toml
├── docs/
├── install/
├── src/
├── systemd/netcore-kmf.service
└── tests/
```

## Kernendpunkte

```text
GET  /api/v1/status
GET  /api/v1/keys
POST /api/v1/keys
POST /api/v1/keys/{id}/rotate
POST /api/v1/keys/{id}/activate
POST /api/v1/nodes
POST /api/v1/otar/jobs
POST /api/v1/otar/jobs/{id}/approve
POST /api/v1/otar/jobs/{id}/queue
POST /api/v1/edge/actions/claim
POST /api/v1/edge/actions/{id}/ack
POST /api/v1/backups
GET  /api/v1/export.json
```

Weitere Details stehen in `docs/architecture.md`, `docs/key-lifecycle.md`, `docs/otar-workflow.md` und `docs/vault-backup-hsm.md`.
