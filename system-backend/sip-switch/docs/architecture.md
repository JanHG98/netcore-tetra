# Architektur und Zuständigkeiten

## Rollen

- **Vorhandenes PBX:** Nebenstellen, DECT, Rufgruppen, externer Rufnummernplan.
- **Zentraler SIP Switch:** Mobility-Core-Auflösung und Auswahl der Serving-TBS.
- **Lokaler TBS-Asterisk:** stabiler Edge-B2BUA, Primär-/Fallback-Trunk und lokaler SIP-Anker.
- **Native TBS-Bridge:** TETRA-Rufsteuerung und PCMU↔TETRA-Codec.

## Wesentlicher Grundsatz

Die native TBS-Bridge registriert sich nicht direkt am zentralen SIP-Switch. Sie registriert sich an ihrem lokalen Asterisk. Der lokale Asterisk registriert sich wiederum als TBS-Endpunkt am zentralen SIP-Switch und hält zusätzlich einen direkten Fallbacktrunk zum PBX.

```text
Native TBS Bridge ⇄ lokaler Asterisk ⇄ zentraler SIP Switch ⇄ PBX
                                  └──── direkter PBX-Fallback ────┘
```

## PBX → TETRA im Normalbetrieb

1. Das PBX sendet den Ruf an den zentralen SIP-Switch.
2. Der SIP-Switch fragt den Mobility Core nach der Serving-TBS.
3. Der zentrale Asterisk wählt den registrierten lokalen Asterisk dieser TBS.
4. Der lokale Asterisk schreibt die Ziel-ISSI in die Request-URI des nativen TBS-Kontakts.
5. Die native TBS-Bridge erzeugt den TETRA-Individualruf.

## PBX → TETRA im Fallback

1. Das PBX erkennt den zentralen NetCore-Trunk als nicht verfügbar.
2. Eine PBX-Failoverroute wählt die direkten TBS-Fallbacktrunks.
3. Jeder lokale TBS-Asterisk versucht die Ziel-ISSI an seine native TBS-Bridge zu liefern.
4. TBS ohne lokalen Teilnehmer antworten nicht erfolgreich; die passende TBS nimmt den Ruf an.

Das PBX bleibt damit die Stelle, die den **eingehenden** Trunk-Failover auslöst. Eine ausgefallene zentrale Instanz kann ihren eigenen Ausfall naturgemäß nicht mehr signalisieren.

## TETRA → PBX

1. Die native TBS-Bridge wählt den lokalen Asterisk.
2. Der lokale Asterisk versucht zuerst den zentralen SIP-Switch.
3. Nur bei `CHANUNAVAIL` oder `CONGESTION` wird der direkte PBX-Trunk gewählt.
4. `BUSY` und `NOANSWER` werden unverändert an die TBS zurückgegeben und nicht dupliziert.

## Rufstabilität

Die Route wird nur beim Aufbau eines neuen Dialogs gewählt. Laufende Gespräche bleiben auf ihrem bestehenden SIP-/RTP-Pfad. Ein automatisches Mid-Call-Failover wäre ohne zentralen Medienpfad nicht zuverlässig und ist deshalb bewusst nicht enthalten.
