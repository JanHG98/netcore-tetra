# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für websocket.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import base64
import hashlib
import os
import socket
import ssl
import struct
from dataclasses import dataclass
from urllib.parse import urlparse


# Was: Bündelt Daten und Verhalten für Weboberfläche socket error.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class WebSocketError(RuntimeError):
    pass


# Was: Bündelt Daten und Verhalten für ws Nachricht.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass
class WsMessage:
    opcode: int
    payload: bytes

    # Was: Führt den Arbeitsschritt `text` für text aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def text(self) -> str:
        return self.payload.decode("utf-8", errors="replace")


# Was: Bündelt Daten und Verhalten für Weboberfläche socket client.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class WebSocketClient:
    """Small RFC6455 client used by the E2E TBS simulator.

    It intentionally implements only the subset needed by the Node Gateway:
    non-fragmented text/binary frames plus ping/pong and close.
    """

    # Was: Diese Funktion initialisiert den vorgesehenen Arbeitsschritt.
    # Warum: Alle benötigten Startwerte werden so in einer festen Reihenfolge eingerichtet.
    def __init__(self, url: str, *, subprotocol: str | None = None, timeout: float = 8.0) -> None:
        self.url = url
        self.subprotocol = subprotocol
        self.timeout = timeout
        self.sock: socket.socket | None = None

    # Was: Diese Funktion verbindet den vorgesehenen Arbeitsschritt.
    # Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    def connect(self) -> None:
        parsed = urlparse(self.url)
        if parsed.scheme not in {"ws", "wss"}:
            raise WebSocketError(f"unsupported WebSocket scheme: {parsed.scheme}")
        host = parsed.hostname or "127.0.0.1"
        port = parsed.port or (443 if parsed.scheme == "wss" else 80)
        path = parsed.path or "/"
        if parsed.query:
            path += "?" + parsed.query
        sock = socket.create_connection((host, port), timeout=self.timeout)
        if parsed.scheme == "wss":
            context = ssl.create_default_context()
            sock = context.wrap_socket(sock, server_hostname=host)
        sock.settimeout(self.timeout)
        key = base64.b64encode(os.urandom(16)).decode("ascii")
        headers = [
            f"GET {path} HTTP/1.1",
            f"Host: {host}:{port}",
            "Upgrade: websocket",
            "Connection: Upgrade",
            f"Sec-WebSocket-Key: {key}",
            "Sec-WebSocket-Version: 13",
            "User-Agent: netcore-open-lab-e2e/1",
        ]
        if self.subprotocol:
            headers.append(f"Sec-WebSocket-Protocol: {self.subprotocol}")
        sock.sendall(("\r\n".join(headers) + "\r\n\r\n").encode("ascii"))
        response = self._read_http_headers(sock)
        status_line = response.split("\r\n", 1)[0]
        if " 101 " not in status_line:
            sock.close()
            raise WebSocketError(f"WebSocket upgrade rejected: {status_line}; response={response[:1000]!r}")
        accept = None
        selected_protocol = None
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for line in response.split("\r\n")[1:]:
            if ":" not in line:
                continue
            name, value = line.split(":", 1)
            if name.lower() == "sec-websocket-accept":
                accept = value.strip()
            elif name.lower() == "sec-websocket-protocol":
                selected_protocol = value.strip()
        expected = base64.b64encode(hashlib.sha1((key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode("ascii")).digest()).decode("ascii")
        if accept != expected:
            sock.close()
            raise WebSocketError("invalid Sec-WebSocket-Accept from server")
        if self.subprotocol and selected_protocol not in {None, self.subprotocol}:
            sock.close()
            raise WebSocketError(f"unexpected subprotocol {selected_protocol!r}")
        self.sock = sock

    # Was: Diese Funktion liest HTTP headers.
    # Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
    @staticmethod
    def _read_http_headers(sock: socket.socket) -> str:
        data = bytearray()
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        while b"\r\n\r\n" not in data:
            chunk = sock.recv(4096)
            if not chunk:
                raise WebSocketError("connection closed during WebSocket handshake")
            data.extend(chunk)
            if len(data) > 65536:
                raise WebSocketError("oversized WebSocket handshake response")
        return bytes(data).decode("iso-8859-1", errors="replace")

    # Was: Diese Funktion sendet text.
    # Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    def send_text(self, text: str) -> None:
        self._send_frame(0x1, text.encode("utf-8"))

    # Was: Diese Funktion sendet binary.
    # Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    def send_binary(self, payload: bytes) -> None:
        self._send_frame(0x2, payload)

    # Was: Diese Funktion sendet pong.
    # Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    def send_pong(self, payload: bytes = b"") -> None:
        self._send_frame(0xA, payload)

    # Was: Diese Funktion schließt den vorgesehenen Arbeitsschritt.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def close(self) -> None:
        if self.sock is None:
            return
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            self._send_frame(0x8, b"")
        except OSError:
            pass
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            self.sock.close()
        finally:
            self.sock = None

    # Was: Diese Funktion sendet Funkrahmen.
    # Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    def _send_frame(self, opcode: int, payload: bytes) -> None:
        if self.sock is None:
            raise WebSocketError("WebSocket is not connected")
        mask = os.urandom(4)
        first = 0x80 | (opcode & 0x0F)
        length = len(payload)
        if length < 126:
            header = bytes([first, 0x80 | length])
        elif length <= 0xFFFF:
            header = bytes([first, 0x80 | 126]) + struct.pack("!H", length)
        else:
            header = bytes([first, 0x80 | 127]) + struct.pack("!Q", length)
        masked = bytes(value ^ mask[index % 4] for index, value in enumerate(payload))
        self.sock.sendall(header + mask + masked)

    # Was: Führt den Arbeitsschritt `recv` für recv aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def recv(self) -> WsMessage:
        if self.sock is None:
            raise WebSocketError("WebSocket is not connected")
        first_two = self._read_exact(2)
        first, second = first_two
        final = bool(first & 0x80)
        opcode = first & 0x0F
        if not final:
            raise WebSocketError("fragmented WebSocket frames are not supported by the E2E client")
        masked = bool(second & 0x80)
        length = second & 0x7F
        if length == 126:
            length = struct.unpack("!H", self._read_exact(2))[0]
        elif length == 127:
            length = struct.unpack("!Q", self._read_exact(8))[0]
        mask = self._read_exact(4) if masked else b""
        payload = self._read_exact(length)
        if masked:
            payload = bytes(value ^ mask[index % 4] for index, value in enumerate(payload))
        return WsMessage(opcode=opcode, payload=payload)

    # Was: Diese Funktion liest exact.
    # Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
    def _read_exact(self, length: int) -> bytes:
        assert self.sock is not None
        chunks = bytearray()
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        while len(chunks) < length:
            chunk = self.sock.recv(length - len(chunks))
            if not chunk:
                raise WebSocketError("WebSocket peer closed the connection")
            chunks.extend(chunk)
        return bytes(chunks)
