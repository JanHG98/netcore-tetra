# Hardware, SDR und HF-Aufbau

Die TBS setzt auf einen Linux-Rechner (etwa Raspberry Pi 5), einen unterstützten SoapySDR-Treiber/SDR, eine stabile Zeitbasis, passende Filter-/Duplexer- und Antennentechnik. Ein zusätzlicher AI-HAT ist für den dokumentierten TETRA-Funkstack **keine Voraussetzung**. Der konkrete HF-Aufbau muss zu SDR, Leistung, Filter, Zulassung und Messmitteln passen.

## Signalweg und Trennung

| Abschnitt | Prüfung |
|---|---|
| SDR TX → Duplexer TX | Frequenzbereich, Dämpfung, zulässige Leistung, Stecker und Kabel |
| SDR RX ← Duplexer RX | Isolation gegen eigenen TX, Empfindlichkeit, LNA/Verstärkung und Übersteuerung |
| Duplexer ANT → Schutz/Koax/Antenne | Rückflussdämpfung, Blitzschutz/Erdung, zulässige Sendeanlage |
| Pi ↔ SDR/HAT | Spannungsversorgung, 40-Pin-Belegung, Temperatur, mechanische Höhe und Luftstrom |

Vor einem offenen Antennenbetrieb zuerst mit geeigneter Last, Messgerät und begrenzter Leistung prüfen. TX-Ausgang nie ohne spezifizierten Abschluss betreiben; fehlende Messung wird nicht durch eine grüne Dashboard-Kachel ersetzt. Arbeiten am 230-V-Teil gehören zur gesondert geprüften Hardwareplanung.

## Treiber und Center-Frequenz

```bash
SoapySDRUtil --info
SoapySDRUtil --find
SoapySDRUtil --probe="driver=<TATSÄCHLICHER-TREIBER>"
```

Die TBS verwendet `[phy_io]`, `[phy_io.soapysdr]` und `[cell_info]`. Für zwei Träger müssen Sample-Rate und TX-/RX-Center **beide** Träger samt Filterreserve abdecken. Für den früher beobachteten Laborstand waren Carrier 720/721 mit 418,000/418,025 MHz Downlink und 408,000/408,025 MHz Uplink sowie Center 418,0125/408,0125 MHz vorgesehen. Das sind **Projektbeispiele, keine allgemeine Betriebsfreigabe**. [[Dual-Carrier]] · [[Configuration]]

## HF-Inbetriebnahme in Reihenfolge

1. Passive Kette durchmessen: Kabel, Stecker, Duplexerpfade, Isolation, Last und Antenne.
2. SDR-Identität, Treiber, Abtastrate, RX-/TX-Kanäle und Gain separat dokumentieren.
3. Erst einen Träger mit Last betreiben; Spektrum, Nebenprodukte, Frequenz und Timing beobachten.
4. Uplink-Empfang bei aktivem Downlink prüfen: RX-Overruns, TX-Late-Skips und Desensibilisierung sind getrennte Fehler.
5. Funkgerät registrieren, affiliieren, SDS senden, Ruf aufbauen und **vollständig freigeben**.
6. Zweiten Träger freigeben, Passband und Last mit mehreren Zeitschlitzen erneut prüfen.

Ein RX-Buffer-Overrun oder TX-Late-Skip beim Anlauf ist von **dauerhaften** Zeitfehlern unter Last zu unterscheiden. Die Abnahme protokolliert Build, Konfiguration, SDR/Clock, Frequenzpaar, Last, Endgerät und Logs. [[Abnahme]]

## GPIO, Sensorik und geplante Leiterplatte

Die 40-Pin-Belegung hängt vom konkreten SXceiver-HAT und seiner Hardwareversion ab. Freie GPIOs erst nach **Schaltplan/Pinout-Abgleich** für I²C, Sensoren, Lüfter, LEDs oder Taster einplanen. Ein durchgeschleifter Header macht benutzte Leitungen nicht frei. LCD/TFT, Watchdog/Power-Control, Temperatur-/Spannungsüberwachung, zweite SDR-Stufe und 230-V-Netzteil sind Hardware-Ausbauziele; aus Wiki-Texten ergibt sich kein fertiger, geprüft bestückbarer PCB-Stand. [[Roadmap]]
