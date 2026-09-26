# Konfiguration der TBS

Die Basisstation nutzt TOML mit `config_version = "0.6"` und `stack_mode = "Bs"` im aktuellen [sanitisierten Beispiel](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml). Die tatsächlich installierte Datei ist entscheidend; sie kann von `main` abweichen. Das Beispiel enthält **Dummy-Zugangsdaten und Beispiel-IP-Adressen** und darf nicht unverändert als Betriebsdatei verwendet werden.

## Konfigurationsbereiche

| Abschnitt | Inhalt | Nach Änderung prüfen |
|---|---|---|
| `[phy_io]`, `[phy_io.soapysdr]` | SDR/Treiber, Gain, Sample-Rate, Center | RX/TX, Timing, Passband, Spektrum |
| `[net_info]`, `[cell_info]` | MCC/MNC, Carrier, LA/CC, Registrierung, Rufe, Paketdaten | Endgeräte-Camping, Affiliation, Call/Release |
| `[recovery]`, `[health]` | Replay, Re-Attract, Health und Watchdog | Join-Verhalten, keine Neustartschleife |
| `[dashboard]` | Bind/Port, Anmeldung, Systemaktionen | Listener und Netzwerkgrenze |
| `[recording]`, `[audio_player]`, `[media_library]`, `[tts]` | Aufnahme, Datei- und Medienpfade, Piper | Rechte, Cache, Ruf/Release, NFS |
| `[netcore_directory]` | URL und Timeout des Directory | Namen/Status ohne Funkblockade |
| `[control_room]`, `[edge_fallback]` | **Node Gateway** und lokale Autonomie | `/ws/node`, Service-Matrix und Rückkehr |
| `[asterisk]`, `[brew]`, `[telegram_alerts]`, `[wx_service]` | optionale Anbindungen | Zielsystem, Zugang und Routing |

## Datenform und Werte

```toml
config_version = "0.6"
stack_mode = "Bs"

[control_room]
enabled = true
host = "<NODE-GATEWAY-IP>"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "TBS-LAB-01"
```

Das ist ein **Ausschnitt**, keine vollständige lauffähige TBS-Datei. Bei einem lokalen Einzelknoten `[control_room].enabled = false` setzen. Die Management-IP des Gateway ist nicht automatisch die Control-Room-IP `9010`. Konfigurations-Drift zwischen eingecheckter `config.toml` und Inventory kann zu irreführenden Verbindungsfehlern führen. [[Architecture]] · [[Netzwerk-und-Ports]]

## Carrier und SDR

`main_carrier` bezeichnet den Hauptträger; `dual_carrier_enabled` und `secondary_carrier` erweitern den RF-Pfad. `tx_center_freq` und `rx_center_freq` bezeichnen **SDR-Mittenfrequenzen**, nicht zwingend die exakt programmierte Trägerfrequenz. Sample-Rate, Filterreserve und Slot-Plan müssen zusammenpassen. [[Dual-Carrier]] · [[Hardware-und-RF]]

## Recovery, Status und Steuer-ISSI

Proaktives Replay und reaktives Re-Attract sind getrennte Mechanismen. `[cell_info.sds_command_control]` definiert autorisierte ISSIs und Aktionen für U-STATUS-Systembefehle; die Anwesenheit dieser Sektion ist relevant, ein frei erfundenes `enabled`-Feld nicht. `4010001` ist die im Beispiel verwendete System-ISSI, **kein MQTT-Topic**. Konfigurationswerte und Endgeräteprogrammierung aufeinander abstimmen. [[Registration-and-Affiliation]] · [[SDS-and-U-STATUS]]

## Änderungen sicher prüfen

```bash
sudo cp /etc/netcore/config.toml /etc/netcore/config.toml.pre-change
python3 -c 'import pathlib,tomllib; tomllib.loads(pathlib.Path("/etc/netcore/config.toml").read_text()); print("TOML OK")'
sudo systemctl restart tetra.service
sudo journalctl -u tetra.service -b -n 200 --no-pager
```

Systemd-Unit-Namen und Pfad anpassen. Beim Start ausdrücklich prüfen, ob **die Primärdatei oder `.fallback`** geladen wurde. Kein Passwort oder Token in Wiki, Tickets, Screenshots oder Logauszüge übernehmen. [[Backup-and-Fallback]]
