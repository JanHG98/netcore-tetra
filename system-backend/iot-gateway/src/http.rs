use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::config::IotGatewayConfig;
use crate::homematic::HomematicControl;
use crate::model::{ActionResult, TestPublishInput};
use crate::mqtt::MqttControl;
use crate::poller::PollControl;
use crate::state::SharedGateway;

pub fn spawn_http_server(
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("IoT Gateway WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config = config.clone();
                    let state = state.clone();
                    let poll_control = poll_control.clone();
                    let mqtt_control = mqtt_control.clone();
                    let homematic_control = homematic_control.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(
                            stream,
                            config,
                            state,
                            poll_control,
                            mqtt_control,
                            homematic_control,
                        ) {
                            tracing::warn!("IoT Gateway HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("IoT Gateway HTTP accept failed: {}", error),
            }
        }
    }))
}

struct HttpRequest {
    method: String,
    path: String,
    query: HashMap<String, String>,
    body: Vec<u8>,
}

struct HttpResponse {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

fn handle_connection(
    mut stream: TcpStream,
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?;
    let response = route(
        request,
        config,
        state,
        poll_control,
        mqtt_control,
        homematic_control,
    );
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

fn route(
    request: HttpRequest,
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    if request.method == "GET" {
        if let Some(raw_id) = request.path.strip_prefix("/api/v1/commands/") {
            return match Uuid::parse_str(raw_id) {
                Ok(command_id) => match state.command(command_id) {
                    Some(record) => json_response(200, &record),
                    None => json_response(404, &json!({"error":"command not found"})),
                },
                Err(error) => json_response(400, &json!({"error":format!("invalid command UUID: {error}")})),
            };
        }
    }
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(INDEX_HTML),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = state.status();
            let ready = status.mqtt_connected
                && status.sources_enabled > 0
                && status.sources_healthy == status.sources_enabled;
            let code = if ready { 200 } else { 503 };
            json_response(code, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &state.status()),
        ("GET", "/api/v1/sources") => json_response(200, &state.sources()),
        ("GET", "/api/v1/topics") => json_response(200, &state.topic_registry()),
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/api/v1/events") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.recent_events(limit))
        }
        ("GET", "/api/v1/commands") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.commands(limit))
        }
        ("GET", "/api/v1/policies") => json_response(200, &state.command_policies()),
        ("GET", "/api/v1/virtual-devices") => json_response(200, &state.virtual_devices()),
        ("GET", "/api/v1/home-assistant") => {
            json_response(200, &state.home_assistant_status())
        }
        ("GET", "/api/v1/home-assistant/entities") => {
            json_response(200, &state.external_entities())
        }
        ("GET", "/api/v1/homematic/datapoints") => {
            json_response(200, &state.homematic_datapoints())
        }
        ("GET", "/api/v1/outbox") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.outbox_entries(limit))
        }
        ("POST", "/api/v1/actions/poll-now") => match poll_control.poll_now() {
            Ok(()) => json_response(
                202,
                &ActionResult {
                    accepted: true,
                    message: "event poll requested".to_string(),
                },
            ),
            Err(error) => json_response(503, &json!({"error":error})),
        },
        ("POST", "/api/v1/actions/reconnect") => {
            mqtt_control.reconnect();
            json_response(
                202,
                &ActionResult {
                    accepted: true,
                    message: "MQTT reconnect requested".to_string(),
                },
            )
        }
        ("POST", "/api/v1/actions/home-assistant-discovery") => {
            match state.enqueue_home_assistant_discovery("http_action") {
                Ok(messages) => json_response(
                    202,
                    &json!({"accepted":true,"messages_enqueued":messages}),
                ),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/actions/homematic-poll-now") => {
            match homematic_control {
                Some(control) => match control.poll_now() {
                    Ok(()) => json_response(
                        202,
                        &ActionResult {
                            accepted: true,
                            message: "Homematic poll requested".to_string(),
                        },
                    ),
                    Err(error) => json_response(503, &json!({"error":error})),
                },
                None => json_response(
                    409,
                    &json!({"error":"Homematic CCU XML-RPC worker is not enabled"}),
                ),
            }
        }
        ("POST", "/api/v1/test/homeassistant-state") => {
            if request.body.is_empty() {
                return json_response(
                    400,
                    &json!({"error":"Home Assistant state JSON body is required"}),
                );
            }
            match state.ingest_home_assistant_state(
                config.home_assistant_state_ingress_topic(),
                request.body,
            ) {
                Ok(update) => json_response(202, &update),
                Err(error) => json_response(400, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/test/command") => {
            if request.body.is_empty() {
                return json_response(400, &json!({"error":"netcore-command-v1 JSON body is required"}));
            }
            let topic = format!(
                "{}/commands/http-test",
                config.mqtt.topic_prefix.trim_matches('/')
            );
            match state.process_command(topic, request.body, 0, false) {
                Ok(record) => json_response(200, &record),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/test/publish") => {
            let input = if request.body.is_empty() {
                TestPublishInput {
                    topic: None,
                    payload: json!({"message":"NetCore IoT Gateway OPEN LAB test"}),
                    retain: false,
                    qos: None,
                }
            } else {
                match serde_json::from_slice::<TestPublishInput>(&request.body) {
                    Ok(value) => value,
                    Err(error) => {
                        return json_response(400, &json!({"error":format!("invalid JSON: {error}")}))
                    }
                }
            };
            let topic = input.topic.unwrap_or_else(|| {
                format!(
                    "{}/test/manual",
                    config.mqtt.topic_prefix.trim_matches('/')
                )
            });
            let payload = if input.payload.is_null() {
                json!({"message":"NetCore IoT Gateway OPEN LAB test"}).to_string()
            } else {
                input.payload.to_string()
            };
            match state.enqueue_manual_message(
                topic,
                payload,
                input.qos.unwrap_or(config.mqtt.qos),
                input.retain,
            ) {
                Ok(message) => json_response(202, &message),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            state.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

fn openapi() -> serde_json::Value {
    json!({
        "openapi":"3.0.3",
        "info":{
            "title":"NetCore IoT Gateway",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"Phase 5 MQTT bridge with Home Assistant discovery, Homematic IP adapters and policy-controlled commands in OPEN LAB mode."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/sources":{"get":{}},
            "/api/v1/topics":{"get":{}},
            "/api/v1/events":{"get":{}},
            "/api/v1/commands":{"get":{}},
            "/api/v1/commands/{command_id}":{"get":{}},
            "/api/v1/policies":{"get":{}},
            "/api/v1/virtual-devices":{"get":{}},
            "/api/v1/home-assistant":{"get":{}},
            "/api/v1/home-assistant/entities":{"get":{}},
            "/api/v1/homematic/datapoints":{"get":{}},
            "/api/v1/outbox":{"get":{}},
            "/api/v1/actions/poll-now":{"post":{}},
            "/api/v1/actions/reconnect":{"post":{}},
            "/api/v1/actions/home-assistant-discovery":{"post":{}},
            "/api/v1/actions/homematic-poll-now":{"post":{}},
            "/api/v1/test/homeassistant-state":{"post":{}},
            "/api/v1/test/command":{"post":{}},
            "/api/v1/test/publish":{"post":{}},
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}},
            "/metrics":{"get":{}}
        }
    })
}

fn query_limit(query: &HashMap<String, String>, default: usize, maximum: usize) -> usize {
    query
        .get("limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
        .min(maximum)
}

fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        body: serde_json::to_vec_pretty(value).unwrap_or_else(|error| {
            format!("{{\"error\":\"JSON serialization failed: {error}\"}}").into_bytes()
        }),
    }
}

fn html(value: &'static str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        body: value.as_bytes().to_vec(),
    }
}

fn text(content_type: &'static str, value: String) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        body: value.into_bytes(),
    }
}

fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        body: Vec::new(),
    }
}

fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .map_err(|error| error.to_string())?;
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end = loop {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("request read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before HTTP headers were complete".to_string());
        }
        buffer.extend_from_slice(&chunk[..read]);
        if buffer.len() > 65_536 + max_body_bytes {
            return Err("HTTP request is too large".to_string());
        }
        if let Some(position) = find_subslice(&buffer, b"\r\n\r\n") {
            break position + 4;
        }
    };

    let headers = std::str::from_utf8(&buffer[..header_end])
        .map_err(|_| "HTTP headers are not valid UTF-8".to_string())?;
    let mut lines = headers.split("\r\n");
    let request_line = lines.next().ok_or_else(|| "missing HTTP request line".to_string())?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .ok_or_else(|| "missing HTTP method".to_string())?
        .to_string();
    let target = request_parts
        .next()
        .ok_or_else(|| "missing HTTP target".to_string())?;
    let (path, query) = parse_path_and_query(target);
    let content_length = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    if content_length > max_body_bytes {
        return Err("HTTP body exceeds configured limit".to_string());
    }

    let mut body = buffer[header_end..].to_vec();
    while body.len() < content_length {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("body read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before HTTP body was complete".to_string());
        }
        body.extend_from_slice(&chunk[..read]);
        if body.len() > max_body_bytes {
            return Err("HTTP body exceeds configured limit".to_string());
        }
    }
    body.truncate(content_length);
    Ok(HttpRequest {
        method,
        path,
        query,
        body,
    })
}

fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nX-NetCore-Security-Mode: open-lab\r\nConnection: close\r\n\r\n",
        response.status,
        reason_phrase(response.status),
        response.content_type,
        response.body.len(),
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn parse_path_and_query(raw: &str) -> (String, HashMap<String, String>) {
    let mut parts = raw.splitn(2, '?');
    let path = parts.next().unwrap_or(raw).to_string();
    let query = parts
        .next()
        .map(|query| {
            query
                .split('&')
                .filter(|pair| !pair.is_empty())
                .map(|pair| {
                    let mut fields = pair.splitn(2, '=');
                    (
                        fields
                            .next()
                            .unwrap_or_default()
                            .replace('+', " ")
                            .replace("%20", " "),
                        fields
                            .next()
                            .unwrap_or("true")
                            .replace('+', " ")
                            .replace("%20", " "),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    (path, query)
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    }
}

const INDEX_HTML: &str = r##"<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore IoT Gateway</title>
<style>
:root{color-scheme:dark;--bg:#08111f;--panel:#101d31;--line:#263a59;--text:#edf4ff;--muted:#93a8c8;--ok:#4ade80;--warn:#facc15;--bad:#fb7185;--accent:#38bdf8}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font-family:Inter,system-ui,sans-serif}.wrap{max-width:1700px;margin:auto;padding:20px}.lab{background:#7f1d1d;border:2px solid var(--bad);padding:14px 18px;border-radius:12px;font-weight:850}.sub{color:var(--muted);margin-top:-10px}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(165px,1fr));gap:12px;margin:18px 0}.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:16px}.value{font-size:1.7rem;font-weight:850}.label,.small{color:var(--muted)}.panel{margin-top:14px;overflow:auto}table{width:100%;border-collapse:collapse;min-width:900px}th,td{text-align:left;padding:9px;border-bottom:1px solid var(--line);vertical-align:top}button{border:0;border-radius:8px;padding:9px 12px;background:var(--accent);font-weight:850;cursor:pointer;margin:3px}.danger{background:var(--bad)}.safe{background:var(--ok)}.pill{padding:3px 8px;border-radius:99px;font-size:.8rem;font-weight:800}.online,.succeeded{background:#14532d}.offline,.rejected,.failed{background:#4c0519}.warn,.duplicate,.accepted,.executing{background:#713f12}pre{white-space:pre-wrap;font-size:.82rem;color:#cbd9ef;max-height:380px;overflow:auto}code{color:#bae6fd}.toolbar{display:flex;flex-wrap:wrap;gap:5px}.mono{font-family:ui-monospace,SFMono-Regular,Consolas,monospace;font-size:.83rem}.split{display:grid;grid-template-columns:repeat(auto-fit,minmax(450px,1fr));gap:14px}.notice{padding:10px;border-left:4px solid var(--warn);background:#422006;border-radius:8px}
</style></head><body><div class="wrap">
<div class="lab">⚠ OPEN LAB: KEIN LOGIN, KEINE TOKENS, KEIN TLS. MQTT ist anonym. Home-Assistant-Discovery und Zustandsimport sind aktiv; reale Home-Assistant-/Homematic-Schreibzugriffe bleiben standardmäßig gesperrt.</div>
<h1>NetCore IoT Gateway – Phase 5</h1><p class="sub">NetCore Events · MQTT · Command/Ack/Policy · Home Assistant MQTT Discovery · Homematic IP via Home Assistant oder optional CCU XML-RPC</p>
<div class="toolbar"><button onclick="post('/api/v1/actions/poll-now')">Events pollen</button><button onclick="post('/api/v1/actions/reconnect')">MQTT neu verbinden</button><button onclick="post('/api/v1/actions/home-assistant-discovery')">Discovery neu senden</button><button onclick="post('/api/v1/actions/homematic-poll-now')">CCU jetzt pollen</button><button onclick="testRelay(true)">Lab-Relais EIN</button><button onclick="testRelay(false)">Lab-Relais AUS</button><button onclick="testLight()">Lab-Licht 42 %</button></div>
<div id="cards" class="cards"></div>
<div class="split">
<div class="panel"><h2>Home Assistant</h2><pre id="ha"></pre><div class="notice">Für einen Homematic IP Access Point werden ausgewählte Home-Assistant-Entitäten per MQTT an den State-Ingress gespiegelt. Direkte CCU/RaspberryMatic-Abfragen laufen optional über XML-RPC.</div></div>
<div class="panel"><h2>Homematic-Datenpunkte</h2><table><thead><tr><th>Status</th><th>ID / Name</th><th>Adresse</th><th>Wert</th><th>Schreibbar</th><th>Fehler</th></tr></thead><tbody id="homematic"></tbody></table></div>
</div>
<div class="panel"><h2>Von Home Assistant importierte Entitäten</h2><table><thead><tr><th>Entität</th><th>Zustand</th><th>Beobachtet</th><th>Empfangen</th><th>Attribute</th></tr></thead><tbody id="external"></tbody></table></div>
<div class="panel"><h2>Command Policies – Default Deny</h2><table><thead><tr><th>Aktiv</th><th>Effekt</th><th>ID</th><th>Command Types</th><th>Target Types</th><th>Target Prefix</th></tr></thead><tbody id="policies"></tbody></table></div>
<div class="panel"><h2>Virtuelle OPEN-LAB-Geräte</h2><table><thead><tr><th>Typ</th><th>ID</th><th>Zustand</th><th>Aktualisiert</th><th>Command</th></tr></thead><tbody id="devices"></tbody></table></div>
<div class="panel"><h2>Commands und terminale Acks</h2><table><thead><tr><th>Zeit</th><th>Status</th><th>Command</th><th>Ziel</th><th>Policy / Grund</th><th>Ergebnis</th></tr></thead><tbody id="commands"></tbody></table></div>
<div class="panel"><h2>Event-Quellen</h2><table><thead><tr><th>Status</th><th>Quelle</th><th>URL</th><th>Letzter Erfolg</th><th>Zähler</th><th>Fehler</th></tr></thead><tbody id="sources"></tbody></table></div>
<div class="panel"><h2>Topic Registry</h2><pre id="topics"></pre></div>
<div class="panel"><h2>Persistente MQTT-Outbox</h2><table><thead><tr><th>Zeit</th><th>Art</th><th>Topic</th><th>QoS</th><th>Retain</th><th>Datei</th></tr></thead><tbody id="outbox"></tbody></table></div>
<div class="panel"><h2>Gateway-Ereignisse</h2><pre id="events"></pre></div>
</div><script>
const esc=v=>String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
async function getj(u){const r=await fetch(u);const t=await r.text();if(!r.ok)throw new Error(t);return t?JSON.parse(t):{}}
async function post(u,b){const r=await fetch(u,{method:'POST',headers:{'Content-Type':'application/json'},body:b?JSON.stringify(b):''});const t=await r.text();if(!r.ok)throw new Error(t);await refresh();return t?JSON.parse(t):{}}
const card=(l,v)=>`<div class="card"><div class="value">${esc(v)}</div><div class="label">${esc(l)}</div></div>`;
function command(type,targetType,targetId,payload,dryRun=false){const now=new Date();const expires=new Date(now.getTime()+30000);return {schema:'netcore-command-v1',command_id:crypto.randomUUID(),command_type:type,source:{service:'iot-gateway-webui',instance:'browser-openlab',actor:'webui-user'},requested_at:now.toISOString(),expires_at:expires.toISOString(),target:{type:targetType,id:targetId},payload,dry_run:dryRun,idempotency_key:crypto.randomUUID(),labels:{environment:'open_lab'}}}
async function runCommand(c){try{const r=await post('/api/v1/test/command',c);alert(`${r.status}: ${r.message}`)}catch(e){alert(e.message)}}
const testRelay=state=>runCommand(command('virtual.relay.set','virtual_relay','lab-relay-01',{state}));
const testLight=()=>runCommand(command('virtual.light.set','virtual_light','lab-light-01',{on:true,brightness:42}));
async function refresh(){try{const [s,src,top,out,cmd,ev,pol,dev,ha,external,hm]=await Promise.all([getj('/api/v1/status'),getj('/api/v1/sources'),getj('/api/v1/topics'),getj('/api/v1/outbox?limit=100'),getj('/api/v1/commands?limit=100'),getj('/api/v1/events?limit=100'),getj('/api/v1/policies'),getj('/api/v1/virtual-devices'),getj('/api/v1/home-assistant'),getj('/api/v1/home-assistant/entities'),getj('/api/v1/homematic/datapoints')]);
document.querySelector('#cards').innerHTML=[card('MQTT',s.mqtt_connected?'ONLINE':'OFFLINE'),card('Quellen gesund',`${s.sources_healthy}/${s.sources_enabled}`),card('Outbox',s.outbox_pending),card('Commands',s.commands_received),card('Ausgeführt',s.commands_executed),card('Home Assistant',s.home_assistant_enabled?'AKTIV':'AUS'),card('Discovery-Läufe',s.home_assistant_discovery_runs),card('HA-Entitäten',s.home_assistant_external_entities),card('Homematic',s.homematic_enabled?s.homematic_mode:'AUS'),card('HmIP gesund',`${s.homematic_datapoints_healthy}/${s.homematic_datapoints_configured}`)].join('');
document.querySelector('#ha').textContent=JSON.stringify(ha,null,2);
document.querySelector('#external').innerHTML=external.map(x=>`<tr><td><b>${esc(x.entity_id)}</b></td><td>${esc(x.state)}</td><td>${esc(x.observed_at)}</td><td>${esc(x.received_at)}</td><td><pre>${esc(JSON.stringify(x.attributes))}</pre></td></tr>`).join('')||'<tr><td colspan="5">Noch keine Home-Assistant-Zustände importiert.</td></tr>';
document.querySelector('#homematic').innerHTML=hm.map(x=>`<tr><td><span class="pill ${x.healthy?'online':'offline'}">${x.healthy?'OK':'FEHLER'}</span></td><td><b>${esc(x.id)}</b><br>${esc(x.name)}</td><td class="mono">${esc(x.address)} / ${esc(x.parameter)}</td><td><pre>${esc(JSON.stringify(x.value))}</pre></td><td>${x.writable?'ja':'nein'}</td><td>${esc(x.last_error||'-')}</td></tr>`).join('')||'<tr><td colspan="6">Keine direkten CCU-Datenpunkte konfiguriert oder noch nicht gepollt.</td></tr>';
document.querySelector('#policies').innerHTML=pol.map(x=>`<tr><td>${x.enabled?'ja':'nein'}</td><td><span class="pill ${x.effect==='allow'?'online':'offline'}">${esc(x.effect.toUpperCase())}</span></td><td><b>${esc(x.id)}</b></td><td class="mono">${esc(x.command_types.join(', '))}</td><td class="mono">${esc(x.target_types.join(', '))}</td><td class="mono">${esc(x.target_prefixes.join(', ')||'*')}</td></tr>`).join('')||'<tr><td colspan="6">Keine Policy – Default Deny greift.</td></tr>';
document.querySelector('#devices').innerHTML=dev.map(x=>`<tr><td>${esc(x.device_type)}</td><td><b>${esc(x.id)}</b></td><td><pre>${esc(JSON.stringify(x.state))}</pre></td><td>${esc(x.updated_at)}</td><td class="mono">${esc(x.command_id)}</td></tr>`).join('')||'<tr><td colspan="5">Noch keine virtuellen Geräte.</td></tr>';
document.querySelector('#commands').innerHTML=cmd.map(x=>{const c=x.command||{};const t=c.target||{};return `<tr><td>${esc(x.completed_at||x.received_at)}</td><td><span class="pill ${esc(x.status)}">${esc(x.status)}</span></td><td><b>${esc(c.command_type||'invalid')}</b><br><span class="mono">${esc(x.command_id)}</span></td><td>${esc(t.type||'-')}<br><b>${esc(t.id||'-')}</b></td><td>${esc(x.policy_id||'-')}<br>${esc(x.reason_code||'-')}</td><td><pre>${esc(JSON.stringify(x.result))}</pre></td></tr>`}).join('')||'<tr><td colspan="6">Noch keine Commands.</td></tr>';
document.querySelector('#sources').innerHTML=src.map(x=>`<tr><td><span class="pill ${x.healthy?'online':'offline'}">${x.healthy?'ONLINE':'FEHLER'}</span></td><td><b>${esc(x.id)}</b></td><td class="mono">${esc(x.url)}</td><td>${esc(x.last_success_at||'-')}</td><td>gesehen ${x.events_seen}<br>queued ${x.events_enqueued}<br>doppelt ${x.duplicates_skipped}</td><td>${esc(x.last_error||'-')}</td></tr>`).join('')||'<tr><td colspan="6">Keine Quellen konfiguriert.</td></tr>';
document.querySelector('#topics').textContent=JSON.stringify(top,null,2);document.querySelector('#outbox').innerHTML=out.map(x=>`<tr><td>${esc(x.created_at||'-')}</td><td>${esc(x.kind||'unlesbar')}</td><td class="mono">${esc(x.topic||x.error||'-')}</td><td>${esc(x.qos??'-')}</td><td>${esc(x.retain??'-')}</td><td class="mono">${esc(x.file_name)}</td></tr>`).join('')||'<tr><td colspan="6">Outbox leer.</td></tr>';document.querySelector('#events').textContent=ev.map(x=>`${x.timestamp} ${x.kind} ${JSON.stringify(x.detail)}`).join('\n')}catch(e){document.querySelector('#events').textContent='Fehler: '+e.message}}
refresh();setInterval(refresh,3000);
</script></body></html>"##;
