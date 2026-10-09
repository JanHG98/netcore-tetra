# OPEN-LAB Smoke Test

```bash
systemctl status asterisk netcore-sip-switch --no-pager --full
ss -lntup | grep -E ':8300|:5060|:10000'
curl -fsS http://IP-DES-SIP-SWITCH:8300/api/v1/status | python3 -m json.tool
curl -fsS 'http://IP-DES-SIP-SWITCH:8300/api/v1/resolve?direction=inbound&number=4010001&check_contact=false' | python3 -m json.tool
asterisk -rx 'pjsip show endpoints'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'dialplan show netcore-from-pbx'
```
