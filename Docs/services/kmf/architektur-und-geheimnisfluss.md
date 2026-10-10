# KMF – Architektur und Secret-Fluss

Die KMF hält die maßgebliche Lab-Lifecycle-Sicht für **CCK/GCK/SCK**. Der Security Core verwaltet unabhängig davon Authentisierung, Security-Class-Policy, Disable/Enable und kurzlebige DCK-Kontexte. Eine automatische Provider-Anbindung zwischen beiden Diensten ist derzeit nicht implementiert.

## Getrennte Persistenz

| Datei | Inhalt |
| --- | --- |
| `state.json` | Schlüsselmetadaten, Versionen, Jobs, Zustände, Audit und Fingerprints |
| `vault.json` | Mit dem Lab-Envelope versiegelte Secret-Blobs |
| `master.key` | Lokale 32-Byte-Schlüsselwurzel für den Vault |
| `bootstrap/*.json` | Node-ID und nodebezogenes Bootstrap-Geheimnis für den Lab-Edge-Test |

Neue private Dateien werden mit `0600` angelegt. Unter dem installierten Dienst begrenzt außerdem `UMask=0077` die Rechte. Versiegelte Daten bleiben von der Lab-Kryptografie und dem Schutz der Schlüsselwurzel abhängig; diese Ablage ist kein zertifiziertes HSM.

## Schlüsselweg

1. Zufälliges CCK/GCK/SCK-Material entsteht aus `/dev/urandom` im Prozess.
2. `seal(master.key, context)` erzeugt den Blob für `vault.json`.
3. Der OTAR-Claim öffnet den Vault-Eintrag im Prozessspeicher.
4. Die KMF versiegelt das Material erneut mit einem aus Node-Geheimnis und Node-ID abgeleiteten Transport-Key.
5. Die Edge erhält Key-ID, Version, Fingerprint, Crypto Period, Ziele, Envelope und Context; sie öffnet und prüft diese mit ihrem Bootstrap-Geheimnis.

Der Master-Key wird dabei nicht an die Edge verteilt. Die Bootstrap-Datei muss für den Lab-Adapter separat bereitgestellt werden; eine API-Anmeldung oder automatische sichere Provisionierung ist nicht implementiert.

## Prozessgrenzen

Managementantworten sind redigiert und enthalten kein Klartextmaterial. Der Claim besitzt das Protokoll `netcore-kmf-otar-edge-v1` und bleibt eine vorbereitende Control-Plane-Schnittstelle. Die Umsetzung auf das TETRA-Air-Interface und ein produktiver Secret-Provider sind weitere Arbeitsschritte.

**Quellabgleich vom 9. Oktober 2026:** [state.rs](../../../system-backend/kmf/src/state.rs), [crypto.rs](../../../system-backend/kmf/src/crypto.rs) und [protocol.rs](../../../system-backend/kmf/src/protocol.rs). Weiter: [OTAR](otar-zustellablauf.md) und [Backups](vault-backups-und-hsm.md).
