# Medienbibliothek: Dateien und Archivstruktur

## Aktive Bibliothek

Die lokale Arbeitskopie liegt unter
`/var/lib/netcore-media-library/assets/<asset-id>/`: `original.*`, `preview.wav`
und optional `audio.tacelp`. Nach geprüfter Archivierung können die Nutzdaten
lokal freigegeben und vor Vorschau oder Aussendung zurückgeholt werden.
Metadaten und Archivbelege bleiben lokal.

## Gemeinsames NFS-/SMB-Archiv

Archivierte Dateien werden ohne UUID-Zwischenordner nach Kalenderdatum abgelegt:

```text
/mnt/nfs-share/Recordings/
└── 2026/
    └── 07/
        └── 29/
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_original.wav
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_preview.wav
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_tetra.tacelp
            └── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_metadata.json
```

Für TTS- und normale Medien gelten dieselben Jahres-/Monats-/Tagesordner. Der
Dateiname enthält Zeit, Typ, Titel und eine kurze Asset-ID zur Kollisionsvermeidung.
Die Metadaten-Datei enthält den vollständigen Asset-Datensatz, Prüfsummen und
Dateigrößen.

Archivordner werden explizit mit `0777`, Archivdateien mit `0666` angelegt. Das
ist in der OPEN-LAB-Umgebung beabsichtigt, damit NFS- und SMB-Clients dieselben
Dateien bearbeiten können.

**Quellabgleich: 15. Oktober 2026.** [Quellcode](../../../system-backend/media-library) · [Konfigurationsvorlagen](../../../system-backend/media-library/config).


## Automatische lokale Freigabe und Rückholung

Original, Vorschau und TETRA-Cache erhalten einen privaten Beleg mit SHA-256
und Dateigröße im lokalen Zustand. Eine lokale Freigabe erfordert dessen
Übereinstimmung mit Manifest, unabhängig gelesenen NAS-Dateien und vorhandenen
lokalen Nutzdaten. Der rückholbare Zustand wird vor dem ersten Unlink
synchronisiert. Fehlende oder beschädigte Kopien lassen Originale erhalten.

Im Abschnitt `[storage]` gelten auch bei fehlenden TOML-Feldern diese Standards:

| Einstellung | Standard | Wirkung |
| --- | --- | --- |
| `auto_offload` | `true` | Geprüfte Archivkopien dürfen lokal freigegeben werden. |
| `local_cache_max_bytes` | `2147483648` | Zielgröße des lokalen Nutzdaten-Caches: 2 GiB. |
| `local_cache_max_age_secs` | `86400` | Seit 24 Stunden nicht abgerufene Kopien sind zur Freigabe geeignet; 0 deaktiviert diese Altersregel. |
| `min_free_bytes` | `2147483648` | 2 GiB lokale Reserve für Importe und Rückholungen. |
| `archive_require_network_mount` | `true` | Ein Verzeichnis ohne tatsächlichen NFS-/SMB-Mount genügt nicht. |

Die Cachegröße ist ein Wartungsziel: Verarbeitung und wartende/laufende
Aussendungen können Nutzdaten vorübergehend festhalten. Der Dienst prüft jede
Minute und gibt höchstens32 Assets pro Durchlauf frei. `max_total_bytes`
bleibt das zusätzliche lokale Importlimit.

`auto_archive_recordings`, `auto_archive_tts` und `auto_archive_uploads`
sind standardmäßig `true`; ausdrücklich gesetzte Werte bleiben maßgeblich.
Ein fehlerhaftes Asset blockiert weitere Archivierungen nicht und wird frühestens
nach60 Sekunden erneut versucht. Alte Archivmarkierungen brauchen vor ihrer
ersten Freigabe einen frischen Beleg gegen die noch lokalen Originale;
ein editierbares NAS-Manifest allein genügt nicht.

Vorschau, Originaldownload, Wellenform, neue Aussendung, Auftragswiederholung
und erneute Verarbeitung holen ausgelagerte Dateien vorher lokal zurück.
Dies gilt auch für den TACELP-Cache direkter Media-Switch-Aussendungen. Für eine
Rückholung schaffen andere entbehrliche, geprüfte Cachekopien bei Bedarf Platz;
die lokale Reserve bleibt berücksichtigt. Vollständig vorhandene, gegen den
privaten Beleg geprüfte Arbeitskopien werden nach einem Neustart ohne erneute
Dateiallokation verwendet, auch bei ausgefallenem NAS.
Wartende und aktive Aussendungen verhindern die Freigabe ihres Assets.
Bereits geöffnete HTTP-Dateideskriptoren bleiben nach einem späteren Unlink
lesbar. Ein Neustart zwischen Zustandscommit und Teilfreigabe lässt sich
ebenfalls zurückholen. Bei NAS-Ausfall oder unzureichender Reserve bleiben
Belege und vorhandene Kopien erhalten; vorübergehend nicht rückholbare
Dateiabrufe antworten mit HTTP503.

Die WebUI zeigt freien lokalen Speicher, ausgelagerte Assets und NAS-Status.
Pro Asset ist sichtbar, ob es nur auf dem NAS oder zusätzlich lokal vorliegt.
NAS-Archive werden durch die Cachewartung nicht gelöscht.

Die Messung auf CT138 vom15.10.2026 ergab dagegen22 GB physische Syslogs bei nur
20 MB Medien. Für diese eigenständige Ursache gilt das
[automatische Logarchiv](../../deployment/logarchiv-auf-nas.md).
