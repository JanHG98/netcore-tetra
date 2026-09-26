# Änderung: End-to-End-Aussendung aus der Media Library

- neuer Playout-Modus `basisstation`
- mehrere TBS-Ziele mit Standardstation in `[playout]`
- Cookie-Login am geschützten TBS-Dashboard
- Übergabe von Asset-ID, GSSI/ISSI und Priorität an `/api/audio/play`
- TBS übernimmt vollständigen Download, lokalen Cache, nativen Codec und Rufaufbau
- Remote-Job-ID und Blockfortschritt werden in Media-Library-Jobs gespiegelt
- Abbruch über `/api/audio/stop`
- WAV-/TTS-Assets benötigen im Basisstationsmodus keinen zentralen TACELP-Cache
- Altpfad `media_switch` bleibt für vorhandene TACELP-/Session-Workflows erhalten
- bestehende Konfigurationen erhalten den `[playout]`-Block über eine idempotente Migration
