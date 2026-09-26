# Netzwerk, Ports und Protokolle

**Beispielwerte, keine globalen Defaults:** Die folgenden Angaben stammen aus [`deploy/open-lab/inventory.example.toml`](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/inventory.example.toml), der [sanitisierten TBS-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml) und den jeweiligen Dienstvorlagen. Auf einem anderen Host kann derselbe Port gleichzeitig genutzt werden. Vor einer Freigabe immer die **gerenderte `/etc/netcore/*.toml` und `ss` am Zielhost** lesen.

## Management/API je Host

| Dienst | TCP-Port im Inventory | Dienst | TCP-Port im Inventory |
|---|---:|---|---:|
| Node Gateway | 8080 | Mobility Core | 8090 |
| Subscriber Core | 8100 | Group Core | 8110 |
| Call Control | 8120 | Media Switch | 8130 |
| Recorder | 8140 | SDS Router | 8150 |
| Packet Core | 8160 | IP Gateway | 8170 |
| Security Core | 8180 | KMF | 8190 |
| Transit | 8200 | Observability | 8210 |
| Application Gateway | 8220 | Media Library | 8230 |
| IoT Gateway | 8240 | Hardware Gateway | 8250 |
| RF Monitor | 8260 | Alarm Workflow | 8270 |
| Task Workflow | 8280 | Asset Management | 8290 |
| SIP Switch | 8300 | Control Room | 9010 |

**Zusätzlich:** TBS-Dashboard `8080` auf der TBS, Provisioning Core `8125`, Directory `8095`, Piper `5005`. Node Gateway `8080` und TBS-Dashboard `8080` dürfen nur auf **verschiedenen Hosts/Adressen** parallel binden. Dienstzweck und Quellpfad: [[Dienstkatalog]].

## Daten- und Steuerwege

| Verbindung | Protokoll/Beispiel | Prüffrage |
|---|---|---|
| TBS → Node Gateway | WebSocket auf `ws://<gateway>:8080/ws/node` | stimmen `host`, `port`, `endpoint_path`, Node-ID und Security-Modus? |
| Backend → Node Gateway | WebSocket `/ws/backend` sowie HTTP APIs | meldet der Dienst sich mit gültiger Vertragsversion? |
| Fach-WebUI/API | HTTP/TCP am jeweiligen Managementport | antworten `/health/live`, `/health/ready`, `/api/v1/status`? `503` bei Ready kann eine echte Abhängigkeit anzeigen. |
| IoT Gateway → Broker | MQTT/TCP, Vorlage `1883` | stimmen Host, Topic-Prefix, QoS, ACL und Retained-Status? |
| TBS → Brew-Peer | WebSocket/TCP, Vorlagenwert `8081` | ist Peer erreichbar und unterstützt er die freigeschalteten Features? |
| TBS → Directory/Piper | HTTP/TCP, Beispiele `8095`/`5005` | Names/Status bzw. Sprachsynthese getrennt testen. |
| IP Gateway | TUN, DNS/UDP, HTTP-WAP-Testserver | TUN-Passthrough, Route, NAT und Firewall getrennt prüfen. |

## SIP und Medien

| Rolle | Beispielwert | Bedeutung |
|---|---|---|
| Zentraler SIP Switch | `5060` UDP/TCP | SIP-Listener des zentralen Asterisk, getrennt vom HTTP-Management `8300` |
| Zentraler SIP Switch | `10000–20000` UDP | RTP-Bereich aus der Beispiel-TOML, standortabhängig |
| Native TBS-SIP-Bridge | `5062` UDP | Beispiel der TBS-Konfiguration; lokalen Asterisk separat prüfen |
| Native TBS-Bridge | `30000–30100` UDP | RTP-Beispielbereich |
| Vorhandene PBX | `5060` im Beispiel | eigener Host/Trunk; kein NetCore-Managementport |

SIP-Registrierung und `180 Ringing` beweisen keinen bidirektionalen RTP-Durchsatz. Eine funktionierende Verbindung benötigt zusätzlich passenden Codec, NAT-/Firewall-Regeln, Rufannahme und Audio in beide Richtungen. [[SIP-und-Brew]]

## Prüfen statt raten

```bash
sudo ss -lntup
systemctl status netcore-node-gateway.service --no-pager
curl -fsS http://<GATEWAY-IP>:8080/health/live
curl -sS -w '\nHTTP %{http_code}\n' http://<GATEWAY-IP>:8080/health/ready
```

Die konkreten Ziele stehen im eigenen `inventory.toml`, in den gerenderten Dienstkonfigurationen und in der TBS-Konfiguration. Ein Verwaltungsport gehört **nicht** ungeprüft an einen öffentlichen Router. [[Security-and-Operations]]
