# OPEN-LAB Smoke Test

```bash
curl -fsS http://127.0.0.1:8280/health/live
curl -fsS http://127.0.0.1:8280/api/v1/templates
curl -fsS -X POST -H 'Content-Type: application/json' \
  -d '{"template_id":"technical_fault","title":"TBS pruefen","assigned_gssi":15201,"form_data":{"asset":"TBS-01","fault":"VSWR"}}' \
  http://127.0.0.1:8280/api/v1/tasks
curl -fsS http://127.0.0.1:8280/x?issi=4010001
curl -fsS http://127.0.0.1:8280/w?issi=4010001
```
