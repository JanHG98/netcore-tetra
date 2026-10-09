# TBS Connect

**Quellen:** [system-backend/tbs-connect](../../../system-backend/tbs-connect) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Dieser bestehende Backend-Baustein stellt eine Verbindung zwischen TBS und zentralen Diensten bereit. Seine langfristige Funktion wird in den geplanten Node Gateway überführt oder klar davon abgegrenzt.

## Geplante WebUI

Solange `tbs-connect` als eigenständiger Container betrieben wird, benötigt er eine eigene Verwaltungsoberfläche für Verbindungen, Sessions, Heartbeats, Fehler, Konfiguration und Wartung. Die Vorgaben stehen in `Docs/design/BACKEND_WEBUI_STANDARD.md`.
