# IoT-Anbindung: Offener Laborbetrieb

Der Transport ist absichtlich offen: kein Login, keine Tokens, keine MQTT-Credentials und kein TLS. Der Betrieb gehört ausschließlich in ein isoliertes Testnetz.

Trotzdem bleiben Aktorpfade geschlossen:

- Default Deny;
- retained Commands gesperrt;
- virtuelle Ziele nur mit `lab-`;
- Home-Assistant-Egress aus;
- Homematic-Schreibzugriffe aus;
- direkte CCU-Datenpunkte explizit statt automatischer Vollinventur.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/iot-gateway) · [Konfigurationsvorlagen](../../../system-backend/iot-gateway/config).
