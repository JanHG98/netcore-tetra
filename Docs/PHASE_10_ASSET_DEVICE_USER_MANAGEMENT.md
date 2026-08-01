# Phase 10 – Asset-, Geräte- und Benutzerverwaltung

## Ziel

Eine zentrale, persistente Sicht auf Funkgeräte, Basisstationen, Racks, Zubehör, Personen, Geräteausgaben und Wartung – ohne die fachliche Autorität von Subscriber Core und Mobility Core zu duplizieren.

## Neuer Dienst

- Dienst: `asset-management`
- Port: `8290/tcp`
- WebUI: `http://<LXC-IP>:8290/`
- Betriebsart: OPEN LAB

## Grenzen

- Subscriber Core: ISSI-Zulassung und Dienstberechtigungen
- Mobility Core: Serving-TBS und Registrierung
- Asset Management: physischer Bestand, Zuordnung, Firmware/Codeplug, Wartung
- Task Workflow: ausführbare Wartungsaufträge

RUI/RUA-Felder sind vorbereitete Metadaten. Es werden keine PINs gespeichert und keine Funkgeräteanmeldung ausgelöst.
