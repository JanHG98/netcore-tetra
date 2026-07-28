use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde_json::{Value, json};

use crate::config::ProvisioningConfig;
use crate::upstream::{self, UpstreamResponse};

struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

struct HttpResponse {
    status: u16,
    content_type: String,
    body: Vec<u8>,
}

pub fn serve(config: ProvisioningConfig) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Provisioning Core WebUI/API listening on http://{}", config.server.bind);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = config.clone();
                thread::spawn(move || {
                    if let Err(error) = handle_connection(stream, &config) {
                        tracing::warn!("HTTP connection failed: {error}");
                    }
                });
            }
            Err(error) => tracing::warn!("HTTP accept failed: {error}"),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, config: &ProvisioningConfig) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

fn route(request: HttpRequest, config: &ProvisioningConfig) -> HttpResponse {
    match (request.method.as_str(), request.path.as_str()) {
        ("OPTIONS", _) => empty(204),
        ("GET", "/") => html(INDEX_HTML),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live","service":"provisioning-core"})),
        ("GET", "/health/ready") | ("GET", "/api/v1/status") => status_response(config),
        ("GET", "/api/v1/dashboard") => dashboard_response(config),
        ("POST", "/api/v1/sync") => sync_response(config),
        ("DELETE", path) if path.starts_with("/api/v1/subscribers/") => {
            delete_subscriber_with_memberships(path, config)
        }
        ("DELETE", path) if path.starts_with("/api/v1/groups/") => {
            delete_group_with_memberships(path, config)
        }
        (_, path) if path == "/api/v1/subscribers" || path.starts_with("/api/v1/subscribers/") => {
            proxy(&config.upstream.subscriber_core, &request, config)
        }
        (_, path) if path == "/api/v1/groups"
            || path.starts_with("/api/v1/groups/")
            || path == "/api/v1/memberships"
            || path.starts_with("/api/v1/memberships/") =>
        {
            proxy(&config.upstream.group_core, &request, config)
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

fn status_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber = upstream_json(&config.upstream.subscriber_core, "/api/v1/status", config);
    let groups = upstream_json(&config.upstream.group_core, "/api/v1/status", config);
    let ready = subscriber.as_ref().is_ok_and(|(status, _)| *status < 500)
        && groups.as_ref().is_ok_and(|(status, _)| *status < 500);
    json_response(
        if ready { 200 } else { 503 },
        &json!({
            "service":"provisioning-core",
            "security_mode":"open_lab",
            "ready":ready,
            "subscriber_core": result_snapshot(subscriber),
            "group_core": result_snapshot(groups),
        }),
    )
}

fn dashboard_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber_status = upstream_json(&config.upstream.subscriber_core, "/api/v1/status", config);
    let group_status = upstream_json(&config.upstream.group_core, "/api/v1/status", config);
    let subscribers = upstream_json(&config.upstream.subscriber_core, "/api/v1/subscribers", config);
    let observed = upstream_json(&config.upstream.subscriber_core, "/api/v1/observed", config);
    let groups = upstream_json(&config.upstream.group_core, "/api/v1/groups", config);
    let memberships = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config);

    let failures = [
        ("subscriber_status", &subscriber_status),
        ("group_status", &group_status),
        ("subscribers", &subscribers),
        ("observed", &observed),
        ("groups", &groups),
        ("memberships", &memberships),
    ]
    .into_iter()
    .filter_map(|(name, result)| result.as_ref().err().map(|error| json!({"source":name,"error":error})))
    .collect::<Vec<_>>();

    json_response(
        if failures.is_empty() { 200 } else { 503 },
        &json!({
            "service":"provisioning-core",
            "security_mode":"open_lab",
            "subscriber_core": value_or_null(subscriber_status),
            "group_core": value_or_null(group_status),
            "subscribers": value_or_array(subscribers),
            "observed": value_or_array(observed),
            "groups": value_or_array(groups),
            "memberships": value_or_array(memberships),
            "failures": failures,
        }),
    )
}

fn sync_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber = upstream::request(
        &config.upstream.subscriber_core,
        "POST",
        "/api/v1/sync",
        b"",
        config.timeout(),
    );
    let group = upstream::request(
        &config.upstream.group_core,
        "POST",
        "/api/v1/sync",
        b"",
        config.timeout(),
    );
    let ok = subscriber.as_ref().is_ok_and(|response| response.status < 400)
        && group.as_ref().is_ok_and(|response| response.status < 400);
    json_response(
        if ok { 202 } else { 503 },
        &json!({
            "subscriber_core": upstream_snapshot(subscriber),
            "group_core": upstream_snapshot(group),
        }),
    )
}

fn delete_subscriber_with_memberships(path: &str, config: &ProvisioningConfig) -> HttpResponse {
    let Some(issi) = path.trim_start_matches("/api/v1/subscribers/").parse::<u32>().ok() else {
        return json_response(400, &json!({"error":"invalid ISSI"}));
    };
    if let Ok((_, Value::Array(memberships))) = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config) {
        for membership in memberships {
            if membership.get("issi").and_then(Value::as_u64) == Some(issi as u64) {
                if let Some(gssi) = membership.get("gssi").and_then(Value::as_u64) {
                    let membership_path = format!("/api/v1/memberships/{issi}/{gssi}");
                    let _ = upstream::request(&config.upstream.group_core, "DELETE", &membership_path, b"", config.timeout());
                }
            }
        }
    }
    match upstream::request(&config.upstream.subscriber_core, "DELETE", path, b"", config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn delete_group_with_memberships(path: &str, config: &ProvisioningConfig) -> HttpResponse {
    let Some(gssi) = path.trim_start_matches("/api/v1/groups/").parse::<u32>().ok() else {
        return json_response(400, &json!({"error":"invalid GSSI"}));
    };
    if let Ok((_, Value::Array(memberships))) = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config) {
        for membership in memberships {
            if membership.get("gssi").and_then(Value::as_u64) == Some(gssi as u64) {
                if let Some(issi) = membership.get("issi").and_then(Value::as_u64) {
                    let membership_path = format!("/api/v1/memberships/{issi}/{gssi}");
                    let _ = upstream::request(&config.upstream.group_core, "DELETE", &membership_path, b"", config.timeout());
                }
            }
        }
    }
    match upstream::request(&config.upstream.group_core, "DELETE", path, b"", config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn proxy(base: &str, request: &HttpRequest, config: &ProvisioningConfig) -> HttpResponse {
    match upstream::request(base, &request.method, &request.path, &request.body, config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn upstream_json(base: &str, path: &str, config: &ProvisioningConfig) -> Result<(u16, Value), String> {
    let response = upstream::request(base, "GET", path, b"", config.timeout())?;
    let value = if response.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&response.body).map_err(|error| format!("invalid JSON from {base}{path}: {error}"))?
    };
    Ok((response.status, value))
}

fn result_snapshot(result: Result<(u16, Value), String>) -> Value {
    match result {
        Ok((status, value)) => json!({"reachable":true,"status":status,"data":value}),
        Err(error) => json!({"reachable":false,"error":error}),
    }
}

fn upstream_snapshot(result: Result<UpstreamResponse, String>) -> Value {
    match result {
        Ok(response) => {
            let data = serde_json::from_slice::<Value>(&response.body).unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&response.body).to_string()));
            json!({"reachable":true,"status":response.status,"data":data})
        }
        Err(error) => json!({"reachable":false,"error":error}),
    }
}

fn value_or_null(result: Result<(u16, Value), String>) -> Value {
    result.map(|(_, value)| value).unwrap_or(Value::Null)
}

fn value_or_array(result: Result<(u16, Value), String>) -> Value {
    result.map(|(_, value)| value).unwrap_or_else(|_| Value::Array(Vec::new()))
}

fn from_upstream(response: UpstreamResponse) -> HttpResponse {
    HttpResponse { status: response.status, content_type: response.content_type, body: response.body }
}

fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).map_err(|error| error.to_string())?;
    let mut data = Vec::new();
    let mut buffer = [0u8; 8192];
    let header_end;
    loop {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 { return Err("connection closed before HTTP headers".into()); }
        data.extend_from_slice(&buffer[..count]);
        if data.len() > max_body_bytes + 65_536 { return Err("request exceeds configured limit".into()); }
        if let Some(position) = data.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
    }
    let header = String::from_utf8_lossy(&data[..header_end]);
    let mut lines = header.lines();
    let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().ok_or_else(|| "missing method".to_string())?.to_string();
    let raw_path = parts.next().ok_or_else(|| "missing path".to_string())?;
    let path = raw_path.split('?').next().unwrap_or(raw_path).to_string();
    let headers = lines.filter_map(|line| line.split_once(':')).map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_string())).collect::<HashMap<_, _>>();
    let content_length = headers.get("content-length").and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
    if content_length > max_body_bytes { return Err("request body exceeds configured limit".into()); }
    while data.len() < header_end + content_length {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 { return Err("truncated request body".into()); }
        data.extend_from_slice(&buffer[..count]);
    }
    Ok(HttpRequest { method, path, body: data[header_end..header_end + content_length].to_vec() })
}

fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    let reason = match response.status {
        200 => "OK", 201 => "Created", 202 => "Accepted", 204 => "No Content",
        400 => "Bad Request", 404 => "Not Found", 405 => "Method Not Allowed",
        409 => "Conflict", 500 => "Internal Server Error", 503 => "Service Unavailable",
        _ => "OK",
    };
    write!(stream,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS\r\nConnection: close\r\n\r\n",
        response.status, reason, response.content_type, response.body.len())?;
    stream.write_all(&response.body)
}

fn json_response(status: u16, value: &Value) -> HttpResponse {
    HttpResponse { status, content_type: "application/json; charset=utf-8".into(), body: serde_json::to_vec_pretty(value).unwrap_or_default() }
}

fn html(value: &'static str) -> HttpResponse {
    HttpResponse { status: 200, content_type: "text/html; charset=utf-8".into(), body: value.as_bytes().to_vec() }
}

fn empty(status: u16) -> HttpResponse {
    HttpResponse { status, content_type: "text/plain; charset=utf-8".into(), body: Vec::new() }
}

const INDEX_HTML: &str = r##"<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore Provisioning Core</title>
<style>:root{
  color-scheme:dark;
  --bg:#08111f;--panel:#101d30;--panel-raised:#13243a;--panel2:#172a45;
  --line:#2b4264;--line-strong:#3a5a84;--text:#eef5ff;--muted:#9fb1c9;
  --ok:#4ade80;--bad:#fb7185;--accent:#60a5fa;--accent-strong:#2563eb;--warn:#fbbf24;
  --header-h:86px;
}
*{box-sizing:border-box}
html{scrollbar-gutter:stable}
body{margin:0;min-width:320px;background:linear-gradient(135deg,#07101d,#0d1830);color:var(--text);font:14px/1.45 system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}
button,input,textarea,select{font:inherit}
header{min-height:var(--header-h);padding:20px 28px;border-bottom:1px solid var(--line);background:#0b1628f2;position:sticky;top:0;z-index:20;backdrop-filter:blur(12px)}
h1{margin:0;font-size:25px;line-height:1.2}header p{margin:5px 0 0;color:var(--muted)}
main{padding:22px;max-width:1800px;margin:auto}
.cards{display:grid;grid-template-columns:repeat(5,minmax(150px,1fr));gap:12px}
.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:14px;box-shadow:0 10px 30px #02081733}
.card{padding:15px}.card b{font-size:23px;display:block}.card span{color:var(--muted)}
.toolbar{display:flex;gap:9px;flex-wrap:wrap;align-items:center}
.toolbar.tabs{margin:16px 0;position:sticky;top:calc(var(--header-h) + 8px);z-index:15;padding:9px;background:#0b1628e8;border:1px solid var(--line);border-radius:12px;backdrop-filter:blur(10px)}
button{border:1px solid var(--line);background:var(--panel2);color:var(--text);padding:9px 13px;border-radius:8px;cursor:pointer;transition:background .15s,border-color .15s,transform .05s}
button:hover{background:#1b3150;border-color:var(--line-strong)}button:active{transform:translateY(1px)}
.tabs button.active,button.primary{background:var(--accent-strong);border-color:#3b82f6}.danger{background:#651d32!important}.warn{background:#614611!important}
.tab{display:none}.tab.active{display:block}.panel{overflow:hidden}
.panel-head{display:flex;gap:18px;align-items:center;justify-content:space-between;padding:16px;border-bottom:1px solid var(--line);background:linear-gradient(180deg,#13243a,#101d30)}
.panel-head-copy h2{font-size:17px;margin:0}.panel-head-copy p{margin:4px 0 0;color:var(--muted)}
.panel-tools{display:flex;gap:9px;align-items:center;justify-content:flex-end;flex-wrap:wrap;min-width:min(100%,540px)}
.search{width:min(360px,100%);min-width:220px}
input,textarea,select{width:100%;background:#091526;color:var(--text);border:1px solid var(--line);border-radius:8px;padding:9px 10px;outline:none}
input:focus,textarea:focus,select:focus{border-color:var(--accent);box-shadow:0 0 0 3px #2563eb26}
.table-wrap{overflow:auto;max-height:calc(100vh - 315px);min-height:180px;scrollbar-gutter:stable both-edges}
table{border-collapse:separate;border-spacing:0;width:100%;min-width:900px}
th,td{border-bottom:1px solid var(--line);padding:11px 12px;text-align:left;vertical-align:middle}
thead th{color:#c6d8ef;position:sticky;top:0;z-index:5;background:#14243b;box-shadow:0 1px 0 var(--line),0 7px 16px #02081755;white-space:nowrap}
tbody tr:hover td{background:#14243b80}tbody tr:last-child td{border-bottom:0}
.groups-table th:nth-child(1),.groups-table td:nth-child(1){width:110px}.groups-table th:nth-child(2),.groups-table td:nth-child(2){min-width:210px}.groups-table th:nth-child(n+3):nth-child(-n+6),.groups-table td:nth-child(n+3):nth-child(-n+6){width:90px;text-align:center}.groups-table th:last-child,.groups-table td:last-child{width:220px}
.devices-table th:first-child,.devices-table td:first-child{width:105px}.devices-table th:last-child,.devices-table td:last-child{width:300px}
.actions{display:flex;gap:7px;flex-wrap:wrap;align-items:center}.actions button{white-space:nowrap}
.pill{display:inline-flex;align-items:center;padding:3px 7px;border-radius:999px;background:#263b58;color:#d6e5f8;font-size:11px;line-height:1.2}.pill.auto{background:#173f5f}.pill.locked{background:#563044;color:#ffd5df}
.ok{color:var(--ok)}.bad{color:var(--bad)}.muted{color:var(--muted)}
.empty{padding:30px!important;text-align:center!important;color:var(--muted)}
.matrix-legend{display:flex;gap:14px;align-items:center;flex-wrap:wrap;padding:10px 16px;border-bottom:1px solid var(--line);background:#0d192a;color:var(--muted);font-size:12px}
.matrix-wrap{max-height:calc(100vh - 360px);min-height:250px;background:#0c1728}
.matrix{width:max-content;min-width:max-content;table-layout:fixed}
.matrix th,.matrix td{width:136px;min-width:136px;max-width:136px;text-align:center;padding:10px 8px}
.matrix thead th{height:76px;white-space:normal;vertical-align:middle}
.matrix th:first-child,.matrix td:first-child{width:220px;min-width:220px;max-width:220px;position:sticky;left:0;text-align:left;background:#14243b;z-index:7;box-shadow:1px 0 0 var(--line),8px 0 18px #02081740}
.matrix thead th:first-child{z-index:10}
.matrix tbody td:first-child{background:#101d30}.matrix tbody tr:hover td:first-child{background:#14243b}
.matrix-group{display:flex;min-height:52px;flex-direction:column;align-items:center;justify-content:center;gap:2px;overflow:hidden}.matrix-group b{font-size:13px}.matrix-group span{display:block;width:100%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:#bdd0e8;font-size:12px}.matrix-group small{color:var(--muted)}
.matrix-device{display:grid;grid-template-columns:auto 1fr;gap:8px;align-items:center}.matrix-device-state{width:9px;height:9px;border-radius:50%;background:#64748b}.matrix-device-state.active{background:var(--ok);box-shadow:0 0 0 4px #4ade8018}.matrix-device-state.blocked{background:var(--bad)}.matrix-device b,.matrix-device span,.matrix-device small{display:block;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.matrix-device span{color:#c7d7ea}.matrix-device small{color:var(--muted)}
.matrix-cell{min-height:82px;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:7px}.matrix-cell.member{background:#14315366;border-radius:10px}.matrix-check{width:22px;height:22px;accent-color:#3b82f6;cursor:pointer}.matrix-detail{padding:5px 9px;font-size:12px;line-height:1}.matrix-flags{min-height:18px;display:flex;justify-content:center;gap:4px;flex-wrap:wrap}
label{display:block;margin:9px 0}.checks{display:grid;grid-template-columns:repeat(3,1fr);gap:6px}.checks label{display:flex;gap:6px;align-items:center}.checks input{width:auto}
dialog{background:var(--panel);color:var(--text);border:1px solid var(--line);border-radius:14px;width:min(720px,95vw);max-height:90vh;overflow:auto}dialog::backdrop{background:#000a}
@media(max-width:1000px){.cards{grid-template-columns:repeat(3,1fr)}.panel-head{align-items:flex-start;flex-direction:column}.panel-tools{width:100%;justify-content:flex-start}.search{flex:1}.table-wrap{max-height:calc(100vh - 350px)}}
@media(max-width:700px){:root{--header-h:0px}header{position:static;padding:17px 18px}main{padding:12px}.cards{grid-template-columns:1fr 1fr}.toolbar.tabs{position:static}.panel-tools{display:grid;grid-template-columns:1fr}.search{min-width:0;width:100%}.checks{grid-template-columns:1fr}.table-wrap,.matrix-wrap{max-height:none}.matrix th:first-child,.matrix td:first-child{width:170px;min-width:170px;max-width:170px}.matrix th,.matrix td{width:122px;min-width:122px;max-width:122px}}</style></head><body>
<header><h1>NetCore Provisioning Core</h1><p>Zentrale Geräte-, Gruppen- und Mitgliedschaftsverwaltung · OPEN LAB</p></header>
<main><div class="cards"><div class="card"><b id="deviceCount">–</b><span>Geräte</span></div><div class="card"><b id="registeredCount">–</b><span>eingebucht</span></div><div class="card"><b id="groupCount">–</b><span>Gruppen</span></div><div class="card"><b id="membershipCount">–</b><span>Mitgliedschaften</span></div><div class="card"><b id="health">–</b><span>Core-Status</span></div></div>
<div class="toolbar tabs"><button data-tab="devices" class="active">Geräte</button><button data-tab="groups">Gruppen</button><button data-tab="matrix">Mitgliedschaftsmatrix</button><button onclick="syncAll()">Jetzt synchronisieren</button><button onclick="refresh()">Neu laden</button></div>
<section id="devices" class="tab active panel"><div class="panel-head"><div class="panel-head-copy"><h2>Geräte</h2><p>Teilnehmer verwalten, sperren und den aktuellen Funkstatus prüfen.</p></div><div class="panel-tools"><button class="primary" onclick="editDevice()">Gerät anlegen</button><input id="deviceSearch" class="search" placeholder="ISSI, Name oder Organisation filtern" oninput="renderDevices()"></div></div><div class="table-wrap"><table class="devices-table"><thead><tr><th>ISSI</th><th>Name</th><th>Organisation</th><th>Gerät</th><th>Freigabe</th><th>Funkstatus</th><th>Aktionen</th></tr></thead><tbody id="deviceRows"></tbody></table></div></section>
<section id="groups" class="tab panel"><div class="panel-head"><div class="panel-head-copy"><h2>Gruppen</h2><p>Attach, Rufarten, SDS und Notruf je GSSI zentral freigeben.</p></div><div class="panel-tools"><button class="primary" onclick="editGroup()">Gruppe anlegen</button><input id="groupSearch" class="search" placeholder="GSSI oder Name filtern" oninput="renderGroups()"></div></div><div class="table-wrap"><table class="groups-table"><thead><tr><th>GSSI</th><th>Name</th><th>Attach</th><th>Ruf</th><th>SDS</th><th>Notruf</th><th>Aktionen</th></tr></thead><tbody id="groupRows"></tbody></table></div></section>
<section id="matrix" class="tab panel"><div class="panel-head"><div class="panel-head-copy"><h2>Mitgliedschaftsmatrix</h2><p>Zeile = Gerät, Spalte = Gruppe. Änderungen werden sofort an Group Core und danach an die TBS verteilt.</p></div><div class="panel-tools"><input id="matrixSearch" class="search" placeholder="Geräte filtern" oninput="renderMatrix()"><input id="matrixGroupSearch" class="search" placeholder="Gruppen filtern" oninput="renderMatrix()"></div></div><div class="matrix-legend"><span>☑ Mitglied</span><span><span class="pill auto">auto</span> Auto-Attach</span><span><span class="pill locked">fix</span> fixierte Mitgliedschaft</span><span>Horizontal scrollen für weitere Gruppen</span></div><div class="table-wrap matrix-wrap"><table class="matrix"><thead id="matrixHead"></thead><tbody id="matrixBody"></tbody></table></div></section>
</main>
<dialog id="deviceDialog"><form id="deviceForm"><h2 id="deviceTitle">Gerät</h2><label>ISSI<input name="issi" type="number" min="1" max="16777215" required></label><label>Anzeigename<input name="display_name"></label><label>Organisation<input name="organization"></label><label>Home MCC<input name="home_mcc" type="number" min="0" max="1023"></label><label>Home MNC<input name="home_mnc" type="number" min="0" max="16383"></label><label>Gerätebezeichnung<input name="device_label"></label><label>TEI<input name="device_tei" type="number"></label><div class="checks"><label><input name="enabled" type="checkbox" checked> Aktiv</label><label><input name="registration_allowed" type="checkbox" checked> Registrierung</label><label><input name="emergency_allowed" type="checkbox"> Notruf</label><label><input name="sds_allowed" type="checkbox" checked> SDS</label><label><input name="packet_data_allowed" type="checkbox"> Paketdaten</label></div><label>Rufpriorität<input name="call_priority" type="number" min="0" max="15" value="0"></label><label>Notizen<textarea name="notes"></textarea></label><div class="actions"><button class="primary" type="submit">Speichern</button><button type="button" onclick="deviceDialog.close()">Abbrechen</button></div></form></dialog>
<dialog id="groupDialog"><form id="groupForm"><h2 id="groupTitle">Gruppe</h2><label>GSSI<input name="gssi" type="number" min="1" max="16777215" required></label><label>Name<input name="name"></label><label>Beschreibung<textarea name="description"></textarea></label><div class="checks"><label><input name="enabled" type="checkbox" checked> Aktiv</label><label><input name="attach_allowed" type="checkbox" checked> Attach</label><label><input name="call_allowed" type="checkbox" checked> Gruppenruf</label><label><input name="sds_allowed" type="checkbox" checked> SDS</label><label><input name="emergency_allowed" type="checkbox"> Notruf</label><label><input name="dgna_allowed" type="checkbox" checked> DGNA</label></div><label>Rufpriorität<input name="call_priority" type="number" min="0" max="15" value="0"></label><label>Class of Usage<input name="class_of_usage" type="number" min="0" max="15" value="4"></label><label>Notizen<textarea name="notes"></textarea></label><div class="actions"><button class="primary" type="submit">Speichern</button><button type="button" onclick="groupDialog.close()">Abbrechen</button></div></form></dialog>
<dialog id="membershipDialog"><form id="membershipForm"><h2 id="membershipTitle">Mitgliedschaft</h2><input name="issi" type="hidden"><input name="gssi" type="hidden"><div class="checks"><label><input name="allowed" type="checkbox" checked> Mitglied</label><label><input name="auto_attach" type="checkbox"> Auto-Attach</label><label><input name="locked" type="checkbox"> Gesperrt/fixiert</label></div><label>Notizen<textarea name="notes"></textarea></label><div class="actions"><button class="primary" type="submit">Speichern</button><button type="button" onclick="membershipDialog.close()">Abbrechen</button></div></form></dialog>
<script>
let state={subscribers:[],observed:[],groups:[],memberships:[]};const $=id=>document.getElementById(id),deviceDialog=$('deviceDialog'),groupDialog=$('groupDialog'),membershipDialog=$('membershipDialog');async function api(path,opt={}){const r=await fetch(path,opt);let body=null;try{body=await r.json()}catch{}if(!r.ok)throw new Error(body?.error||`HTTP ${r.status}`);return body}
async function refresh(){try{const d=await api('/api/v1/dashboard');state=d;$('deviceCount').textContent=d.subscribers.length;$('registeredCount').textContent=d.observed.filter(o=>o.registered).length;$('groupCount').textContent=d.groups.length;$('membershipCount').textContent=d.memberships.length;$('health').innerHTML=d.failures.length?'<span class="bad">DEGRADED</span>':'<span class="ok">ONLINE</span>';renderDevices();renderGroups();renderMatrix()}catch(e){$('health').innerHTML='<span class="bad">OFFLINE</span>';console.error(e)}}
document.querySelectorAll('.tabs [data-tab]').forEach(b=>b.onclick=()=>{document.querySelectorAll('.tabs [data-tab]').forEach(x=>x.classList.toggle('active',x===b));document.querySelectorAll('.tab').forEach(x=>x.classList.toggle('active',x.id===b.dataset.tab))});
function esc(v){return String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]))}
function renderDevices(){const q=$('deviceSearch').value.toLowerCase(),rows=state.subscribers.filter(s=>`${s.issi} ${s.display_name} ${s.organization}`.toLowerCase().includes(q)).map(s=>{const o=state.observed.find(x=>x.issi===s.issi),radio=o?.registered?`<span class="ok">eingebucht</span><br><small>${esc(o.serving_node||'')} ${o.last_rssi_dbfs==null?'':Number(o.last_rssi_dbfs).toFixed(1)+' dBFS'}</small>`:'<span class="muted">offline/unbekannt</span>';return `<tr><td><b>${s.issi}</b></td><td>${esc(s.display_name)}</td><td>${esc(s.organization)}</td><td>${esc(s.device_label)}</td><td>${s.enabled&&s.registration_allowed?'<span class="ok">freigegeben</span>':'<span class="bad">gesperrt</span>'}</td><td>${radio}</td><td><div class="actions"><button onclick="editDevice(${s.issi})">Bearbeiten</button><button class="warn" onclick="toggleBlock(${s.issi})">${s.enabled?'Sperren':'Freigeben'}</button><button class="danger" onclick="deleteDevice(${s.issi})">Löschen</button></div></td></tr>`});$('deviceRows').innerHTML=rows.length?rows.join(''):'<tr><td class="empty" colspan="7">Keine Geräte gefunden.</td></tr>'}
function renderGroups(){const q=$('groupSearch').value.toLowerCase(),rows=state.groups.filter(g=>`${g.gssi} ${g.name}`.toLowerCase().includes(q)).map(g=>`<tr><td><b>${g.gssi}</b></td><td>${esc(g.name)}</td><td>${mark(g.attach_allowed)}</td><td>${mark(g.call_allowed)}</td><td>${mark(g.sds_allowed)}</td><td>${mark(g.emergency_allowed)}</td><td><div class="actions"><button onclick="editGroup(${g.gssi})">Bearbeiten</button><button class="danger" onclick="deleteGroup(${g.gssi})">Löschen</button></div></td></tr>`);$('groupRows').innerHTML=rows.length?rows.join(''):'<tr><td class="empty" colspan="7">Keine Gruppen gefunden.</td></tr>'}function mark(v){return v?'<span class="ok">✓</span>':'<span class="bad">–</span>'}
function membership(issi,gssi){return state.memberships.find(m=>m.issi===issi&&m.gssi===gssi&&m.allowed)}
function renderMatrix(){const deviceQuery=$('matrixSearch').value.toLowerCase(),groupQuery=$('matrixGroupSearch').value.toLowerCase(),groups=[...state.groups].filter(g=>`${g.gssi} ${g.name}`.toLowerCase().includes(groupQuery)).sort((a,b)=>a.gssi-b.gssi),subs=[...state.subscribers].filter(s=>`${s.issi} ${s.display_name} ${s.organization}`.toLowerCase().includes(deviceQuery)).sort((a,b)=>a.issi-b.issi);$('matrixHead').innerHTML='<tr><th><div class="matrix-group"><b>Gerät / ISSI</b><small>'+subs.length+' angezeigt</small></div></th>'+groups.map(g=>`<th><div class="matrix-group"><b>${g.gssi}</b><span title="${esc(g.name)}">${esc(g.name)}</span><small>${g.enabled?'aktiv':'deaktiviert'}</small></div></th>`).join('')+'</tr>';if(!subs.length){$('matrixBody').innerHTML=`<tr><td class="empty" colspan="${Math.max(1,groups.length+1)}">Keine Geräte gefunden.</td></tr>`;return}$('matrixBody').innerHTML=subs.map(s=>{const active=s.enabled&&s.registration_allowed,stateClass=active?'active':'blocked',stateText=active?'freigegeben':'gesperrt';return `<tr><td><div class="matrix-device"><span class="matrix-device-state ${stateClass}"></span><div><b>${s.issi}</b><span>${esc(s.display_name||'Ohne Namen')}</span><small>${stateText}${s.organization?' · '+esc(s.organization):''}</small></div></div></td>${groups.map(g=>{const m=membership(s.issi,g.gssi),flags=`${m?.auto_attach?'<span class="pill auto">auto</span>':''}${m?.locked?'<span class="pill locked">fix</span>':''}`;return `<td><div class="matrix-cell ${m?'member':''}"><input class="matrix-check" type="checkbox" ${m?'checked':''} onchange="setMembership(${s.issi},${g.gssi},this.checked)" aria-label="ISSI ${s.issi} Mitglied in GSSI ${g.gssi}"><button class="matrix-detail" type="button" onclick="editMembership(${s.issi},${g.gssi})">Details</button><div class="matrix-flags">${flags}</div></div></td>`}).join('')}</tr>`}).join('')}
function fill(form,obj,names){names.forEach(n=>{const el=form.elements[n];if(!el)return;if(el.type==='checkbox')el.checked=!!obj?.[n];else el.value=obj?.[n]??''})}
function editDevice(issi){const s=state.subscribers.find(x=>x.issi===issi),f=$('deviceForm');f.dataset.id=s?.issi||'';fill(f,s,['issi','display_name','organization','home_mcc','home_mnc','device_label','device_tei','enabled','registration_allowed','emergency_allowed','sds_allowed','packet_data_allowed','call_priority','notes']);if(!s){f.reset();f.elements.enabled.checked=true;f.elements.registration_allowed.checked=true;f.elements.sds_allowed.checked=true}f.elements.issi.disabled=!!s;$('deviceTitle').textContent=s?`Gerät ${s.issi} bearbeiten`:'Gerät anlegen';deviceDialog.showModal()}
$('deviceForm').onsubmit=async e=>{e.preventDefault();const f=e.target,id=f.dataset.id,n=n=>Number(f.elements[n].value||0),b=n=>f.elements[n].checked,current=state.subscribers.find(x=>x.issi===Number(id)),p={issi:id?Number(id):n('issi'),home_mcc:n('home_mcc'),home_mnc:n('home_mnc'),display_name:f.elements.display_name.value,organization:f.elements.organization.value,device_label:f.elements.device_label.value,device_tei:f.elements.device_tei.value?Number(f.elements.device_tei.value):null,enabled:b('enabled'),registration_allowed:b('registration_allowed'),call_priority:n('call_priority'),emergency_allowed:b('emergency_allowed'),sds_allowed:b('sds_allowed'),packet_data_allowed:b('packet_data_allowed'),default_groups:current?.default_groups||[],notes:f.elements.notes.value};await api(id?`/api/v1/subscribers/${id}`:'/api/v1/subscribers',{method:id?'PUT':'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});deviceDialog.close();await refresh()};
async function toggleBlock(issi){const s=state.subscribers.find(x=>x.issi===issi),p={...s,enabled:!s.enabled,registration_allowed:!s.enabled,default_groups:s.default_groups||[]};await api(`/api/v1/subscribers/${issi}`,{method:'PUT',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});refresh()}async function deleteDevice(issi){if(!confirm(`Gerät ${issi} inklusive aller Mitgliedschaften löschen?`))return;await api(`/api/v1/subscribers/${issi}`,{method:'DELETE'});refresh()}
function editGroup(gssi){const g=state.groups.find(x=>x.gssi===gssi),f=$('groupForm');f.dataset.id=g?.gssi||'';fill(f,g,['gssi','name','description','enabled','attach_allowed','call_allowed','sds_allowed','emergency_allowed','dgna_allowed','call_priority','class_of_usage','notes']);if(!g){f.reset();['enabled','attach_allowed','call_allowed','sds_allowed','dgna_allowed'].forEach(n=>f.elements[n].checked=true);f.elements.class_of_usage.value=4}f.elements.gssi.disabled=!!g;$('groupTitle').textContent=g?`Gruppe ${g.gssi} bearbeiten`:'Gruppe anlegen';groupDialog.showModal()}
$('groupForm').onsubmit=async e=>{e.preventDefault();const f=e.target,id=f.dataset.id,n=x=>Number(f.elements[x].value||0),b=x=>f.elements[x].checked,current=state.groups.find(x=>x.gssi===Number(id)),p={gssi:id?Number(id):n('gssi'),name:f.elements.name.value,description:f.elements.description.value,enabled:b('enabled'),attach_allowed:b('attach_allowed'),dgna_allowed:b('dgna_allowed'),call_allowed:b('call_allowed'),sds_allowed:b('sds_allowed'),emergency_allowed:b('emergency_allowed'),call_priority:n('call_priority'),class_of_usage:n('class_of_usage'),area_nodes:current?.area_nodes||[],notes:f.elements.notes.value};await api(id?`/api/v1/groups/${id}`:'/api/v1/groups',{method:id?'PUT':'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});groupDialog.close();refresh()};async function deleteGroup(gssi){if(!confirm(`Gruppe ${gssi} inklusive aller Mitgliedschaften löschen?`))return;await api(`/api/v1/groups/${gssi}`,{method:'DELETE'});refresh()}
function editMembership(issi,gssi){const m=state.memberships.find(x=>x.issi===issi&&x.gssi===gssi),f=$('membershipForm');f.elements.issi.value=issi;f.elements.gssi.value=gssi;f.elements.allowed.checked=m?.allowed??true;f.elements.auto_attach.checked=!!m?.auto_attach;f.elements.locked.checked=!!m?.locked;f.elements.notes.value=m?.notes||'';$('membershipTitle').textContent=`ISSI ${issi} ↔ GSSI ${gssi}`;membershipDialog.showModal()}
$('membershipForm').onsubmit=async e=>{e.preventDefault();const f=e.target,issi=Number(f.elements.issi.value),gssi=Number(f.elements.gssi.value),p={issi,gssi,allowed:f.elements.allowed.checked,auto_attach:f.elements.auto_attach.checked,locked:f.elements.locked.checked,notes:f.elements.notes.value};await api('/api/v1/memberships',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(p)});membershipDialog.close();refresh()};
async function setMembership(issi,gssi,checked){try{if(checked)await api('/api/v1/memberships',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({issi,gssi,allowed:true,auto_attach:false,locked:false,notes:''})});else await api(`/api/v1/memberships/${issi}/${gssi}`,{method:'DELETE'});await refresh()}catch(e){alert(e.message);refresh()}}async function syncAll(){const r=await api('/api/v1/sync',{method:'POST'});alert('Synchronisation an Subscriber Core und Group Core ausgelöst');refresh()}refresh();setInterval(refresh,10000);
</script></body></html>"##;
