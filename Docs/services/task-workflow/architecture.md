# Architektur

```text
WAP/XHTML/WML, WebUI, REST
          │
          ▼
Task Workflow (8280)
  ├─ Task-State-Machine
  ├─ Vorlagen/Formulardaten
  ├─ Persistenz/Audit
  ├─ MQTT netcore-event-v1
  └─ SDS Router → Funkgeräte/Gruppen
```

Der Task Workflow besitzt die fachliche Aufgabe. Der SDS Router übernimmt ausschließlich Transport, Store-and-forward und Zustellung über die zuständige TBS. Das WAP-Frontend ist absichtlich klein und serverseitig gerendert.
