# Auftragsbearbeitung: Funktionsprüfung im Labor

`TASK-WORKFLOW-IP` durch den tatsächlichen Listener ersetzen. Der POST legt einen persistenten Auftrag an und kann über den SDS Router an GSSI 15201 senden; eine passende Lab-Gruppe verwenden. `issi` ist im offenen WAP-Pfad eine ungeschützte Komfortangabe.


```bash
curl -fsS http://TASK-WORKFLOW-IP:8280/health/live
curl -fsS http://TASK-WORKFLOW-IP:8280/api/v1/templates
curl -fsS -X POST -H 'Content-Type: application/json' \
  -d '{"template_id":"technical_fault","title":"TBS pruefen","assigned_gssi":15201,"form_data":{"asset":"TBS-01","fault":"VSWR"}}' \
  http://TASK-WORKFLOW-IP:8280/api/v1/tasks
curl -fsS 'http://TASK-WORKFLOW-IP:8280/x?issi=4010001'
curl -fsS 'http://TASK-WORKFLOW-IP:8280/w?issi=4010001'
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/task-workflow) · [Konfigurationsvorlagen](../../../../system-backend/task-workflow/config).
