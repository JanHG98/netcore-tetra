# KMF – Vault, Backups und HSM-Grenze

`lab_file_vault` trennt Metadaten und Secret-Blobs. Der lokale Master-Key schützt die Blobs mit `lab_sha256_stream_mac_v1`. Dieses Lab-Verfahren dient der Integrationsprüfung; es ist kein HSM, kein PKCS#11-Backend und keine zertifizierte Kryptografie.

## Backup-Inhalt

`POST /api/v1/backups` schreibt ein eigenes Verzeichnis unter `storage.backup_dir` mit:

| Datei | Inhalt |
| --- | --- |
| `state.json` | Metadaten, Schlüsselversionen, Jobs und Audit |
| `vault.json` | Versiegelte Secret-Blobs |
| `manifest.json` | SHA-256-Prüfsummen, Audit-Head, Erstellungsangaben und Read-back-Ergebnis |

Die KMF liest die geschriebenen State-/Vault-Dateien zur Prüfung zurück. **Master-Key und Bootstrap-Dateien werden nicht in dieses Paket kopiert.** Ohne die unveränderte separat gesicherte `master.key` kann der Vault nicht geöffnet werden. Die für Edges provisionierten Bootstrap-Dateien müssen ebenfalls getrennt verfügbar bleiben; sie werden nicht durch ein Metadatenbackup ausgeliefert.

Das Backup verschlüsselt nicht sämtliche Metadaten: `state.json` und `manifest.json` bleiben lesbares JSON; nur die Secret-Blobs sind versiegelt. Auch diese Dateien können sensible Betriebsmetadaten enthalten.

## Wiederherstellung

Ein Restore-Endpunkt ist derzeit nicht implementiert. Eine Wiederherstellung muss offline mit gestopptem Dienst, geprüften Manifest-Prüfsummen, passender Schlüsselwurzel und wiederhergestellten Dateirechten erfolgen. Die Anleitung behauptet deshalb keine automatisierte Restore-Abnahme.

## Reservierte HSM-Konfiguration

```toml
[vault]
provider = "lab_file_vault"
hsm_library = "/path/to/pkcs11.so"
hsm_slot = 0
```

Diese optionalen Felder sind lediglich reserviert: Status kann `hsm_configured=true` melden, während `hsm_connected=false` bleibt. Andere Provider werden von der Konfigurationsprüfung abgelehnt. Ein späteres HSM-Backend benötigt eigenständige Operationen für Generation, Wrap/Unwrap, Zerstörung, Referenzen, Backup-Policy und Health/Attestation.

**Quellabgleich vom 9. Oktober 2026:** [Backup und Status](../../../system-backend/kmf/src/state.rs), [Lab-Kryptografie](../../../system-backend/kmf/src/crypto.rs) und [Providerprüfung](../../../system-backend/kmf/src/config.rs).
