//! Bounded HTTP/1 response framing for the small synchronous backend clients.
//! A complete Content-Length or chunked response does not require peer EOF.

use std::io::{BufRead, BufReader, Read};

const MAX_HEADER_BYTES: usize = 64 * 1024;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

pub fn read_response<R: Read>(reader: R, max_body_bytes: usize) -> Result<HttpResponse, String> {
    let mut reader = BufReader::new(reader);
    for _ in 0..8 {
        let mut header_bytes = 0;
        let status_line = read_line(&mut reader, &mut header_bytes)?;
        let mut parts = status_line.split_whitespace();
        let version = parts.next().unwrap_or_default();
        let code = parts.next().unwrap_or_default();
        if !matches!(version, "HTTP/1.0" | "HTTP/1.1")
            || code.len() != 3
            || !code.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err("invalid HTTP status line".to_string());
        }
        let status = code
            .parse::<u16>()
            .map_err(|_| "invalid HTTP status".to_string())?;
        if !(100..600).contains(&status) {
            return Err("invalid HTTP status".to_string());
        }
        let mut content_length = None;
        let mut transfer_encoding = None;
        loop {
            let line = read_line(&mut reader, &mut header_bytes)?;
            if line == "\r\n" {
                break;
            }
            let (name, value) = line
                .split_once(':')
                .ok_or_else(|| "invalid HTTP header".to_string())?;
            let value = value.trim();
            if name.eq_ignore_ascii_case("content-length") {
                let length = value
                    .parse::<usize>()
                    .map_err(|_| "invalid Content-Length".to_string())?;
                if content_length.is_some_and(|previous| previous != length) {
                    return Err("conflicting Content-Length headers".to_string());
                }
                content_length = Some(length);
            } else if name.eq_ignore_ascii_case("transfer-encoding") {
                if transfer_encoding.is_some() {
                    return Err("multiple Transfer-Encoding headers".to_string());
                }
                transfer_encoding = Some(value.to_ascii_lowercase());
            }
        }
        if (100..200).contains(&status) {
            if status == 101 {
                return Err("unexpected HTTP protocol upgrade".to_string());
            }
            continue;
        }
        if matches!(status, 204 | 304) {
            return Ok(HttpResponse {
                status,
                body: Vec::new(),
            });
        }
        let body = if let Some(encoding) = transfer_encoding {
            if content_length.is_some() || encoding != "chunked" {
                return Err("unsupported or ambiguous HTTP response framing".to_string());
            }
            read_chunked(&mut reader, max_body_bytes)?
        } else if let Some(length) = content_length {
            if length > max_body_bytes {
                return Err(format!("response body exceeds {max_body_bytes} bytes"));
            }
            let mut body = vec![0; length];
            reader
                .read_exact(&mut body)
                .map_err(|error| format!("read HTTP body: {error}"))?;
            body
        } else {
            let mut body = Vec::new();
            reader
                .take((max_body_bytes as u64).saturating_add(1))
                .read_to_end(&mut body)
                .map_err(|error| format!("read HTTP body: {error}"))?;
            if body.len() > max_body_bytes {
                return Err(format!("response body exceeds {max_body_bytes} bytes"));
            }
            body
        };
        return Ok(HttpResponse { status, body });
    }
    Err("too many interim HTTP responses".to_string())
}

fn read_line<R: BufRead>(reader: &mut R, budget: &mut usize) -> Result<String, String> {
    let remaining = MAX_HEADER_BYTES.saturating_sub(*budget);
    let mut line = Vec::new();
    let count = reader
        .take((remaining as u64).saturating_add(1))
        .read_until(b'\n', &mut line)
        .map_err(|error| format!("read HTTP headers: {error}"))?;
    *budget += count;
    if *budget > MAX_HEADER_BYTES {
        return Err("HTTP headers exceed limit".to_string());
    }
    if !line.ends_with(b"\r\n") {
        return Err("truncated HTTP headers".to_string());
    }
    String::from_utf8(line).map_err(|_| "HTTP headers are not UTF-8".to_string())
}

fn read_chunked<R: BufRead>(reader: &mut R, max_body_bytes: usize) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    let mut framing_bytes = 0;
    loop {
        let line = read_line(reader, &mut framing_bytes)?;
        let size = usize::from_str_radix(line.split(';').next().unwrap_or_default().trim(), 16)
            .map_err(|_| "invalid HTTP chunk size".to_string())?;
        if size == 0 {
            loop {
                let trailer = read_line(reader, &mut framing_bytes)?;
                if trailer == "\r\n" {
                    return Ok(body);
                }
                if !trailer.contains(':') {
                    return Err("invalid HTTP trailer".to_string());
                }
            }
        }
        if size > max_body_bytes.saturating_sub(body.len()) {
            return Err(format!("response body exceeds {max_body_bytes} bytes"));
        }
        let start = body.len();
        body.resize(start + size, 0);
        reader
            .read_exact(&mut body[start..])
            .map_err(|error| format!("read HTTP chunk: {error}"))?;
        let mut end = [0; 2];
        reader
            .read_exact(&mut end)
            .map_err(|error| format!("read HTTP chunk end: {error}"))?;
        if end != *b"\r\n" {
            return Err("invalid HTTP chunk terminator".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::sync::mpsc;
    use std::time::Duration;

    fn persistent_response(response: &'static [u8]) -> Result<HttpResponse, String> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (release, hold) = mpsc::channel();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            // Deliberately keep the socket open after sending the full response.
            for fragment in response.chunks(7) {
                stream.write_all(fragment).unwrap();
            }
            let _ = hold.recv_timeout(Duration::from_secs(2));
        });
        let stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_millis(150)))
            .unwrap();
        let result = read_response(stream, 1024);
        let _ = release.send(());
        server.join().unwrap();
        result
    }

    #[test]
    fn complete_content_length_response_does_not_wait_for_close() {
        let response = persistent_response(
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\n{}",
        )
        .unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"{}");
    }

    #[test]
    fn old_read_to_eof_waited_past_a_complete_http_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (release, hold) = mpsc::channel();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}")
                .unwrap();
            let _ = hold.recv_timeout(Duration::from_secs(2));
        });
        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        let mut raw = Vec::new();
        let result = stream.read_to_end(&mut raw);
        let _ = release.send(());
        server.join().unwrap();
        assert!(
            result.is_err(),
            "waiting for EOF used to discard an otherwise complete reply"
        );
        assert!(raw.ends_with(b"{}"));
        assert_eq!(read_response(raw.as_slice(), 2).unwrap().body, b"{}");
    }

    #[test]
    fn complete_chunked_response_with_trailer_does_not_wait_for_close() {
        let response = persistent_response(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2;test=1\r\n{}\r\n0\r\nX-Test: done\r\n\r\n").unwrap();
        assert_eq!(response.body, b"{}");
    }

    #[test]
    fn unhealthy_http_status_is_preserved() {
        let response = persistent_response(
            b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 8\r\n\r\nnotready",
        )
        .unwrap();
        assert_eq!(response.status, 503);
        assert_eq!(response.body, b"notready");
    }

    #[test]
    fn no_content_response_does_not_wait_for_close() {
        let response = persistent_response(b"HTTP/1.1 204 No Content\r\n\r\n").unwrap();
        assert_eq!(response.status, 204);
        assert!(response.body.is_empty());
    }

    #[test]
    fn handles_interim_response_then_final_status() {
        let response = read_response(
            &b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\n{}"[..],
            2,
        )
        .unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"{}");
    }

    #[test]
    fn rejects_truncated_conflicting_and_oversized_responses() {
        for bytes in [
            &b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\n{}"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nContent-Length: 4\r\n\r\n{}"[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Length: 2\r\n\r\n0\r\n\r\n"
                [..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: 999999\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\n{}xx0\r\n\r\n"[..],
        ] {
            assert!(read_response(bytes, 16).is_err());
        }
    }

    #[test]
    fn reads_close_delimited_http10_and_rejects_oversized_body() {
        let raw = &b"HTTP/1.0 200 OK\r\n\r\nbody"[..];
        assert_eq!(read_response(raw, 4).unwrap().body, b"body");
        assert!(read_response(raw, 3).is_err());
    }

    #[test]
    fn rejects_oversized_unterminated_headers() {
        let raw = vec![b'x'; MAX_HEADER_BYTES + 1];
        assert!(
            read_response(raw.as_slice(), 1)
                .unwrap_err()
                .contains("headers exceed")
        );
    }
}
