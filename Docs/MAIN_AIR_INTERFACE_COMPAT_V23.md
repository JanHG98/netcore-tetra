# NetCore-TETRA SWMI v23 – Main-compatible local air interface

## Ziel

Der lokale Funkpfad der Basisstation verwendet wieder die im bereitgestellten `main`-Stand
bewährten Zustandsautomaten. Die Core-LXCs bleiben im Repository und die Provisioning-Daten
werden weiterhin übernommen, dürfen aber keine bereits eingebuchten Funkgeräte automatisch
neu registrieren oder den laufenden lokalen Rufzustand rekonstruieren.

## Wiederhergestellte Laufzeit

* MLE-Basisstationszustandsautomat und Network-Broadcast aus `main`
* MM-Registrierungs- und Location-Update-Ablauf aus `main`
* vollständige lokale CMCE-Ruf-FSM, Floor-Control, Hangtime und Release-Sequenz aus `main`
* keine aktive MM-Migration oder Cell-Change-Erweiterung im normalen lokalen Funkpfad
* keine zentrale zweite Ruf-FSM im lokalen CMCE-Pfad

Beim Start erscheint:

```text
Radio runtime: MAIN-COMPAT (local MM/MLE/CMCE state machines)
```

## Core-Anbindung

Subscriber- und Group-Policy werden weiterhin gespeichert. Die Anbindung ist bewusst nur ein
Admission-Gate:

* neue Registrierungen werden gegen die Subscriber-Policy geprüft;
* neue Gruppen-Attach-Anfragen werden gegen die Group-/Membership-Policy geprüft;
* eine Policy-Synchronisierung trennt keine bereits eingebuchten Geräte;
* sie löst keine automatische Re-Affiliation und kein Location Update aus;
* bestehende lokale Calls werden nicht durch Core-State-Machines verändert.

Advanced Mobility und zentraler Call-Control bleiben als LXC-/Datenmodell und Telemetriepfad
vorhanden, greifen im Main-Kompatibilitätsmodus jedoch nicht in die lokale Funk-FSM ein.

## Individualrufe und SIP

Simplex und Duplex werden entsprechend der tatsächlich empfangenen `U-SETUP`-Anforderung
bearbeitet. Es gibt keine Provisioning-Berechtigung und keinen Capability-Gate anhand der
registrierten `ClassOfMs`-Telemetrie.

Explizite Asterisk-/EchoLink-Wählplanrouten haben Vorrang vor der lokalen ISSI-Datenbank.
SIP-Nebenstellen müssen deshalb nicht als Funkgerät im Provisioning Core angelegt werden.

## Konfiguration

Für diesen Kompatibilitätsmodus ist kein neuer TOML-Schalter erforderlich. Die vorhandene
Konfiguration bleibt erhalten. Die lokale Gruppen-Hangtime verwendet wieder den Main-Ablauf und
den bestehenden Wert `cell_info.hangtime_secs` beziehungsweise dessen Main-Standardwert.
