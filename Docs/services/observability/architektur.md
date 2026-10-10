# Betriebsüberwachung: Architektur

## Verantwortungsgrenze

Observability beobachtet alle Core-Dienste, wird aber niemals fachlicher Eigentümer ihrer Teilnehmer-, Gruppen-, Mobility-, Call-, SDS-, Packet- oder Schlüsselzustände. Der Dienst darf bei Ausfall weder Call Control noch Media oder TBS Edge blockieren.

## Zwei Ebenen

1. Der interne Rust-Collector liefert sofort nutzbare Zielzustände, bounded Zeitreihen, JSON-Logs, Trace-Spans, Alarmregeln, Silence/Acknowledge, Audit und Diagnose.
2. Prometheus, Grafana, Loki/Promtail und Alertmanager sind als optionaler klassischer Stack vorbereitet. Sie sind getrennte Prozesse; der Stack-Installer aktiviert nur bereits vorhandene Binaries.
3. Die aktuelle Systemlog-Pipeline verwendet einen eigenen rsyslog-/RELP-Empfänger, begrenzte lokale Segmente, NMS-Vorschau und ein tägliches gzip-Share-Archiv. Sie benötigt weder Promtail noch Loki und bleibt von Controller und NMS-Prozess unabhängig.

## Persistenz

Der Managementzustand wird atomar als JSON geschrieben. Große Langzeitmengen gehören in Prometheus/Loki, nicht in `state.json`. Interne Daten bleiben durch Retention und Mengenlimits begrenzt.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/observability) · [Konfigurationsvorlagen](../../../system-backend/observability/config).
