# OPEN-LAB-Modus Phase 5

Der Transport ist absichtlich offen: kein Login, keine Tokens, keine MQTT-Credentials und kein TLS. Der Betrieb gehört ausschließlich in ein isoliertes Testnetz.

Trotzdem bleiben Aktorpfade geschlossen:

- Default Deny;
- retained Commands gesperrt;
- virtuelle Ziele nur mit `lab-`;
- Home-Assistant-Egress aus;
- Homematic-Schreibzugriffe aus;
- direkte CCU-Datenpunkte explizit statt automatischer Vollinventur.
