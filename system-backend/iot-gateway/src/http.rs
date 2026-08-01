use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::IotGatewayConfig;
use crate::model::{ActionResult, TestPublishInput};
use crate::mqtt::MqttControl;
use crate::poller::PollControl;
use crate::state::SharedGateway;

pub fn spawn_http_server(
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
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
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(
                            stream,
                            config,
                            state,
                            poll_control,
                            mqtt_control,
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
) -> Result<(), String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?;
    let response = route(request, config, state, poll_control, mqtt_control);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

fn route(
    request: HttpRequest,
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
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
            "description":"Phase 3 MQTT bridge in OPEN LAB mode. No login, no tokens and no TLS. MQTT commands are observed but never executed."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/sources":{"get":{}},
            "/api/v1/topics":{"get":{}},
            "/api/v1/events":{"get":{}},
            "/api/v1/commands":{"get":{}},
            "/api/v1/outbox":{"get":{}},
            "/api/v1/actions/poll-now":{"post":{}},
            "/api/v1/actions/reconnect":{"post":{}},
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

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore IoT Gateway</title>
<style>
:root{color-scheme:dark;--bg:#08111f;--panel:#101d31;--line:#263a59;--text:#edf4ff;--muted:#93a8c8;--ok:#4ade80;--warn:#facc15;--bad:#fb7185;--accent:#38bdf8}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font-family:Inter,system-ui,sans-serif}.wrap{max-width:1550px;margin:auto;padding:20px}.lab{background:#7f1d1d;border:2px solid var(--bad);padding:14px 18px;border-radius:12px;font-weight:850}.sub{color:var(--muted);margin-top:-10px}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(175px,1fr));gap:12px;margin:18px 0}.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:16px}.value{font-size:1.85rem;font-weight:850}.label,.small{color:var(--muted)}.panel{margin-top:14px;overflow:auto}table{width:100%;border-collapse:collapse;min-width:880px}th,td{text-align:left;padding:9px;border-bottom:1px solid var(--line);vertical-align:top}button{border:0;border-radius:8px;padding:9px 12px;background:var(--accent);font-weight:850;cursor:pointer;margin-right:7px}.danger{background:var(--bad)}.pill{padding:3px 8px;border-radius:99px;font-size:.8rem;font-weight:800}.online{background:#14532d}.offline{background:#4c0519}.warn{background:#713f12}pre{white-space:pre-wrap;font-size:.82rem;color:#cbd9ef;max-height:380px;overflow:auto}code{color:#bae6fd}.toolbar{display:flex;flex-wrap:wrap;gap:8px}.mono{font-family:ui-monospace,SFMono-Regular,Consolas,monospace;font-size:.83rem}
</style></head><body><div class="wrap">
<div class="lab">⚠ OPEN LAB: KEIN LOGIN, KEINE TOKENS, KEIN TLS. MQTT ist anonym. Eingehende Commands werden nur protokolliert und in Phase 3 niemals ausgeführt.</div>
<h1>NetCore IoT Gateway</h1><p class="sub">netcore-event-v1 → MQTT, persistente Outbox, Retained States und Command-Beobachtung</p>
<div class="toolbar"><button onclick="post('/api/v1/actions/poll-now')">Jetzt pollen</button><button onclick="post('/api/v1/actions/reconnect')">MQTT neu verbinden</button><button onclick="testPublish()">Testnachricht senden</button></div>
<div id="cards" class="cards"></div>
<div class="panel"><h2>Event-Quellen</h2><table><thead><tr><th>Status</th><th>Quelle</th><th>URL</th><th>Letzter Erfolg</th><th>Zähler</th><th>Fehler</th></tr></thead><tbody id="sources"></tbody></table></div>
<div class="panel"><h2>Topic Registry</h2><pre id="topics"></pre></div>
<div class="panel"><h2>Persistente MQTT-Outbox</h2><table><thead><tr><th>Zeit</th><th>Art</th><th>Topic</th><th>QoS</th><th>Retain</th><th>Datei</th></tr></thead><tbody id="outbox"></tbody></table></div>
<div class="panel"><h2>Beobachtete Commands – keine Ausführung</h2><table><thead><tr><th>Zeit</th><th>Topic</th><th>Status</th><th>Payload</th></tr></thead><tbody id="commands"></tbody></table></div>
<div class="panel"><h2>Gateway-Ereignisse</h2><pre id="events"></pre></div>
</div><script>
const esc=v=>String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
async function getj(u){const r=await fetch(u);const t=await r.text();if(!r.ok)throw new Error(t);return t?JSON.parse(t):{}}
async function post(u,b){const r=await fetch(u,{method:'POST',headers:{'Content-Type':'application/json'},body:b?JSON.stringify(b):''});const t=await r.text();if(!r.ok)throw new Error(t);await refresh();return t?JSON.parse(t):{}}
const card=(l,v)=>`<div class="card"><div class="value">${esc(v)}</div><div class="label">${esc(l)}</div></div>`;
async function testPublish(){try{await post('/api/v1/test/publish',{payload:{message:'Hallo aus der IoT-Gateway-WebUI',timestamp:new Date().toISOString()},retain:false})}catch(e){alert(e.message)}}
async function refresh(){try{const [s,src,top,out,cmd,ev]=await Promise.all([getj('/api/v1/status'),getj('/api/v1/sources'),getj('/api/v1/topics'),getj('/api/v1/outbox?limit=100'),getj('/api/v1/commands?limit=50'),getj('/api/v1/events?limit=100')]);document.querySelector('#cards').innerHTML=[card('MQTT',s.mqtt_connected?'ONLINE':'OFFLINE'),card('Quellen gesund',`${s.sources_healthy}/${s.sources_enabled}`),card('Outbox',s.outbox_pending),card('Events erkannt',s.events_seen),card('MQTT bestätigt',s.events_published),card('Duplikate',s.duplicates_skipped),card('Commands beobachtet',s.commands_observed),card('Commands ausgeführt',s.commands_executed)].join('');document.querySelector('#sources').innerHTML=src.map(x=>`<tr><td><span class="pill ${x.healthy?'online':'offline'}">${x.healthy?'ONLINE':'FEHLER'}</span></td><td><b>${esc(x.id)}</b></td><td class="mono">${esc(x.url)}</td><td>${esc(x.last_success_at||'-')}</td><td>gesehen ${x.events_seen}<br>queued ${x.events_enqueued}<br>doppelt ${x.duplicates_skipped}</td><td>${esc(x.last_error||'-')}</td></tr>`).join('')||'<tr><td colspan="6">Keine Quellen konfiguriert.</td></tr>';document.querySelector('#topics').textContent=JSON.stringify(top,null,2);document.querySelector('#outbox').innerHTML=out.map(x=>`<tr><td>${esc(x.created_at||'-')}</td><td>${esc(x.kind||'unlesbar')}</td><td class="mono">${esc(x.topic||x.error||'-')}</td><td>${esc(x.qos??'-')}</td><td>${esc(x.retain??'-')}</td><td class="mono">${esc(x.file_name)}</td></tr>`).join('')||'<tr><td colspan="6">Outbox leer.</td></tr>';document.querySelector('#commands').innerHTML=cmd.map(x=>`<tr><td>${esc(x.received_at)}</td><td class="mono">${esc(x.topic)}</td><td><span class="pill warn">NICHT AUSGEFÜHRT</span></td><td><pre>${esc(x.payload)}</pre></td></tr>`).join('')||'<tr><td colspan="4">Noch keine Commands beobachtet.</td></tr>';document.querySelector('#events').textContent=ev.map(x=>`${x.timestamp} ${x.kind} ${JSON.stringify(x.detail)}`).join('\n')}catch(e){document.querySelector('#events').textContent='Fehler: '+e.message}}
refresh();setInterval(refresh,3000);
</script></body></html>"#;
