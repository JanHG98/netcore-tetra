# KMF – Schlüsselverwaltung und Lab-OTAR

Der KMF verwaltet die Schlüsselklassen CCK/GCK/SCK im unten beschriebenen Labormodell.

Die Key Management Facility verwaltet den Lebenszyklus der Common Cipher Keys (**CCK**), Group Cipher Keys (**GCK**) und Static Cipher Keys (**SCK**). Sie hält Versionen, Vorgänger-/Nachfolgerketten, Crypto Periods, Rotation, nodegebundene Transportprofile, OTAR-Jobs und ein hashverkettetes Audit. Weboberfläche und API verwenden standardmäßig TCP **8190**.

**Stand: 9. Oktober 2026.** Abgeglichen mit [Konfiguration](../../../system-backend/kmf/config/kmf.example.toml), [API](../../../system-backend/kmf/src/http.rs) und [Lifecycle-Logik](../../../system-backend/kmf/src/state.rs). Beschrieben ist der vorhandene Lab-Code, keine zertifizierte Schlüsselhaltung oder On-Air-OTAR-Abnahme.

## Schlüsselgrenzen

Die Management-API, WebUI, Status, Metrics und Export liefern keine Rohschlüssel. Der Edge-Claim liefert Schlüsselmaterial nur im nodegebundenen `SealedBlob`. Das Bootstrap-Geheimnis wird serverseitig in einer lokalen Datei angelegt; die API nennt lediglich Pfad, Fingerprint und Metadaten.

`lab_file_vault` und `lab_sha256_stream_mac_v1` dienen Integrationstests. Implementiert sind **kein HSM/PKCS#11-Provider, keine TETRA-TA-Algorithmen und keine D-OTAR-Air-Interface-PDUs**. Der [Security Core](../security-core/README.md) bleibt für Authentisierung, Security-Class-Policy, Disable/Enable und kurzlebige DCK-Kontexte zuständig; die KMF ersetzt dessen Lab-Provider derzeit nicht.

## Betrieb

Aus dem Repository-Hauptverzeichnis auf einem Linux-Lab-Host mit systemd, `iproute2` und passender Rust-/Cargo-Toolchain:

```bash
sudo system-backend/kmf/install/install.sh
systemctl status netcore-kmf
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/ready"
```

`shadow` erlaubt Schlüssel- und Jobvorbereitung, gibt jedoch keine Aktionen an die Edge frei. `authoritative` erlaubt Claims vollständig freigegebener und gequeueter Aktionen. Die Beispielkonfiguration beginnt in `shadow` und bindet an `0.0.0.0:8190`.

Die aktuelle API hat **keine Anmeldung, Tokens oder TLS**. Die verlangten zwei unterschiedlichen Actor-Namen sind eine Workflow-Prüfung, keine verifizierten Vier-Augen-Identitäten. Der Dienst gehört ausschließlich ins isolierte Management-Lab.

## Wichtige API-Pfade

| Aufgabe | Pfad |
| --- | --- |
| Schlüssel ansehen/erzeugen | `GET` / `POST /api/v1/keys` |
| Rotieren/aktivieren | `POST /api/v1/keys/{id}/rotate` / `activate` |
| Node-Profil erzeugen | `POST /api/v1/nodes` |
| Job erzeugen, freigeben, queueen | `POST /api/v1/otar/jobs`, `/{id}/approve`, `/{id}/queue` |
| Edge-Aktion beanspruchen/quittieren | `POST /api/v1/edge/actions/claim`, `/{id}/ack` |
| Wartung / Backup | `POST /api/v1/maintenance/tick`, `POST /api/v1/backups` |

Es läuft kein automatischer KMF-Wartungstimer; zeitbezogene Zustandsbereinigung erfolgt über den Wartungsendpunkt.

## Anleitungen und Quellen

- [Architektur und Secret-Fluss](architektur-und-geheimnisfluss.md)
- [Schlüssellebenszyklus](schluessellebenszyklus.md)
- [OTAR-Ablauf](otar-zustellablauf.md)
- [Vault, Backup und HSM-Grenze](vault-backups-und-hsm.md)
- [Offene Testumgebung](offene-testumgebung.md)
- [LXC-Installation](lxc-installation.md)
- [API-Beispiele](tests/api-beispiele-im-labor.md)

Quellcode, Konfiguration, Installer, systemd-Unit und Testhelfer liegen unter [system-backend/kmf](../../../system-backend/kmf). Die ausführlichen Anleitungen liegen ausschließlich hier unter `Docs/services/kmf/`.
