# Phase 11c – Exklusive SIP-Registrierung und lokale Notvermittlung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 11c – Exklusive SIP-Registrierung und lokale Notvermittlung. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [sip-switch](../../services/sip-switch/README.md) und [Quellcode](../../../system-backend/sip-switch). Der lokale TBS-Asterisk und die exklusive Registrierungsumschaltung gehören zur aktuellen Fallbackbeschreibung; der alte Phase-11b-Dialplan allein beschreibt den Betrieb nicht vollständig.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 11c – Exklusive SIP-Registrierung und lokale Notvermittlung

Phase 11c ersetzt den rein dialplanbasierten Rückfall aus Phase 11b durch eine lokale State Machine auf jeder TBS.

## Ziel

- Normalbetrieb ausschließlich über den zentralen NetCore SIP-Switch
- direkte PBX-Registrierung der TBS im Normalbetrieb nicht aktiv
- Umschaltung erst nach bestätigtem Ausfall
- Hysterese bei Wiederkehr
- keine doppelte absichtlich aktive Registrierung einer TBS
- kein Neustart der nativen TBS-SIP-/Codec-Bridge

## Zustände

```text
CENTRAL_ACTIVE
FAILOVER_PENDING
PBX_DIRECT_ACTIVE
RECOVERY_PENDING
```

## Schutzmechanismen

1. Getrennte PJSIP-Registrierungsdateien
2. Atomarer einzelner Include `netcore-active-registration.conf`
3. Asterisk-Reload nur des Outbound-Registration-Moduls, ersatzweise `core reload`
4. explizites `pjsip send unregister` beim Failback
5. Dialplan-Gate über `DB(netcore/failover_mode)`
6. Fehlerzähler und Wiederkehr-Hysterese
7. kurze Registrierungslaufzeit zur Begrenzung veralteter Kontakte nach hartem Ausfall

## Wichtige Grenze

Ein hart ausgefallener zentraler Switch kann seine alte Registrierung am PBX nicht mehr sauber mit `Expires: 0` abmelden. Der Kontakt kann daher bis zum Ablauf der PBX-Registrierungszeit sichtbar bleiben. Die TBS hält ihn jedoch nicht parallel aktiv; nach dem Umschalten wird ausschließlich die direkte PBX-Registrierung geladen.
