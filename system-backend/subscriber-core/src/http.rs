// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Teilnehmerdaten, Berechtigungen und Registrierung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::SubscriberCoreConfig;
use crate::protocol::BackendRequest;
use crate::state::{ImportRequest, SharedSubscribers, SubscriberInput};

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: SubscriberCoreConfig,
    subscribers: SharedSubscribers,
    gateway_tx: Sender<BackendRequest>,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Subscriber Core WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => {
                    let subscribers = subscribers.clone();
                    let gateway_tx = gateway_tx.clone();
                    let config = config.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, subscribers, gateway_tx, config) {
                            tracing::warn!("HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("HTTP accept failed: {}", error),
            }
        }
    }))
}

// Was: Bündelt die zusammengehörigen Werte für HTTP request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpRequest { method: String, path: String, query: HashMap<String,String>, body: Vec<u8> }
// Was: Bündelt die zusammengehörigen Werte für HTTP response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResponse { status: u16, content_type: &'static str, body: Vec<u8>, disposition: Option<String> }

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(mut stream: TcpStream, subscribers: SharedSubscribers, gateway_tx: Sender<BackendRequest>, config: SubscriberCoreConfig) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, subscribers, gateway_tx, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(request: HttpRequest, subscribers: SharedSubscribers, gateway_tx: Sender<BackendRequest>, config: SubscriberCoreConfig) -> HttpResponse {
    if request.method == "OPTIONS" { return empty(204); }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Subscriber Core", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = subscribers.status();
            let code = if status.node_gateway_connected {200} else {503}; json_response(code,&status)
        }
        ("GET", "/api/v1/status") => json_response(200,&subscribers.status()),
        ("GET", "/api/v1/nodes") => json_response(200,&subscribers.nodes()),
        ("GET", "/api/v1/subscribers") => json_response(200,&subscribers.subscribers()),
        ("GET", "/api/v1/observed") => json_response(200,&subscribers.observed()),
        ("GET", "/api/v1/syncs") => json_response(200,&subscribers.syncs()),
        ("GET", "/api/v1/events") => {
            let limit=request.query.get("limit").and_then(|v|v.parse::<usize>().ok()).unwrap_or(100).min(1000);
            json_response(200,&subscribers.recent_events(limit))
        }
        ("GET", "/api/v1/config") => json_response(200,&config),
        ("GET", "/api/v1/export.json") => download_json("netcore-subscribers.json",&subscribers.export_database()),
        ("GET", "/api/v1/export.csv") => download("text/csv; charset=utf-8","netcore-subscribers.csv",subscribers.export_csv().into_bytes()),
        ("GET", "/metrics") => text("text/plain; version=0.0.4; charset=utf-8",subscribers.metrics()),
        ("GET", "/openapi.json") => json_response(200,&openapi()),
        ("POST", "/api/v1/subscribers") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<SubscriberInput>(&request.body).and_then(|input| subscribers.create_subscriber(input)) {
                Ok((profile,commands)) => match dispatch_all(&gateway_tx,commands) { Ok(())=>json_response(201,&profile), Err(e)=>json_response(503,&json!({"error":e})) },
                Err(e)=>json_response(409,&json!({"error":e})),
            }
        }
        ("POST", "/api/v1/import") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<ImportRequest>(&request.body).and_then(|input| subscribers.import_subscribers(input)) {
                Ok((count,commands)) => match dispatch_all(&gateway_tx,commands) { Ok(())=>json_response(200,&json!({"imported":count})), Err(e)=>json_response(503,&json!({"error":e})) },
                Err(e)=>json_response(409,&json!({"error":e})),
            }
        }
        ("POST", "/api/v1/sync") => {
            let commands=subscribers.sync_all(); let count=commands.len();
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match dispatch_all(&gateway_tx,commands) { Ok(())=>json_response(202,&json!({"queued":count})), Err(e)=>json_response(503,&json!({"error":e})) }
        }
        _ if request.path.starts_with("/api/v1/subscribers/") => subscriber_route(request,subscribers,gateway_tx),
        _ => json_response(404,&json!({"error":"not found"})),
    }
}

// Was: Führt den Arbeitsschritt `subscriber_route` für Teilnehmer Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn subscriber_route(request: HttpRequest, subscribers: SharedSubscribers, gateway_tx: Sender<BackendRequest>) -> HttpResponse {
    let tail=request.path.trim_start_matches("/api/v1/subscribers/");
    let Ok(issi)=tail.parse::<u32>() else { return json_response(400,&json!({"error":"invalid ISSI"})); };
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match request.method.as_str() {
        "GET" => subscribers.subscriber(issi).map_or_else(||json_response(404,&json!({"error":"not found"})),|p|json_response(200,&p)),
        "PUT" => match parse_json::<SubscriberInput>(&request.body).and_then(|input|subscribers.update_subscriber(issi,input)) {
            Ok((profile,commands))=>match dispatch_all(&gateway_tx,commands){Ok(())=>json_response(200,&profile),Err(e)=>json_response(503,&json!({"error":e}))},
            Err(e)=>json_response(409,&json!({"error":e})),
        },
        "DELETE" => match subscribers.delete_subscriber(issi) {
            Ok(commands)=>match dispatch_all(&gateway_tx,commands){Ok(())=>empty(204),Err(e)=>json_response(503,&json!({"error":e}))},
            Err(e)=>json_response(404,&json!({"error":e})),
        },
        _=>json_response(405,&json!({"error":"method not allowed"})),
    }
}

// Was: Diese Funktion verteilt all.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dispatch_all(tx:&Sender<BackendRequest>,commands:Vec<BackendRequest>)->Result<(),String>{for command in commands{tx.send(command).map_err(|_|"node gateway worker is unavailable".to_string())?;}Ok(())}
// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T:serde::de::DeserializeOwned>(body:&[u8])->Result<T,String>{serde_json::from_slice(body).map_err(|e|format!("invalid JSON: {e}"))}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi()->serde_json::Value{json!({"openapi":"3.0.3","info":{"title":"NetCore Subscriber Core","version":env!("CARGO_PKG_VERSION"),"description":"OPEN LAB API. No authentication, no token and no TLS."},"paths":{"/api/v1/status":{"get":{}},"/api/v1/subscribers":{"get":{},"post":{}},"/api/v1/subscribers/{issi}":{"get":{},"put":{},"delete":{}},"/api/v1/observed":{"get":{}},"/api/v1/nodes":{"get":{}},"/api/v1/syncs":{"get":{}},"/api/v1/sync":{"post":{}},"/api/v1/import":{"post":{}},"/api/v1/export.json":{"get":{}},"/api/v1/export.csv":{"get":{}},"/health/live":{"get":{}},"/health/ready":{"get":{}},"/metrics":{"get":{}}}})}

// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T:Serialize>(status:u16,value:&T)->HttpResponse{match serde_json::to_vec_pretty(value){Ok(body)=>HttpResponse{status,content_type:"application/json; charset=utf-8",body,disposition:None},Err(e)=>HttpResponse{status:500,content_type:"application/json; charset=utf-8",body:format!("{{\"error\":\"serialization failed: {e}\"}}").into_bytes(),disposition:None}}}
// Was: Führt den Arbeitsschritt `download_json` für download JSON-Daten aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download_json<T:Serialize>(name:&str,value:&T)->HttpResponse{match serde_json::to_vec_pretty(value){Ok(body)=>download("application/json; charset=utf-8",name,body),Err(e)=>json_response(500,&json!({"error":e.to_string()}))}}
// Was: Führt den Arbeitsschritt `download` für download aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download(content_type:&'static str,name:&str,body:Vec<u8>)->HttpResponse{HttpResponse{status:200,content_type,body,disposition:Some(format!("attachment; filename=\"{name}\""))}}
// Was: Führt den Arbeitsschritt `text` für text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn text(content_type:&'static str,value:String)->HttpResponse{HttpResponse{status:200,content_type,body:value.into_bytes(),disposition:None}}
// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value:&str)->HttpResponse{HttpResponse{status:200,content_type:"text/html; charset=utf-8",body:value.as_bytes().to_vec(),disposition:None}}
// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status:u16)->HttpResponse{HttpResponse{status,content_type:"text/plain; charset=utf-8",body:Vec::new(),disposition:None}}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream:&mut TcpStream,max_body_bytes:usize)->Result<HttpRequest,String>{let mut buffer=Vec::new();let mut chunk=[0u8;4096];let header_end=loop{let read=stream.read(&mut chunk).map_err(|e|format!("request read failed: {e}"))?;if read==0{return Err("connection closed before request was complete".into())}buffer.extend_from_slice(&chunk[..read]);if buffer.len()>max_body_bytes+65536{return Err("request too large".into())}if let Some(pos)=find_subslice(&buffer,b"\r\n\r\n"){break pos+4}};let header_text=std::str::from_utf8(&buffer[..header_end]).map_err(|_|"request headers are not utf-8".to_string())?;let mut lines=header_text.split("\r\n");let request_line=lines.next().ok_or_else(||"missing request line".to_string())?;let mut parts=request_line.split_whitespace();let method=parts.next().ok_or_else(||"missing method".to_string())?.to_ascii_uppercase();let raw_path=parts.next().ok_or_else(||"missing path".to_string())?;let(path,query)=parse_path_and_query(raw_path);let mut content_length=0usize;for line in lines{if let Some((name,value))=line.split_once(':'){if name.eq_ignore_ascii_case("content-length"){content_length=value.trim().parse().map_err(|_|"invalid content-length".to_string())?;}}}if content_length>max_body_bytes{return Err("body too large".into())}let mut body=buffer[header_end..].to_vec();while body.len()<content_length{let read=stream.read(&mut chunk).map_err(|e|format!("body read failed: {e}"))?;if read==0{return Err("connection closed before body was complete".into())}body.extend_from_slice(&chunk[..read]);if body.len()>max_body_bytes{return Err("body too large".into())}}body.truncate(content_length);Ok(HttpRequest{method,path,query,body})}
// Was: Diese Funktion schreibt response.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_response(stream:&mut TcpStream,response:HttpResponse)->std::io::Result<()>{let disposition=response.disposition.map(|v|format!("Content-Disposition: {v}\r\n")).unwrap_or_default();let header=format!("HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nX-NetCore-Security-Mode: open-lab\r\nConnection: close\r\n\r\n",response.status,reason_phrase(response.status),response.content_type,response.body.len(),disposition);stream.write_all(header.as_bytes())?;stream.write_all(&response.body)?;stream.flush()}
// Was: Diese Funktion sucht subslice.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_subslice(h:&[u8],n:&[u8])->Option<usize>{h.windows(n.len()).position(|w|w==n)}
// Was: Diese Funktion liest und prüft path and query.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_path_and_query(raw:&str)->(String,HashMap<String,String>){let mut parts=raw.splitn(2,'?');let path=parts.next().unwrap_or(raw).to_string();let query=parts.next().map(|q|q.split('&').filter(|p|!p.is_empty()).filter_map(|p|{let mut v=p.splitn(2,'=');Some((v.next()?.to_string(),v.next().unwrap_or("").to_string()))}).collect()).unwrap_or_default();(path,query)}
// Was: Führt den Arbeitsschritt `reason_phrase` für reason phrase aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn reason_phrase(status:u16)->&'static str{match status{200=>"OK",201=>"Created",202=>"Accepted",204=>"No Content",400=>"Bad Request",404=>"Not Found",405=>"Method Not Allowed",409=>"Conflict",413=>"Payload Too Large",500=>"Internal Server Error",503=>"Service Unavailable",_=>"Response"}}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
