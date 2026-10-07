#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

use crate::collector;
use crate::config::ObservabilityConfig;
use crate::protocol::{
    ActionInput, DiagnosticInput, LogIngestInput, MaintenanceInput, RuleInput, SilenceInput,
    TargetCreateInput, TraceIngestInput,
};
use crate::state::SharedObservability;

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(config: ObservabilityConfig, observability: SharedObservability) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Observability WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => {
                    let config = config.clone(); let observability = observability.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, config, observability) { tracing::warn!("Observability HTTP connection failed: {}", error); }
                    });
                }
                Err(error) => tracing::warn!("Observability HTTP accept failed: {}", error),
            }
        }
    }))
}

// Was: Bündelt die zusammengehörigen Werte für HTTP request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpRequest { method: String, path: String, query: HashMap<String,String>, body: Vec<u8> }
// Was: Bündelt die zusammengehörigen Werte für HTTP response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResponse { status: u16, content_type: &'static str, body: Vec<u8>, disposition: Option<String>, extra_headers: Vec<(String,String)> }

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(mut stream: TcpStream, config: ObservabilityConfig, observability: SharedObservability) -> Result<(),String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?; let response = route(request, config, observability);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(request: HttpRequest, config: ObservabilityConfig, observability: SharedObservability) -> HttpResponse {
    if request.method == "OPTIONS" { return empty(204); }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Observability", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => { let status = observability.status(); json_response(if status.ready {200} else {503}, &status) }
        ("GET", "/api/v1/status") => json_response(200, &observability.status()),
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/api/v1/syslog") => {
            let root = config.storage.state_path.parent().unwrap_or_else(|| std::path::Path::new("."));
            let read = |name: &str| -> serde_json::Value {
                std::fs::read(root.join("logs").join(name)).ok().and_then(|v| serde_json::from_slice(&v).ok()).unwrap_or(serde_json::Value::Null)
            };
            json_response(200, &json!({"receiver": read("receiver-status.json"), "archive": read("archive-status.json")}))
        },
        ("GET", "/api/v1/targets/prometheus") => {
            let values: Vec<_> = observability.targets_for_scrape().iter().map(|t| json!({
                "targets": [t.base_url.trim_start_matches("http://")],
                "labels": {"service": t.service, "target_id": t.target_id, "__metrics_path__": t.metrics_path}
            })).collect();
            json_response(200, &values)
        },
        ("GET", "/api/v1/discovery") => json_response(200, &observability.discovery_status()),
        ("POST", "/api/v1/discovery/sync") => match crate::discovery::sync(&config, &observability) {
            Ok(_) => json_response(200, &observability.discovery_status()),
            Err(error) => { observability.discovery_failed(error); json_response(503, &observability.discovery_status()) }
        },
        ("GET", "/api/v1/targets") => json_response(200, &observability.targets()),
        ("POST", "/api/v1/targets") => match parse_json::<TargetCreateInput>(&request.body).and_then(|input| observability.create_target(input)) { Ok(value) => json_response(201,&value), Err(error) => conflict(error) },
        ("GET", "/api/v1/stack") => json_response(200, &observability.stack()),
        ("GET", "/api/v1/metrics/catalog") => json_response(200, &observability.metric_catalog()),
        ("GET", "/api/v1/metrics/series") => {
            let limit = query_usize(&request,"limit",200,5000);
            json_response(200,&observability.series(request.query.get("metric").map(String::as_str),request.query.get("target_id").map(String::as_str),request.query.get("service").map(String::as_str),limit))
        }
        ("GET", "/api/v1/rules") => json_response(200,&observability.rules()),
        ("POST", "/api/v1/rules") => match parse_json::<RuleInput>(&request.body).and_then(|input| observability.create_rule(input)) { Ok(value)=>json_response(201,&value),Err(error)=>conflict(error) },
        ("GET", "/api/v1/alerts") => {
            let limit=query_usize(&request,"limit",500,5000);
            json_response(200,&observability.alerts(request.query.get("state").map(String::as_str),request.query.get("severity").map(String::as_str),request.query.get("service").map(String::as_str),limit))
        }
        ("GET", "/api/v1/silences") => json_response(200,&observability.silences()),
        ("POST", "/api/v1/silences") => match parse_json::<SilenceInput>(&request.body).and_then(|input| observability.create_silence(input)) { Ok(value)=>json_response(201,&value),Err(error)=>conflict(error) },
        ("GET", "/api/v1/logs") => {
            let limit=query_usize(&request,"limit",500,10000);
            json_response(200,&observability.logs(request.query.get("service").map(String::as_str),request.query.get("level").map(String::as_str),request.query.get("contains").map(String::as_str),request.query.get("trace_id").map(String::as_str),limit))
        }
        ("POST", "/api/v1/logs/ingest") => match parse_json::<LogIngestInput>(&request.body).and_then(|input| observability.ingest_logs(input)) { Ok(count)=>json_response(202,&json!({"accepted":count})),Err(error)=>conflict(error) },
        ("GET", "/api/v1/traces") => {
            let limit=query_usize(&request,"limit",500,10000);
            json_response(200,&observability.traces(request.query.get("service").map(String::as_str),request.query.get("trace_id").map(String::as_str),request.query.get("status").map(String::as_str),limit))
        }
        ("POST", "/api/v1/traces/ingest") => match parse_json::<TraceIngestInput>(&request.body).and_then(|input| observability.ingest_traces(input)) { Ok(count)=>json_response(202,&json!({"accepted":count})),Err(error)=>conflict(error) },
        ("GET", "/api/v1/audit") => json_response(200,&observability.audit(query_usize(&request,"limit",500,5000))),
        ("GET", "/api/v1/diagnostics") => json_response(200,&observability.diagnostics()),
        ("POST", "/api/v1/diagnostics") => match parse_json_or_default::<DiagnosticInput>(&request.body).and_then(|input| observability.create_diagnostic(input)) { Ok(value)=>json_response(201,&value),Err(error)=>json_response(500,&json!({"error":error})) },
        ("POST", "/api/v1/maintenance/tick") => {
            let input=parse_json_or_default::<MaintenanceInput>(&request.body);
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match input.and_then(|input| observability.maintenance(input.actor).map_err(|error|error.to_string())) { Ok(value)=>json_response(200,&value),Err(error)=>json_response(500,&json!({"error":error})) }
        }
        ("POST", "/api/v1/maintenance/scrape-now") => { collector::run_cycle(&config,&observability); json_response(200,&observability.status()) }
        ("POST", "/api/v1/maintenance/backup") => {
            let input=parse_json_or_default::<MaintenanceInput>(&request.body);
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match input.and_then(|input| observability.backup(input.actor)) { Ok(value)=>json_response(201,&value),Err(error)=>json_response(500,&json!({"error":error})) }
        }
        ("GET", "/api/v1/export.json") => download("netcore-observability-export.json","application/json",serde_json::to_vec_pretty(&observability.export()).unwrap_or_default()),
        ("GET", "/metrics") => text("text/plain; version=0.0.4; charset=utf-8",observability.metrics()),
        ("GET", "/openapi.json") => json_response(200,&openapi()),
        _ => dynamic_route(request,config,observability),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(request: HttpRequest, config: ObservabilityConfig, observability: SharedObservability) -> HttpResponse {
    let parts: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(),parts.as_slice()) {
        ("POST",["api","v1","targets",target_id,"test"]) => {
            let Some(target)=observability.target(target_id) else { return not_found(format!("target {target_id} not found")); };
            let result=collector::scrape_target(&config,&target); let snapshot=result.clone();
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match observability.record_scrape(result) { Ok(())=>json_response(200,&json!({"target":observability.target(target_id),"live":snapshot.live,"ready":snapshot.ready,"metrics_ok":snapshot.metrics_ok,"response_ms":snapshot.response_ms,"samples":snapshot.metrics.len(),"error":snapshot.error})),Err(error)=>json_response(500,&json!({"error":error.to_string()})) }
        }
        ("POST",["api","v1","targets",target_id,action]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.target_action(target_id,action,input)) { Ok(value)=>json_response(200,&value),Err(error)=>conflict(error) },
        ("DELETE",["api","v1","targets",target_id]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.delete_target(target_id,input)) { Ok(())=>empty(204),Err(error)=>not_found(error) },
        ("PUT",["api","v1","rules",rule_id]) => match parse_json::<RuleInput>(&request.body).and_then(|input|observability.update_rule(rule_id,input)) { Ok(value)=>json_response(200,&value),Err(error)=>conflict(error) },
        ("POST",["api","v1","rules",rule_id,action]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.rule_action(rule_id,action,input)) { Ok(value)=>json_response(200,&value),Err(error)=>conflict(error) },
        ("DELETE",["api","v1","rules",rule_id]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.delete_rule(rule_id,input)) { Ok(())=>empty(204),Err(error)=>not_found(error) },
        ("POST",["api","v1","alerts",alert_id,action]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.alert_action(alert_id,action,input)) { Ok(value)=>json_response(200,&value),Err(error)=>conflict(error) },
        ("POST",["api","v1","silences",silence_id,"expire"]) => match parse_json_or_default::<ActionInput>(&request.body).and_then(|input|observability.expire_silence(silence_id,input)) { Ok(value)=>json_response(200,&value),Err(error)=>conflict(error) },
        ("GET",["api","v1","diagnostics",diagnostic_id,"download"]) => match observability.diagnostic_file(diagnostic_id) { Ok((name,bytes))=>download(&name,"application/gzip",bytes),Err(error)=>not_found(error) },
        _ => not_found("route not found".to_string()),
    }
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> Value { json!({
    "openapi":"3.0.3","info":{"title":"NetCore Observability API","version":"1.0.0","description":"OPEN LAB: no authentication, no tokens and no TLS"},
    "servers":[{"url":"/"}],"paths":{
        "/api/v1/status":{"get":{"summary":"NMS status"}},
        "/api/v1/targets":{"get":{"summary":"List scrape targets"},"post":{"summary":"Create target"}},
        "/api/v1/targets/{target_id}/test":{"post":{"summary":"Test and ingest one target"}},
        "/api/v1/metrics/catalog":{"get":{"summary":"Metric catalog"}},
        "/api/v1/metrics/series":{"get":{"summary":"Query bounded time series"}},
        "/api/v1/logs":{"get":{"summary":"Search logs"}},
        "/api/v1/discovery":{"get":{"summary":"Discovery and cached endpoint status"}},
        "/api/v1/discovery/sync":{"post":{"summary":"Refresh endpoints from deployment VM"}},
        "/api/v1/syslog":{"get":{"summary":"Syslog buffers, sources, drops and archive status"}},
        "/api/v1/targets/prometheus":{"get":{"summary":"Prometheus HTTP service discovery"}},
        "/api/v1/logs/ingest":{"post":{"summary":"Ingest NetCore JSON logs"}},
        "/api/v1/traces":{"get":{"summary":"Search trace spans"}},
        "/api/v1/traces/ingest":{"post":{"summary":"Ingest NetCore JSON spans"}},
        "/api/v1/rules":{"get":{"summary":"List alert rules"},"post":{"summary":"Create alert rule"}},
        "/api/v1/alerts":{"get":{"summary":"List alert instances"}},
        "/api/v1/silences":{"get":{"summary":"List silences"},"post":{"summary":"Create silence"}},
        "/api/v1/diagnostics":{"get":{"summary":"List diagnostic bundles"},"post":{"summary":"Build diagnostic bundle"}},
        "/api/v1/maintenance/scrape-now":{"post":{"summary":"Run collection immediately"}},
        "/metrics":{"get":{"summary":"Prometheus metrics for the NMS"}}
    }
}) }

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream:&mut TcpStream,max_body_bytes:usize)->Result<HttpRequest,String>{
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).map_err(|error|error.to_string())?;
    let mut raw=Vec::new(); let mut buffer=[0u8;8192]; let header_end;
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop { let count=stream.read(&mut buffer).map_err(|error|error.to_string())?; if count==0{return Err("connection closed before headers".to_string());} raw.extend_from_slice(&buffer[..count]); if let Some(position)=raw.windows(4).position(|window|window==b"\r\n\r\n"){header_end=position;break;} if raw.len()>64*1024{return Err("request headers too large".to_string());} }
    let headers=String::from_utf8(raw[..header_end].to_vec()).map_err(|_|"headers are not UTF-8".to_string())?; let mut lines=headers.lines(); let request_line=lines.next().ok_or_else(||"missing request line".to_string())?; let mut parts=request_line.split_whitespace(); let method=parts.next().unwrap_or("").to_string(); let target=parts.next().unwrap_or("/");
    let content_length=lines.filter_map(|line|line.split_once(':')).find(|(key,_)|key.eq_ignore_ascii_case("content-length")).and_then(|(_,value)|value.trim().parse::<usize>().ok()).unwrap_or(0); if content_length>max_body_bytes{return Err(format!("request body exceeds {max_body_bytes} bytes"));}
    let body_start=header_end+4; while raw.len()<body_start+content_length { let count=stream.read(&mut buffer).map_err(|error|error.to_string())?; if count==0{break;} raw.extend_from_slice(&buffer[..count]); if raw.len()>body_start+max_body_bytes{return Err("request body too large".to_string());} }
    if raw.len()<body_start+content_length{return Err("truncated request body".to_string());}
    let (path,query)=parse_target(target); Ok(HttpRequest{method,path,query,body:raw[body_start..body_start+content_length].to_vec()})
}

// Was: Diese Funktion liest und prüft target.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_target(target:&str)->(String,HashMap<String,String>){let (path,query)=target.split_once('?').unwrap_or((target,""));let mut values=HashMap::new();for part in query.split('&').filter(|part|!part.is_empty()){let (key,value)=part.split_once('=').unwrap_or((part,""));values.insert(percent_decode(key),percent_decode(value));}(percent_decode(path),values)}
// Was: Führt den Arbeitsschritt `percent_decode` für percent Dekodierung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn percent_decode(value:&str)->String{let bytes=value.as_bytes();let mut output=Vec::new();let mut index=0;while index<bytes.len(){match bytes[index]{b'%' if index+2<bytes.len()=>{if let Ok(text)=std::str::from_utf8(&bytes[index+1..index+3]){if let Ok(decoded)=u8::from_str_radix(text,16){output.push(decoded);index+=3;continue;}}output.push(bytes[index]);index+=1},b'+'=>{output.push(b' ');index+=1},value=>{output.push(value);index+=1}}}String::from_utf8_lossy(&output).to_string()}
// Was: Diese Funktion schreibt response.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_response(stream:&mut TcpStream,response:HttpResponse)->std::io::Result<()>{let reason=match response.status{200=>"OK",201=>"Created",202=>"Accepted",204=>"No Content",400=>"Bad Request",404=>"Not Found",409=>"Conflict",413=>"Payload Too Large",500=>"Internal Server Error",503=>"Service Unavailable",_=>"OK"};let mut headers=format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n",response.status,reason,response.content_type,response.body.len());if let Some(disposition)=response.disposition{headers.push_str(&format!("Content-Disposition: {disposition}\r\n"));}for(key,value)in response.extra_headers{headers.push_str(&format!("{key}: {value}\r\n"));}headers.push_str("\r\n");stream.write_all(headers.as_bytes())?;stream.write_all(&response.body)}
// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T:DeserializeOwned>(body:&[u8])->Result<T,String>{serde_json::from_slice(body).map_err(|error|format!("invalid JSON: {error}"))}
// Was: Diese Funktion liest und prüft JSON-Daten or default.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json_or_default<T:DeserializeOwned+Default>(body:&[u8])->Result<T,String>{if body.is_empty(){Ok(T::default())}else{parse_json(body)}}
// Was: Führt den Arbeitsschritt `query_usize` für query usize aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn query_usize(request:&HttpRequest,key:&str,default:usize,max:usize)->usize{request.query.get(key).and_then(|value|value.parse::<usize>().ok()).unwrap_or(default).clamp(1,max)}
// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T:Serialize>(status:u16,value:&T)->HttpResponse{HttpResponse{status,content_type:"application/json; charset=utf-8",body:serde_json::to_vec_pretty(value).unwrap_or_else(|_|b"{}".to_vec()),disposition:None,extra_headers:Vec::new()}}
// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value:&str)->HttpResponse{HttpResponse{status:200,content_type:"text/html; charset=utf-8",body:value.as_bytes().to_vec(),disposition:None,extra_headers:vec![("Content-Security-Policy".to_string(),"default-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'; frame-src http:; connect-src 'self' http:".to_string())]}}
// Was: Führt den Arbeitsschritt `text` für text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn text(content_type:&'static str,value:String)->HttpResponse{HttpResponse{status:200,content_type,body:value.into_bytes(),disposition:None,extra_headers:Vec::new()}}
// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status:u16)->HttpResponse{HttpResponse{status,content_type:"text/plain; charset=utf-8",body:Vec::new(),disposition:None,extra_headers:Vec::new()}}
// Was: Führt den Arbeitsschritt `conflict` für conflict aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn conflict(error:String)->HttpResponse{json_response(409,&json!({"error":error}))}
// Was: Führt den Arbeitsschritt `not_found` für not found aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn not_found(error:String)->HttpResponse{json_response(404,&json!({"error":error}))}
// Was: Führt den Arbeitsschritt `download` für download aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download(name:&str,content_type:&'static str,body:Vec<u8>)->HttpResponse{HttpResponse{status:200,content_type,body,disposition:Some(format!("attachment; filename=\"{}\"",name.replace('"',""))),extra_headers:Vec::new()}}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
