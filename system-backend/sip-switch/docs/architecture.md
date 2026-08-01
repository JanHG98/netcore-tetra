# Architektur und Zuständigkeiten

## Was der Dienst ist

Der SIP Switch ist ein zentraler SIP-Router/B2BUA für NetCore-TETRA. Asterisk wird als technische SIP-Engine eingesetzt, nicht als zweite PBX. Es werden keine normalen Telefoniebenutzer, Mailboxen, IVR-Menüs oder DECT-Teilnehmer verwaltet.

## PBX → TETRA

1. Das vorhandene PBX sendet einen Ruf über den einen NetCore-Trunk.
2. `netcore-sip-route.py` übergibt Zielnummer, Caller-ID und Kanalquelle an die lokale API.
3. Der Dienst normalisiert die Zielnummer zu einer ISSI.
4. Mobility Core liefert `serving_node` und Routenzustand.
5. Der SIP Switch ordnet den Node einem PJSIP-Endpunkt zu.
6. `PJSIP_DIAL_CONTACTS()` wählt den aktuell registrierten Kontakt dieser TBS.
7. Der vorhandene TBS-SIP-Connector erzeugt den TETRA-Individualruf und transcodiert PCMU/TETRA.

## TETRA → PBX

1. Der lokale SIP-Connector der TBS wählt seine konfigurierte Telefoniepräfixroute.
2. Der SIP-Ruf geht an den registrierten TBS-Endpunkt im SIP Switch.
3. Die AGI erkennt den Quellendpunkt aus dem PJSIP-Kanalnamen.
4. Die Zielnummer wird über den einzigen PBX-Endpunkt weitergegeben.

## Nummernmodell

Eine rein numerische PBX-Zielnummer wird standardmäßig direkt als 24-Bit-ISSI interpretiert. Optional sind ein Präfix und explizite `[[number_mappings]]` möglich. Gruppenrufe werden in dieser Phase nicht über den lokalen TBS-SIP-Connector angeboten, weil dessen eingehender Pfad Individualrufe auf eine ISSI erzeugt.

## Handover

Neue Rufe verwenden immer die aktuelle Mobility-Core-Route. Ein Zellwechsel während eines laufenden SIP-Rufs wird im Monitoring sichtbar, aber der aktive lokale TBS-Medienpfad wird noch nicht live umgehängt. Das spätere zentrale Medienmodell benötigt:

- externes SIP-Leg im Call Control,
- Media-Switch-Leg für den SIP Switch,
- gemeinsamen TETRA-Mediencodec außerhalb der TBS-Runtime,
- RTP/PCM↔TETRA-ACELP am zentralen Gateway,
- Call-Restore mit unverändertem PBX-SIP-Dialog.
