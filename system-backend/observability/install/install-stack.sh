#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "install-stack.sh must run as root" >&2; exit 1; }
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -m 0755 /etc/prometheus/rules /etc/alertmanager /etc/loki /etc/promtail /etc/grafana/provisioning/datasources /etc/grafana/provisioning/dashboards /var/lib/grafana/dashboards
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/prometheus/prometheus.yml" /etc/prometheus/prometheus.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/prometheus/rules/netcore.rules.yml" /etc/prometheus/rules/netcore.rules.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/alertmanager/alertmanager.yml" /etc/alertmanager/alertmanager.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/loki/loki.yml" /etc/loki/loki.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/promtail/promtail.yml" /etc/promtail/promtail.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/grafana/provisioning/datasources/netcore.yml" /etc/grafana/provisioning/datasources/netcore.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/grafana/provisioning/dashboards/netcore.yml" /etc/grafana/provisioning/dashboards/netcore.yml
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "${ROOT}/system-backend/observability/stack/grafana/dashboards/netcore-overview.json" /var/lib/grafana/dashboards/netcore-overview.json
# Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
# Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
for unit in prometheus alertmanager loki promtail grafana-server; do
  # Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
  # Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
  if systemctl list-unit-files "${unit}.service" >/dev/null 2>&1; then systemctl enable --now "${unit}.service" || true; else echo "${unit}: no installed systemd unit, configuration staged only"; fi
done
echo "Stack configuration installed. No third-party binary was downloaded by this script."
