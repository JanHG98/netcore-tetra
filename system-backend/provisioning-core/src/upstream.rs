use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct UpstreamResponse {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone)]
struct HttpTarget {
    host_header: String,
    connect_host: String,
    port: u16,
    base_path: String,
}

pub fn request(
    base_url: &str,
    method: &str,
    path: &str,
    body: &[u8],
    timeout: Duration,
) -> Result<UpstreamResponse, String> {
    let target = parse_http_url(base_url)?;
    let address = format!("{}:{}", target.connect_host, target.port)
        .to_socket_addrs()
        .map_err(|error| format!("DNS lookup failed for {}: {error}", target.connect_host))?
        .next()
        .ok_or_else(|| format!("no address found for {}", target.connect_host))?;
    let mut stream = TcpStream::connect_timeout(&address, timeout)
        .map_err(|error| format!("connect to {base_url} failed: {error}"))?;
    stream.set_read_timeout(Some(timeout)).map_err(|error| error.to_string())?;
    stream.set_write_timeout(Some(timeout)).map_err(|error| error.to_string())?;

    let path = format!("{}{}", target.base_path, if path.starts_with('/') { path.to_string() } else { format!("/{path}") });
    let content_type = if body.is_empty() { "application/octet-stream" } else { "application/json" };
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        target.host_header,
        body.len()
    );
    stream.write_all(request.as_bytes()).map_err(|error| error.to_string())?;
    if !body.is_empty() {
        stream.write_all(body).map_err(|error| error.to_string())?;
    }
    stream.flush().map_err(|error| error.to_string())?;

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|error| error.to_string())?;
    parse_response(&raw)
}

fn parse_http_url(url: &str) -> Result<HttpTarget, String> {
    let rest = url.strip_prefix("http://").ok_or_else(|| "only http:// upstream URLs are supported".to_string())?;
    let (authority, base_path) = rest.split_once('/').map_or((rest, String::new()), |(a, p)| (a, format!("/{p}")));
    let (host, port) = if authority.starts_with('[') {
        let end = authority.find(']').ok_or_else(|| "invalid IPv6 URL".to_string())?;
        let host = authority[1..end].to_string();
        let port = authority[end + 1..].strip_prefix(':').map_or(Ok(80), |v| v.parse::<u16>().map_err(|_| "invalid URL port".to_string()))?;
        (host, port)
    } else if let Some((host, port)) = authority.rsplit_once(':') {
        (host.to_string(), port.parse::<u16>().map_err(|_| "invalid URL port".to_string())?)
    } else {
        (authority.to_string(), 80)
    };
    if host.is_empty() {
        return Err("upstream URL has no host".into());
    }
    Ok(HttpTarget { host_header: authority.to_string(), connect_host: host, port, base_path })
}

fn parse_response(raw: &[u8]) -> Result<UpstreamResponse, String> {
    let split = raw.windows(4).position(|window| window == b"\r\n\r\n").ok_or_else(|| "invalid HTTP response".to_string())?;
    let head = String::from_utf8_lossy(&raw[..split]);
    let mut lines = head.lines();
    let status_line = lines.next().ok_or_else(|| "missing HTTP status".to_string())?;
    let status = status_line.split_whitespace().nth(1).ok_or_else(|| "missing HTTP status code".to_string())?.parse::<u16>().map_err(|_| "invalid HTTP status code".to_string())?;
    let mut content_type = "application/json; charset=utf-8".to_string();
    let mut chunked = false;
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-type") {
                content_type = value.trim().to_string();
            }
            if name.eq_ignore_ascii_case("transfer-encoding") && value.to_ascii_lowercase().contains("chunked") {
                chunked = true;
            }
        }
    }
    let body = raw[split + 4..].to_vec();
    let body = if chunked { decode_chunked(&body)? } else { body };
    Ok(UpstreamResponse { status, content_type, body })
}

fn decode_chunked(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut cursor = 0usize;
    loop {
        let line_end = input[cursor..].windows(2).position(|w| w == b"\r\n").ok_or_else(|| "invalid chunked response".to_string())? + cursor;
        let size_text = std::str::from_utf8(&input[cursor..line_end]).map_err(|_| "invalid chunk size".to_string())?;
        let size = usize::from_str_radix(size_text.split(';').next().unwrap_or("0").trim(), 16).map_err(|_| "invalid chunk size".to_string())?;
        cursor = line_end + 2;
        if size == 0 { break; }
        if cursor + size + 2 > input.len() { return Err("truncated chunked response".into()); }
        output.extend_from_slice(&input[cursor..cursor + size]);
        cursor += size + 2;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_http_target_with_port_and_base_path() {
        let target = parse_http_url("http://10.0.1.181:8100/base").unwrap();
        assert_eq!(target.connect_host, "10.0.1.181");
        assert_eq!(target.port, 8100);
        assert_eq!(target.base_path, "/base");
    }

    #[test]
    fn parses_chunked_response() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\n\r\n7\r\n{\"a\":1}\r\n0\r\n\r\n";
        let response = parse_response(raw).unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"{\"a\":1}");
    }
}
