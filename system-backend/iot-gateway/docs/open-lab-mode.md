# OPEN-LAB-Modus Phase 4

Der IoT Gateway besitzt weiterhin keine Authentisierung und keine Verschlüsselung. Jeder Teilnehmer im erreichbaren Netzwerk kann MQTT-Nachrichten publizieren und Management-Endpunkte aufrufen.

Die Schutzgrenze liegt deshalb bewusst innerhalb der Fachlogik:

- `default_deny = true`;
- nur explizite Allow-Policies;
- Deny gewinnt;
- nur virtuelle Sandbox-Executor;
- Standardpolicies nur für Ziel-IDs unter `lab-`;
- keine GPIO-, Homematic-, Modbus-, Tür-, Tor- oder Stromschalt-Adapter;
- Retained Commands verboten;
- TTL und Dublettensperre verpflichtend.

Das ist kein Ersatz für spätere Authentisierung. Die Angaben `source.service`, `source.instance` und `source.actor` sind im OPEN LAB nicht beweiskräftig.
