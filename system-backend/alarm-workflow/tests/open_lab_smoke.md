# OPEN-LAB-Smoke-Test

```bash
curl -fsS -X POST -H 'Content-Type: application/json' \
  -d '{"title":"Testalarm","severity":"warning","subject_id":"rack-test","recipients":["technik-gruppe"]}' \
  http://ALARM-WORKFLOW-IP:8270/api/v1/alarms | python3 -m json.tool

curl -fsS http://ALARM-WORKFLOW-IP:8270/api/v1/alarms?active=true | python3 -m json.tool
```

ACK über API:

```bash
curl -fsS -X POST -H 'Content-Type: application/json' \
  -d '{"actor":"smoke-test","note":"Quittiert"}' \
  http://ALARM-WORKFLOW-IP:8270/api/v1/alarms/ALARM-ID/ack | python3 -m json.tool
```
