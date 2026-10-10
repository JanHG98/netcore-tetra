# Service Contract Versioning

**Quellbezug:** gemeinsamer Vertrag und Schemas unter `system-backend/shared/`, Stand `main` (`c3ccdb4`, 09.10.2026). Phasenangaben sind Einführungshistorie; Ausführungsberechtigungen und optionale Felder richten sich nach dem konkreten Dienst.

- Aktuelle gemeinsame Hauptversion: `netcore.v1`.
- Minor-Erweiterungen müssen abwärtskompatibel und optional sein.
- Major-Änderungen erhalten einen parallelen Adapter oder Endpunkt; kein stiller In-place-Bruch.
- Wiederholbare Commands benötigen `message_id` und `idempotency_key`.
- `correlation_id` verbindet Request, Folgeevents, Audit und Antwort; `causation_id` bezeichnet den direkten Auslöser.
- Service Descriptor und Capabilities entscheiden vor der Nutzung optionaler Funktionen über Kompatibilität.
- Generische Envelopes dürfen kein Rohschlüsselmaterial und keine unredigierten Secrets transportieren.
