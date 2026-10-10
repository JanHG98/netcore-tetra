# Telefonievermittlung: Funktionsprüfung im Labor

Die Befehle auf dem SIP-Switch-LXC ausführen; `IP-DES-SIP-SWITCH` und ISSI ersetzen. Eine erfolgreiche Route oder SIP-Registrierung belegt noch keine Sprache in beiden Richtungen. Audio, Anruferanzeige, Abbruch und zentralen Ausfall separat mit Lab-Telefon und Funkgerät abnehmen.


```bash
systemctl status asterisk netcore-sip-switch --no-pager --full
ss -lntup | grep -E ':8300|:5060|:10000'
curl -fsS http://IP-DES-SIP-SWITCH:8300/api/v1/status | python3 -m json.tool
curl -fsS 'http://IP-DES-SIP-SWITCH:8300/api/v1/resolve?direction=inbound&number=4010001&check_contact=false' | python3 -m json.tool
asterisk -rx 'pjsip show endpoints'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'dialplan show netcore-from-pbx'
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/sip-switch) · [Konfigurationsvorlagen](../../../../system-backend/sip-switch/config).
