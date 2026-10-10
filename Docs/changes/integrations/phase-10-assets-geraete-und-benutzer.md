# Phase 10 – Asset-, Geräte- und Benutzerverwaltung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 10 – Asset-, Geräte- und Benutzerverwaltung. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [asset-management](../../services/asset-management/README.md) und [Quellcode](../../../system-backend/asset-management). Physischer Bestand und Personen-/Gerätezuordnung ersetzen keine Teilnehmerfreigabe durch Subscriber Core oder Serving-TBS-Lage durch Mobility Core.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 10 – Asset-, Geräte- und Benutzerverwaltung

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
