# OPEN LAB

Der Dienst besitzt in Phase 3 absichtlich keine Authentisierung und keine
Transportverschlüsselung. Jeder Client mit Netzwerkkontakt kann:

- Status und Events lesen;
- Polling und Reconnect auslösen;
- Testnachrichten unterhalb des NetCore-Präfixes veröffentlichen;
- MQTT-Commands einspeisen, die protokolliert, aber nicht ausgeführt werden.

Der lokale Mosquitto-Broker akzeptiert anonyme Clients. Diese Konfiguration darf
nicht ins Internet, in ein Gastnetz oder in ein fremd administriertes VLAN
exponiert werden.

Die harte Schutzgrenze der Phase 3 ist `execute_commands = false`. Der Dienst
verweigert den Start, wenn diese Option aktiviert wird.
