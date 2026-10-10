# Security Core – Lab-Provider und Geheimnisse

`lab_hmac_sha256` leitet mit HMAC-SHA-256 Teilnehmerprüfwerte aus einem lokalen Seed und der ISSI ab. Die Antwort ist zusätzlich an Node-ID, Kontext-ID und Challenge gebunden. Neue Seeds werden mit 32 Bytes Zufall aus `/dev/urandom` und Dateimodus `0600` angelegt; vorhandene Seeds müssen mindestens 32 Bytes enthalten.

## Was wo sichtbar ist

| Material | Ablage und Ausgabe |
| --- | --- |
| Seed | Lokale Datei `storage.lab_seed_path`, keine API-Ausgabe |
| Abgeleiteter Teilnehmerschlüssel / erwartete Antwort | Prozessspeicher, keine Managementausgabe |
| Challenge / DCK | Prozessspeicher; Ausgabe nur im Edge-Claim bei freigegebenem Export |
| Fingerprints, Referenzen, Zustände | Metadaten, Audit und Management-API |

Eine Challenge wird zufällig erzeugt; „deterministisch“ beschreibt die Ableitung der Prüfwerte aus den Eingaben, keine feste Challenge. Nach einem ACK wird das Aktionspayload gelöscht. DCK-Material und erwartete Antwort folgen ihrem eigenen Ablauf- und Widerrufsworkflow.

## Lab-Prüfung und Grenzen

[lab_response.py](../../../system-backend/security-core/tests/lab_response.py) berechnet eine Lab-Antwort mit Seed, ISSI, Node, Kontext und Challenge. Dies ist kein Endgerätetest mit TETRA-TA-Algorithmen. Ein Seed-Zugriff ist deshalb ausschließlich für den isolierten Integrationsaufbau vorgesehen.

Die KMF stellt bisher keinen produktiven Ersatzkanal für diesen Provider bereit. Schlüsseldateirechte, Prozessspeicher und Netzisolation bleiben Bestandteil des Lab-Aufbaus; vorhandene Secret-Dateien werden beim Laden nicht automatisch einer umfassenden Rechteprüfung unterzogen.

**Quellabgleich vom 9. Oktober 2026:** [crypto.rs](../../../system-backend/security-core/src/crypto.rs) und [state.rs](../../../system-backend/security-core/src/state.rs).
