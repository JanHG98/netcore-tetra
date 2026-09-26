# Dual Carrier

Dual Carrier erweitert die Basisstation um einen zweiten TETRA-Träger. Im Dashboard werden damit acht logische Zeitschlitze dargestellt: vier pro Carrier. Das ist keine Zusage für acht gleichzeitig nutzbare Sprachkanäle. Die tatsächliche Kapazität hängt von Control-Channel-Belegung, Scheduler, Reservierungen und Endgeräten ab.

## Voraussetzungen

- SDR und Treiber unterstützen die erforderliche Sample-Rate stabil.
- TX- und RX-Center-Frequenz decken beide Carrier vollständig ab.
- Haupt- und Sekundär-Carrier sind verschieden und innerhalb des zulässigen Bereichs.
- Antennen, Filter und Verstärker decken die Frequenzen ab.
- Endgeräte sind auf das Zell- und Carrier-Konzept programmiert.

## Beispiel

```toml
[phy_io.soapysdr]
sample_rate = 600000
tx_center_freq = <MITTELPUNKT-DOWNLINK-HZ>
rx_center_freq = <MITTELPUNKT-UPLINK-HZ>

[cell_info]
main_carrier = 720
secondary_carrier = 721
dual_carrier_enabled = true
```

Die Zahlen sind nur ein Schema. Die tatsächliche Nutzung muss zum genehmigten Laboraufbau passen.

## Passband-Logik

Bei zwei Carriern reicht es nicht, nur `secondary_carrier` einzutragen. Die Basisstation prüft, ob die gewählte Sample-Rate und die Center-Frequenzen beide Träger aufnehmen können. Sinnvoll ist ein Center-Frequenzpunkt zwischen den Carriern mit genügend Reserve für Filterflanken.

## Control Channel und Traffic

Der Hauptträger führt im beschriebenen Aufbau den primären Control Channel. Der zweite Träger ist zunächst eine zusätzliche Traffic-Ressource; ob und welche Control-Aufgaben er übernimmt, muss mit der **konkreten Laufzeitversion und Air-Logs** nachgewiesen werden. Ein Funkgerät muss daher nicht pauschal beide Frequenzen als zwei getrennte Kontrollkanäle programmiert bekommen. Für ein Gerät zunächst die belegte Kontrollträgerfrequenz und korrekte Netz-/Zellparameter setzen, danach Verhalten bei Traffic-Zuweisung auf den zweiten Träger testen. Endgeräte ohne parallelen Multicarrier-Zugriff können auf dem Sekundärträger nicht beliebig Random Access oder CCCH nutzen.

## Dashboard-Schalter

Das Dashboard kann Dual Carrier ein- oder ausschalten und die Carrier-Nummer in der Konfiguration erhalten. Eine Änderung führt zu einem geplanten Neustart, weil RF- und Scheduler-Strukturen neu aufgebaut werden müssen.

## Typische Fehler

### Sekundärträger bleibt leer

- Endgerät kann den zugewiesenen Traffic-Carrier nicht nutzen.
- Control-/Traffic-Zuordnung passt nicht.
- Scheduler reserviert einen Slot.
- Gruppenruf hängt noch in Hangtime oder Release.

### Endgeräte werden bei hoher Belegung abgeworfen

- Timing- oder Bufferproblem unter Last.
- Zu geringe Sample-Rate oder instabiler SDR-Durchsatz.
- fehlerhafte Ruf-Freigabe auf einem Carrier.
- ältere Geräte reagieren problematisch auf schnelle Retakes.

### Passband-Fehler beim Start

- `sample_rate` fehlt.
- Center-Frequenz liegt auf einem Carrier statt zwischen beiden.
- Carrier-Abstand ist größer als die nutzbare SDR-Bandbreite.

## Testplan

1. Einzelträger stabil betreiben.
2. Zweiten Carrier ohne Last aktivieren und Spektrum prüfen.
3. Je einen Ruf auf Haupt- und Sekundärträger testen.
4. Gruppenrufe mit Hangtime und wiederholtem PTT testen.
5. Bis zur geplanten Maximalbelegung steigern.
6. Release aller Calls und erneute Registrierung kontrollieren.
7. Logs carrier- und timeslotbezogen sichern.

[[Hardware-und-RF]] beschreibt die HF-Prüfung; [[Abnahme]] enthält den vollständigen Funk- und Integrationstest. Werte aus einem alten Log sind keine Freigabe für eine aktuelle Konfiguration.
