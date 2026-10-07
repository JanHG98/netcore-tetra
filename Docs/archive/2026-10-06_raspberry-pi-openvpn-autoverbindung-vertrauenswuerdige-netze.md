# Brainstorming: Raspberry Pi OS – OpenVPN-Autoverbindung abhängig von vertrauenswürdigen Netzen

Stand der Repository- und Quellenprüfung: **06.10.2026**. Historische Ergebnisse beziehen sich auf die jeweils genannten Daten und Commits.

Entwurf für einen mobilen Raspberry Pi: In benannten vertrauenswürdigen WLANs oder im physischen Heim-LAN `10.0.1.0/24` bleibt der Heim-VPN-Tunnel aus; außerhalb wird er automatisch aufgebaut. Vorgesehen sind ein NetworkManager-OpenVPN-Profil, eine Bash-Policy, ein systemd-Oneshot und ein NetworkManager-Dispatcher. Ein nftables-Kill-Switch ist eine optionale Erweiterung. Installation, erfolgreicher Test und Betrieb sind nicht belegt.

Die am 06.10.2026 gelesenen Repository-Dateien enthalten eine separate historische Imagebuilder-/VPN-Vorschau und allgemeine WLAN-/Firewall-Komponenten. Der dazugehörige `deployment-core` und die hier vorgeschlagenen Policy-Dateien sind im geprüften `Archiving`-Stand nicht vorhanden. Diese unterschiedlichen Quellenstände werden unten ausdrücklich getrennt.

## 1. Projektstand und Quellenbasis

| Feld | Wert |
|---|---|
| Technischer Arbeitszeitraum | 25.09.2026, 20:36–20:38 Uhr MESZ, Europe/Berlin; Zeitangaben aus dem Entwurfabruf |
| Erstellung und Repository-Prüfung | 2026-10-06, Europe/Berlin |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Geprüfter Branch | `Archiving` |
| Geprüfter Basiscommit vor der Dokumentationsänderung | [`0f87d409d400376385013754418aaacfe04a830a`](https://github.com/JanHG98/netcore-tetra/commit/0f87d409d400376385013754418aaacfe04a830a) |
| Betreff des Basiscommits | `docs: archive TBS container updates and KatWarn release history` |
| Archivdatei | `Docs/archive/2026-10-06_raspberry-pi-openvpn-autoverbindung-vertrauenswuerdige-netze.md` |
| Archivindex | [README.md](README.md) |

Die Befunde beziehen sich auf den angegebenen Stand von `Archiving`. Andere Branches und installierte Zielgeräte wurden nicht geprüft.

### 1.1 Tatsächlich zugänglicher Verlauf

Die erhaltene Ausarbeitung vom 25.09.2026 enthält das funktionale Ziel, vollständige Skriptentwürfe und die optionale Kill-Switch-Idee.

Der vollständige Skriptentwurf ist erhalten. Spätere technische Korrekturen, reale SSID-Festlegungen oder erfolgreiche Zielgerätetests sind nicht dokumentiert.

Bilder, `.ovpn`-Profile, Zielgeräteausgaben und Testprotokolle sind nicht erhalten.

Die historischen Quellenmarker 0 bis 4 wurden ohne zugehörige Ziel-URLs geliefert. Ihre ursprüngliche Zuordnung ist nicht rekonstruierbar. Die offiziellen Quellen in Abschnitt 10 wurden zusätzlich am Archivdatum geprüft und ersetzen keine fehlenden historischen Referenzen.

## 2. Ziel, Ausgangslage und verbindliche Anforderungen

Ziel: Raspberry Pi OS und OpenVPN; der Pi soll sich automatisch nach Hause verbinden, wenn er weder in ausdrücklich benannten WLANs noch im Heim-LAN `10.0.1.0/24` ist.

Die funktionale Regel:

~~~text
VPN erforderlich = NICHT (vertrauenswürdiges WLAN ODER physisches Heim-LAN)
~~~

| Situation | Gewünschtes Verhalten | Beleg / Grenze |
|---|---|---|
| Mit einem vertrauenswürdigen WLAN verbunden | VPN aus | Anforderung; tatsächliche SSIDs fehlen |
| Physisches LAN im Netz `10.0.1.0/24` | VPN aus | Anforderung; reale Interfaces und Netzparameter fehlen |
| Fremdes WLAN, Hotspot oder fremdes LAN | Heim-VPN automatisch an | Anforderung; erreichbarer Uplink und gültiges Profil vorausgesetzt |
| Heimnetz nur durch VPN-Route erreichbar | VPN eingeschaltet lassen | Zentrale Korrektur eines möglichen Erkennungsfehlers im ersten Entwurf |
| Kein nutzbarer Uplink | Noch nicht abschließend definiert | Der Vorschlag versucht ohne gesonderte Offline-Prüfung den VPN-Start |
| Vertrauenswürdiger und fremder Anschluss gleichzeitig aktiv | Noch nicht ausdrücklich entschieden | Das Skript lässt jeden erkannten vertrauenswürdigen Anschluss global gewinnen |
| VPN-Verbindung scheitert | Historischer Vorschlag bleibt fail-open | Direkter Verkehr wird von der Policy nicht gesperrt; Kill-Switch nur angeboten |

**Festgelegt ist das funktionale Ziel.** Der technische Weg ist ein ausgearbeiteter Vorschlag. Dispatcher oder Timer sowie die Einführung eines Kill-Switch sind noch zu entscheiden; ein Zielgeräte-Rollout ist nicht erfolgt.

Nicht geliefert wurden Pi-Modell, installierte OS-/NetworkManager-/OpenVPN-Versionen, reale SSID-Liste, Interface-Namen, vorhandenes VPN-Profil, VPN-Endpunkt, Port, UDP/TCP-Auswahl, Tunneladressen, DNS-Konzept, IPv6-Regel, Split-/Full-Tunnel-Anforderung oder Betriebsprotokolle. Auch die im Beispiel angenommene bereits vorhandene `.ovpn`-Datei wurde nicht angehängt.

## 3. Statusmodell und erreichter Stand

| Status | Verwendung in diesem Archiv |
|---|---|
| **Idee** | Option oder Vorschlag ohne ausdrücklichen Umsetzungsbeschluss |
| **beschlossen/geplant** | Festgelegtes Ziel oder autorisierte Arbeit; kein Umsetzungsnachweis |
| **implementiert** | Konkreter Code oder eine Konfiguration im benannten Repository-Stand vorhanden |
| **getestet** | Nachvollziehbar ausgeführter, abgegrenzter Test mit Ergebnis |
| **im Betrieb bestätigt** | Beobachteter erfolgreicher Einsatz auf dem Zielsystem |

| Gegenstand | Entwurfsergebnis | Am Archivdatum belegter Stand |
|---|---|---|
| Automatisches Heim-VPN außerhalb vertrauenswürdiger Netze | **beschlossen/geplant** als gewünschte Funktion | Kein zugehöriger Zielgerätebetrieb nachgewiesen |
| NetworkManager-OpenVPN-Profil `VPN-Home` | Konkreter Vorschlag, **Idee** | Kein Profil oder Importbeleg dieser Arbeitsphase vorhanden |
| Bash-Policy `/usr/local/sbin/vpn-policy` | Vollständiger Skriptentwurf | Im geprüften Repository keine entsprechende Implementierung gefunden |
| `vpn-policy.service` und `90-vpn-policy` | Vollständige vorgeschlagene Konfigurationen | Keine installierten Units, Dispatcher-Dateien oder Laufzeitlogs belegt |
| nftables-Kill-Switch | **Idee**, ausdrücklich optional | Keine Regeln dieses VPN-Kill-Switches vorhanden oder getestet |
| Allgemeine Dashboard-WLAN-Verwaltung | Kein Umsetzungsergebnis dieser Arbeitsphase | **implementiert**: `crates/tetra-entities/src/wifi.rs` vorhanden |
| Historische Imagebuilder-/VPN-Variante | Im Fachentwurf nicht behandelt | In geprüften Handbüchern als andere Entwicklungsvorschau beschrieben; Quellpfad im geprüften Branch fehlt |
| Netzwerkwechsel, Wiederverbindung und Leakfreiheit | Kein Testbericht im dokumentierten Arbeitsstand | Weder **getestet** noch **im Betrieb bestätigt** |

Ein kopierbarer Codeblock ist noch keine Repository-Implementierung. Ein erfolgreicher Dokumentationscheck oder Git-Push wäre ebenfalls kein Nachweis eines funktionsfähigen VPN.

## 4. Vorgeschlagene Architektur und technische Parameter

### 4.1 Komponenten und Ablauf

~~~text
NetworkManager verwaltet Ethernet/WLAN und OpenVPN-Profil
  -> Netzwerkereignis
  -> /etc/NetworkManager/dispatcher.d/90-vpn-policy
  -> systemctl restart --no-block vpn-policy.service
  -> /usr/local/sbin/vpn-policy
       -> verbundene Geräte, IPv4-Adressen und WLAN-SSID lesen
       -> Vertrauensregel und aktuellen VPN-Status auswerten
       -> nmcli connection up/down "VPN-Home"
       -> Entscheidung mit logger ins Journal schreiben
~~~

Der Dispatcher soll kurz bleiben; den eigentlichen Policy-Lauf übernimmt systemd. Das ist ein **Oneshot**, kein ständig laufender Überwachungsprozess. NetworkManager soll den OpenVPN-Lebenszyklus über sein Plugin steuern. Für diese Variante ist kein zusätzlich parallel gesteuerter `openvpn-client@…`-Dienst vorgesehen.

| Komponente / Parameter | Historischer Vorschlag | Bedeutung und Grenze |
|---|---|---|
| Betriebssystem | Raspberry Pi OS mit NetworkManager | Der Entwurf setzt Bookworm/Trixie voraus; installiertes System unbekannt |
| Pakete | `network-manager-openvpn`, `openvpn` | Installation nur vorgeschlagen |
| Hilfsprogramme | Bash, `nmcli`, `ip`, `awk`, `grep`, `logger`, `systemctl`, `journalctl` | Implizite Laufzeitabhängigkeiten des Entwurfs |
| VPN-Profilname | `VPN-Home` | Beispielname; tatsächliche UUID fehlt |
| Profilimport | `/pfad/home.ovpn` | Platzhalter, kein vorhandener oder archivierter Dateipfad |
| Policy | `/usr/local/sbin/vpn-policy` | Bash erforderlich wegen Array und Prozesssubstitution |
| systemd-Unit | `/etc/systemd/system/vpn-policy.service` | `Type=oneshot`, `ExecStart=/usr/local/sbin/vpn-policy` |
| Dispatcher | `/etc/NetworkManager/dispatcher.d/90-vpn-policy` | Shell-Skript, Ereignis in Argument `$2` |
| Ereignisse im konkreten Skript | `up`, `down`, `dhcp4-change`, `connectivity-change` | `vpn-up` wurde erläutert, aber nicht in den Handler aufgenommen; auch `vpn-down` fehlt |
| Rechte | Beide Skripte `root:root` und Modus `700` | Vorgeschlagen, nicht als tatsächlich gesetzt belegt |
| Heimnetz | `10.0.1.0/24` | Standortparameter |
| Adressprüfung im Entwurf | `^10\.0\.1\.[0-9]+/`, `ip -4 -o addr … scope global` | Prüft IPv4-Adressmuster auf Geräten vom Typ `wifi` oder `ethernet` |
| VPN-Aufruf | `nmcli -w 0 connection up/down` | Wartet nicht auf Abschluss; keine erfolgreiche Tunnelprüfung |
| Logging | `logger -t vpn-policy` | Meldet die beabsichtigte Aktion, keinen Verbindungserfolg |
| VPN-Transport und Port | Nicht festgelegt | Keine pauschale Annahme „UDP/1194“ |
| Zusätzlich geöffnete API-/Webports | Keine im Entwurf definiert | Policy benutzt lokale Werkzeuge und Dienste |
| DNS, IPv6 und Full-/Split-Tunnel | Nicht festgelegt | Importiertes Profil und Netzkonzept müssen später geprüft werden |

### 4.2 Vertrauensliste: Beispiele sind keine Standortkonfiguration

Der erste Entwurf verwendete `WLAN-Zuhause`, `WLAN-Firma` und `NetCore`. Später wurden die Kategorien „HOME-LAN“, „WLAN Haus“, „WLAN NetCore Campus“ und „WLAN Werkstatt“ erläutert sowie folgende erweiterbare Beispielliste gezeigt:

```bash
TRUSTED_SSIDS=(
    "HOME"
    "HOME-5G"
    "NetCore-Campus"
    "Werkstatt"
    "xGEAR"
)
```

Die SSID-Namen sind Beispiele. Die zweite Liste ist eine Ausbauidee; reale vertrauenswürdige SSIDs sind noch festzulegen.

## 5. Historisch vorgeschlagene Befehle und Konfigurationen

**Status sämtlicher Linux-Befehle in diesem Abschnitt: nur vorgeschlagen, nicht als ausgeführt oder erfolgreich belegt.** Die folgenden Originalblöcke sichern den damaligen Entwurf einschließlich seiner Grenzen. Sie sind keine nachträglich gehärtete Installationsanleitung; vor Nutzung sind insbesondere Abschnitt 7 und die offene Standortkonfiguration abzuarbeiten.

### 5.1 Pakete installieren und VPN-Profil importieren

```bash
sudo apt update
sudo apt install network-manager-openvpn openvpn
```

```bash
sudo nmcli connection import type openvpn file /pfad/home.ovpn
```

```bash
sudo nmcli connection modify "ALTER-NAME" connection.id "VPN-Home"
sudo nmcli connection modify "VPN-Home" connection.autoconnect no
```

`ALTER-NAME` muss den tatsächlich importierten Profilnamen bezeichnen. Das Ziel des letzten Befehls war, die Steuerung der Policy zu überlassen. Die geprüfte NetworkManager-Dokumentation präzisiert jedoch: Allgemeines `connection.autoconnect` ist für VPN-Profile nicht implementiert; `connection.secondaries` ist ein anderer Aktivierungsmechanismus. `connection.autoconnect no` allein verhindert daher nicht jede konkurrierende Aktivierung. Siehe Abschnitt 10.

### 5.2 Policy-Skript

Im Entwurf vorgeschlagener Editoraufruf:

```bash
sudo nano /usr/local/sbin/vpn-policy
```

Vollständiger historischer Skriptinhalt, unverändert aus dem ersten Entwurf:

```bash
#!/bin/bash

VPN="VPN-Home"

TRUSTED_SSIDS=(
    "WLAN-Zuhause"
    "WLAN-Firma"
    "NetCore"
)

TRUSTED_SUBNET_REGEX='^10\.0\.1\.[0-9]+/'

TRUSTED=0


# --------------------------------------------------
# Physische Netzwerkinterfaces prüfen
# --------------------------------------------------

while IFS=: read -r DEV TYPE STATE
do
    [ "$STATE" = "connected" ] || continue

    case "$TYPE" in
        wifi|ethernet)

            # Ist das Interface direkt im Heimnetz?
            if ip -4 -o addr show dev "$DEV" scope global \
                | awk '{print $4}' \
                | grep -qE "$TRUSTED_SUBNET_REGEX"
            then
                TRUSTED=1
            fi

            ;;

    esac


    # WLAN-SSID prüfen
    if [ "$TYPE" = "wifi" ]; then

        CONN=$(
            nmcli -g GENERAL.CONNECTION device show "$DEV" 2>/dev/null
        )

        if [ -n "$CONN" ]; then

            SSID=$(
                nmcli -g 802-11-wireless.ssid \
                    connection show "$CONN" 2>/dev/null
            )

            for TRUSTED_SSID in "${TRUSTED_SSIDS[@]}"
            do
                if [ "$SSID" = "$TRUSTED_SSID" ]; then
                    TRUSTED=1
                fi
            done

        fi
    fi

done < <(
    nmcli -t -f DEVICE,TYPE,STATE device status
)


# --------------------------------------------------
# VPN steuern
# --------------------------------------------------

VPN_ACTIVE=0

if nmcli -t -f NAME,TYPE connection show --active \
    | grep -Fxq "${VPN}:vpn"
then
    VPN_ACTIVE=1
fi


if [ "$TRUSTED" -eq 1 ]; then

    if [ "$VPN_ACTIVE" -eq 1 ]; then
        logger -t vpn-policy "Trusted network detected - disconnecting VPN"
        nmcli -w 0 connection down "$VPN"
    fi

else

    if [ "$VPN_ACTIVE" -eq 0 ]; then
        logger -t vpn-policy "Untrusted network detected - connecting VPN"
        nmcli -w 0 connection up "$VPN"
    fi

fi
```

Vorgeschlagene Eigentümer-/Rechtesetzung:

```bash
sudo chmod 700 /usr/local/sbin/vpn-policy
sudo chown root:root /usr/local/sbin/vpn-policy
```

Das Skript bildet einen globalen booleschen Vertrauenszustand. Es vergleicht die konfigurierte SSID des ermittelten NetworkManager-Verbindungsprofils und führt die Adressprüfung auf beiden Gerätetypen `wifi` und `ethernet` aus. Die Beschreibung „nur physisch“ ist daher eine Absicht des Entwurfs, keine vollständige Hardwarevalidierung.

### 5.3 systemd-Service

```bash
sudo nano /etc/systemd/system/vpn-policy.service
```

```ini
[Unit]
Description=Automatic VPN Policy

[Service]
Type=oneshot
ExecStart=/usr/local/sbin/vpn-policy
```

```bash
sudo systemctl daemon-reload
```

Die Original-Unit besitzt keinen `[Install]`-Abschnitt, kein `WantedBy`, keine explizite NetworkManager-Reihenfolge, keinen Timer und keine Wiederholungsregel. Ein `systemctl enable vpn-policy.service` wurde im dokumentierten Arbeitsstand nicht vorgeschlagen. Das Anstoßen erfolgt über den Dispatcher beziehungsweise den einmaligen manuellen Start.

### 5.4 NetworkManager-Dispatcher

```bash
sudo nano /etc/NetworkManager/dispatcher.d/90-vpn-policy
```

```bash
#!/bin/sh

case "$2" in

    up|down|dhcp4-change|connectivity-change)

        systemctl restart --no-block vpn-policy.service
        ;;

esac
```

```bash
sudo chmod 700 /etc/NetworkManager/dispatcher.d/90-vpn-policy
sudo chown root:root /etc/NetworkManager/dispatcher.d/90-vpn-policy
```

Die offiziellen Dispatcher-Regeln verlangen ausführbare, root-eigene Dateien ohne Schreibrecht für Gruppe/Andere und ohne Setuid. Die gewählte Trennung in kurzen Dispatcher und ausgelagerten Service passt zur begrenzten Dispatcher-Laufzeit. Das belegt noch nicht, dass die gezeigte Ereignisauswahl jeden nötigen Wiederanlauf erfasst. [NetworkManager-Dispatcher](https://networkmanager.dev/docs/api/latest/NetworkManager-dispatcher.html)

### 5.5 Vorgeschlagener Initialtest und Logs

```bash
sudo systemctl start vpn-policy.service
```

```bash
journalctl -u vpn-policy.service
```

Oder über das Logger-Tag:

```bash
journalctl -t vpn-policy
```

Dazu wurden keine Ausgaben zurückgemeldet. Insbesondere ist ein erfolgreicher Oneshot-Abschluss wegen `nmcli -w 0` kein Nachweis, dass OpenVPN authentisiert ist, die erforderlichen Routen installiert sind und die Heimdienste erreicht werden.

## 6. Optionale nftables-Kill-Switch-Idee

Die Basisvariante ist ausdrücklich **fail-open**: Wenn der VPN-Aufbau scheitert, verhindert die Policy normalen direkten Internetverkehr nicht. Selbst bei aktivem Tunnel entscheidet die tatsächliche Routing-/DNS-Konfiguration, welcher Verkehr über das VPN läuft.

Als zusätzliche, nicht beschlossene Funktion wurde vorgeschlagen:

~~~text
Vertrauenswürdiges Netz: direkter Verkehr erlaubt
Fremdes Netz: zunächst nur erforderlicher Verkehr zum VPN-Endpunkt
Tunnel betriebsbereit: übriger erlaubter Verkehr über das VPN
Tunnel ausgefallen: kein ungeschützter allgemeiner Internetverkehr
~~~

**Status: Idee.** Im Entwurf existieren weder `nft`-Befehle noch Tabellen, Chains, Hook-Prioritäten, Policies oder eine getestete Regeldatei. Es gibt auch keine Entscheidung, ob nur lokaler Pi-Verkehr oder weitergeleiteter TBS-/Containerverkehr erfasst werden soll.

Als geprüfte Planungsableitung bleiben zu klären:

- IPv4 und IPv6 gemeinsam behandeln; DNS-Leaks und Split-/Full-Tunnel-Anforderungen festlegen.
- Für den Aufbau erforderliche Ausnahmen definieren, etwa DHCP, Namensauflösung eines VPN-Hostnamens und gegebenenfalls Zeitabgleich. „Nur VPN-Server“ ist bislang ein Zielbild, keine vollständige Regelmenge.
- Erreichbarkeit mehrerer oder wechselnder VPN-Endpunktadressen sowie Captive-Portal-Verhalten entscheiden.
- Filter zeitlich vor ungeschütztem Verkehr wirksam machen und Tunnelverlust berücksichtigen; bloßes nachträgliches `vpn-up`-Handling genügt nicht als Leakfreiheitsnachweis.
- Bestehende Host-/Gateway-Regeln erhalten; keine globale Regeln-Löschung als Installationsschritt.
- Wiederherstellung und lokalen Konsolenzugang vorsehen, damit eine fehlerhafte Regel keinen dauerhaften Verwaltungsverlust verursacht.

Diese Punkte sind zusätzliche technische Prüfaufgaben der Archivierung und keine nachträglich festgelegten Projektentscheidungen.

## 7. Fehlerbilder, Risiken und verworfene Ansätze

### 7.1 Zentrales Risiko: VPN-Routen-Schleife

Verworfen wurde folgende Erkennung des Heimnetzes:

```bash
ip route | grep 10.0.1.0/24
```

Die beispielhafte Route `10.0.1.0/24 via tun0` war eine schematische Beschreibung der Erreichbarkeit durch den Tunnel, kein tatsächlich ausgelesener Routingtabelleneintrag.

Wenn die Policy jede solche Route als „zu Hause“ wertet, kann eine Rückkopplung entstehen:

~~~text
Fremdes Netz -> VPN startet -> Heimnetzroute erscheint
-> Policy hält den Pi für zu Hause -> VPN stoppt -> Heimnetzroute entfällt
-> Policy startet erneut -> wiederholtes Verbinden/Trennen
~~~

Das ist ein erläutertes mögliches Fehlerbild, kein im dokumentierten Arbeitsstand beobachteter Ausfall. Vorgesehen war deshalb eine Adressprüfung auf Ethernet/WLAN. Beispiele:

| Beispiel aus dem Entwurf | Beabsichtigte Einordnung |
|---|---|
| `eth0 = 10.0.1.42/24` | Heimnetz, VPN aus |
| `wlan0 = 192.168.178.34/24`, `tun0 = 10.100.0.3`, Heimnetzroute über Tunnel | Fremdes Netz, VPN bleibt an |

Die Beispiele enthalten keine Messung eines realen Pi. Die Vermeidung der Routen-Schleife ist ein begründeter Entwurfsansatz, noch keine getestete Lösung.

### 7.2 Statische Prüfung des Skriptentwurfs vom 06.10.2026

Die folgenden Befunde entstehen aus dem Lesen des Originalcodes und dem dokumentierten Schnittstellenverhalten. Sie wurden nicht als Laufzeitfehler reproduziert.

| Befund | Mögliche Wirkung | Offene Maßnahme |
|---|---|---|
| Adressprüfung läuft bei `wifi` **und** `ethernet` | Ein fremdes WLAN mit einer `10.0.1.x`-Adresse wird auch ohne SSID-Treffer vertrauenswürdig; engerer Anforderungsbegriff „physisches LAN“ ist nicht eindeutig umgesetzt | LAN-Kriterium ausdrücklich auf die gewünschten kabelgebundenen Geräte begrenzen oder die breitere Regel bewusst bestätigen |
| Adressregex verlangt nicht die Präfixlänge `/24` | Auch `10.0.1.x/16` oder `/32` trifft; IP-Bereich und tatsächliches lokales Netz werden nicht getrennt geprüft | Festlegen, ob Adresszugehörigkeit, exaktes Interfacepräfix oder konkrete LAN-Verbindung gefordert ist |
| Gerätetyp ist kein physischer Herkunftsnachweis | Virtuelle Ethernet-Geräte können ebenfalls als `ethernet` erscheinen; Bridges/Bonds/VLANs mit IP auf dem logischen Interface werden wiederum ausgelassen | Reale Geräte-/Profil-Allowlist und gewünschte Topologien festlegen |
| Jeder einzelne Treffer setzt global `TRUSTED=1` | Ein Nebenanschluss im Heimnetz kann VPN abschalten, während die Standardroute über fremdes WLAN führt | Regel für mehrere aktive Uplinks, Routing und Egress-Vertrauen definieren |
| SSID und private IPv4-Adresse sind keine sichere Netzidentität | Gleichnamige WLANs und überlappende fremde Netze können die Erkennung täuschen | Angemessenes Trust-Modell bestimmen; SSID-/Adressprüfung nicht als Authentisierung bezeichnen |
| Literal `STATE = connected` ohne festgelegte Locale | Lokalisierte oder abweichende Zustandsausgaben können gültige Verbindungen auslassen | Locale oder maschinenlesbare Zustandswerte bewusst festlegen und testen |
| Profilname statt UUID; terse Ausgabe mit Escape-Regeln | Mehrdeutige Namen sowie Doppelpunkte/Backslashes in Namen oder SSIDs können falsch zugeordnet werden | Eindeutige UUIDs und korrektes Parsen; Sonderzeichentests |
| `nmcli -w 0` wartet nicht | Start-/Stoppantrag und fertiger Tunnel werden verwechselt; Erfolgsmeldung der Policy reicht nicht | Asynchronen Abschluss, Fehler, Routen und Erreichbarkeit separat überwachen |
| Keine gesonderte Fehlerbehandlung der Abfragen | Fehlende Werkzeuge, gestoppter NetworkManager oder fehlendes Profil können als „untrusted/inaktiv“ weiterverarbeitet werden | Abfragefehler von gültigem Netzstatus unterscheiden, Rückgabewerte prüfen |
| Kein Offline-/Retry-/Backoff-Zustand | Startversuche ohne Uplink; nach fehlgeschlagenem Aufbau eventuell kein neuer Anlass zum Wiederholen | Offline, verbindend, verbunden und fehlgeschlagen modellieren |
| `vpn-down` fehlt im Dispatcher | Tunnelverlust bei unverändertem WLAN muss keinen erneuten Policy-Lauf auslösen | VPN-Ausfallereignis und/oder kontrollierten Wiederholungsmechanismus vorsehen |
| Wiederholtes `restart --no-block` ohne Entprellung | Ereignisfolgen können laufende Auswertung unterbrechen oder veraltete Entscheidungen anstoßen | Serialisierung, Entprellung und Neuauswertung des aktuellen Zustands prüfen |
| Kein expliziter Boot-Trigger außerhalb Dispatcher-Ereignissen | Nachträgliche Installation braucht den manuellen Start; Wiederanlauf ist nicht separat abgesichert | Boot-/Resume-/NetworkManager-Neustart-Test und eindeutige Aktivierung |
| Kein lokaler Konfigurations-/Rollback-Ablauf | Eine spätere Änderung könnte vorhandene Profile oder Verwaltungserreichbarkeit beeinträchtigen | Bestehende Dateien und Units vor Deployment sichern; Rückweg erproben |

Die Hinweise zu Locale, Escape-Regeln, UUID-Auswahl und asynchronem `--wait 0` beruhen zusätzlich auf der [nmcli-Referenz](https://networkmanager.dev/docs/api/latest/nmcli.html). Dispatcher kann noch wartende Ereignisse ausführen, nachdem sich der Zustand bereits wieder geändert hat; das stützt die Forderung nach erneuter Zustandsprüfung. [Dispatcher-Referenz](https://networkmanager.dev/docs/api/latest/NetworkManager-dispatcher.html)

Die historische Annahme einer gültigen, unbeaufsichtigt aktivierbaren VPN-Konfiguration bleibt offen. Ein interaktiver Secret-Agent, Passphrasebedarf oder ein auf eine angemeldete Sitzung beschränktes Profil kann den Headless-Betrieb verhindern. Zugangsdaten gehören ausschließlich in geeignete lokale geschützte Konfiguration, nicht in dieses Archiv.

### 7.3 Verworfene, ersetzte und lediglich alternative Ansätze

| Ansatz | Einordnung |
|---|---|
| Heimnetz ausschließlich aus einer vorhandenen Route erkennen | Im Entwurf ausdrücklich verworfen, um die Rückkopplung mit der VPN-Route zu vermeiden |
| Gesamten VPN-Aufbau synchron im Dispatcher durchführen | Im Entwurf zugunsten eines kurzen Triggers und systemd-Service verworfen |
| VPN immer aktiv halten | Entspricht nicht der gewünschten Ausnahme für Heim-LAN/vertrauenswürdige WLANs |
| Nur SSID prüfen | Vom Vorschlag um das LAN-Kriterium ergänzt; allein würde es die Anforderung nicht erfüllen |
| fail-open vollständig durch Kill-Switch ersetzen | Nur als Option angeboten, nicht beschlossen und nicht implementiert |
| Spätere Beispiel-SSID-Liste | Ausbauidee, keine bestätigte neue Standortkonfiguration |
| Timer + `openvpn-client@netcore.service` | Separate Repository-Dokumentation aus anderem Entwicklungsstand; im Fachentwurf weder verworfen noch als Nachfolger beschlossen |
| WireGuard, IPsec oder anderer VPN-Anbieter | Im zugänglichen Fachentwurf nicht erörtert; keine erfundene Alternativenentscheidung |

Es wurden keine tatsächlichen Fehlerlogs übergeben. Eine „funktionierende Reparatur“ kann daher nicht als bereits durchgeführt dokumentiert werden.

## 8. Aktuell überprüfter Repository-Stand

Alle folgenden Live-Repository-Befunde beziehen sich auf `Archiving` bei `0f87d409d400376385013754418aaacfe04a830a`. Der Branch wurde frisch in ein separates lokales Verzeichnis geladen und vor dem Schreiben erneut abgeglichen. Die Arbeitskopie war zunächst sauber.

### 8.1 Archivbestand und eindeutige Zuordnung

Im geprüften Archivbestand lagen 66 datierte Dokumente und `README.md` vor. Eine passende Implementierung von `vpn-policy` oder `TRUSTED_SSIDS` war dort nicht vorhanden. Die [Deployment-/Imagebuilder-Ausarbeitung](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md) beschreibt eine separate Variante.

### 8.2 Code und Dokumentation getrennt bewertet

| Geprüfte Quelle | Befund | Aussagegrenze |
|---|---|---|
| [`crates/tetra-entities/src/wifi.rs`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/crates/tetra-entities/src/wifi.rs) | WLAN-Scan, gespeicherte Profile, Verbinden/Trennen und Radio-Steuerung über `nmcli`; Aufruf-Timeout und Parser für terse Escape-Zeichen vorhanden | Allgemeine WLAN-Verwaltung ist implementiert, die hier beschriebene Heim-VPN-Policy nicht |
| [`crates/tetra-entities/src/net_dashboard/server.rs`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/crates/tetra-entities/src/net_dashboard/server.rs) und [Dashboard-HTML](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/crates/tetra-entities/src/net_dashboard/ui/dashboard.html) | Anbindung der WLAN-Bedienung im Dashboard identifiziert | Kein Beleg für automatische OpenVPN-Steuerung |
| [`Docs/NetCore-Tetra-Komplettguide-2026-09-28.md`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/Docs/NetCore-Tetra-Komplettguide-2026-09-28.md) | Ab Zeile 3952 ausdrücklich gesonderte Entwicklungsvorschau; ab Zeile 4013 Pi-Images und VPN-Automatik beschrieben | Historische Dokumentation eines anderen Quellstands |
| [`Docs/NetCore-Tetra-Systemhandbuch-2026-09-28.md`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/Docs/NetCore-Tetra-Systemhandbuch-2026-09-28.md) | Entsprechende Vorschau mit OpenVPN-Timer, `openvpn-client@netcore.service` und `/etc/netcore/image-network.json` vorhanden | Dokumentierte Variante, kein Nachweis ihrer Integration in diesen Branch |
| `system-backend/deployment-core/` | Weder als getrackter Verzeichnisbaum noch als lokaler Pfad vorhanden | Die beschriebene Imagebuilder-Implementierung ist am geprüften Branchstand nicht verfügbar |
| Suche nach `vpn-policy`, `TRUSTED_SSIDS`, `network-manager-openvpn` und `dispatcher.d` | Vor dem Schreiben keine entsprechende Policy-Implementierung im getrackten Textbestand gefunden | Keine Aussage über unbekannte Dateien auf Zielgeräten oder andere Git-Refs |
| [`system-backend/ip-gateway/docs/architecture.md`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/system-backend/ip-gateway/docs/architecture.md) | TETRA-Paketdatenpfad und eigene nftables-Tabellen `inet netcore_ip_gateway` / `ip netcore_ip_gateway_nat` dokumentiert | Diese Gateway-Regeln sind kein Heim-VPN-Kill-Switch |
| [`crates/tetra-entities/src/sndcp/packet_gateway.rs`](https://github.com/JanHG98/netcore-tetra/blob/0f87d409d400376385013754418aaacfe04a830a/crates/tetra-entities/src/sndcp/packet_gateway.rs) und `contrib/packet-data/` | Vorhandene nftables-/iptables-Anbindung für Packet Data | Keine Abnahme von Host-VPN-Leakfreiheit |

Die Suchprüfung umfasste getrackte Textdateien und Dateinamen; relevante Treffer wurden einzeln gelesen. Weder aus Quellkommentaren noch aus Handbuchformulierungen wird eine zum Prüfstand vom 06.10.2026 laufende Installation abgeleitet.

### 8.3 Separate Imagebuilder-Variante und ihr Herkunftsstand

Die zum Prüfstand vom 06.10.2026 im Branch vorhandenen Handbücher kennzeichnen ihre Vorschau mit dem historischen Commit `bbf039729b9b05f8d623b11195ca24a124f68d16` des damals genannten `feature/openlab-discovery-deployment`. Dieser SHA ist eine **übernommene Herkunftsangabe der gelesenen Dokumentation**, kein in diesem Auftrag ausgecheckter oder auf Funktionsfähigkeit getesteter Stand.

Laut dieser Vorschau:

- prüft eine Netzrichtlinie etwa alle **15 Sekunden** aktive NetworkManager-Verbindungen;
- gilt eine erlaubte SSID oder eine **kabelgebundene** Adresse im konfigurierten LAN-Präfix als vertrauenswürdig;
- wird `openvpn-client@netcore.service` außerhalb gestartet und zu Hause gestoppt;
- liegen Richtliniendaten unter `/etc/netcore/image-network.json`;
- wird der Timer ohne eingebettetes OpenVPN-Profil nicht aktiviert;
- soll die Richtlinie keine Funkdienste neu starten.

Diese Variante benutzt NetworkManager zur Netzerkennung, steuert aber einen systemd-OpenVPN-Client statt des im Fachentwurf vorgeschlagenen importierten NM-VPN-Profils. Ein Timer unterscheidet sich auch vom vorgeschlagenen Dispatcher. Die Handbuchaussage zur kabelgebundenen Prüfung ist enger als der historische Bash-Code, der auch WLAN-Adressen prüft.

Das [benachbarte Deployment-Archiv vom 05.10.2026](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md) beschreibt eine damalige historische Quellprüfung und eine Integrationslücke. Dessen Aussagen über `main`, frühere Branchlisten, Z01-Roadmap, PRs und Tests bleiben diesem anderen Archiv zugeordnet. Sie wurden hier nicht als geprüfter Stand anderer Branches erneut bestätigt.

Vor einer Implementierung ist deshalb zu entscheiden, welcher Lebenszyklus verwendet oder kontrolliert übernommen wird. Zwei parallel schaltende VPN-Policies dürfen nicht versehentlich für dasselbe Profil aktiviert werden. Dieses Archiv integriert keinen historischen Code und erzeugt keinen neuen technischen PR.

## 9. Tests, Ergebnisse und fehlende Abnahme

### 9.1 In diesem Prüfdurchlauf vom 06.10.2026 durchgeführt

- Funktionales Ziel und vollständigen Skriptentwurf geprüft.
- Erhaltene Entwurfsunterlagen und fehlende Anhänge festgestellt.
- Aktuellen Remote-Branch, lokalen Branch und vollständigen Basis-SHA verglichen; saubere separate Arbeitskopie vorgefunden.
- Bestehende Ausarbeitungen und Index auf bereits dokumentierte Policyvarianten geprüft.
- Relevante Repository-Dateien und das Fehlen der konkreten Policy-/Deployment-Core-Pfade statisch geprüft.
- Den historischen Entwurf gelesen und seine Grenzen von tatsächlich beobachteten Fehlern getrennt.
- Aktuelle offizielle Dokumentation zu NetworkManager, nmcli, Dispatcher und Debian-Paketangebot abgeglichen.

Diese Punkte sind Quellen-/Dokumentationsprüfungen. Es wurden keine Linux-Dienste gestartet, keine VPN-Verbindung aufgebaut, keine Firewallregeln verändert, kein Pi neu gestartet und keine TBS getestet.

### 9.2 Noch ausstehende Testmatrix

Alle folgenden Tests sind **geplant als Empfehlung dieses Archivs**, nicht durchgeführt und nicht bereits durch den Betreiber terminiert.

| Testfall | Erwartung / zu prüfendes Kriterium |
|---|---|
| Boot im bestätigten Heim-LAN | VPN aus; kein kurzzeitiger ungewollter Neuaufbau |
| Boot in jedem bestätigten WLAN | Exakter SSID-/Profilabgleich; VPN aus |
| Fremdes WLAN oder Hotspot | VPN-Aufbau einschließlich Authentisierung, Route und Heimdienst-Erreichbarkeit erfolgreich |
| Fremdes Ethernet | VPN an; kein Vertrauen allein wegen Geräteart |
| VPN bringt eine Route nach `10.0.1.0/24` mit | Keine Umschalt-Rückkopplung |
| Fremdes Netz verwendet ebenfalls `10.0.1.0/24` | Verhalten gemäß bewusst festgelegtem Trust-Modell; keine unbeachtete Freigabe |
| WLAN im Bereich `10.0.1.x` ohne erlaubte SSID | Gewollte LAN-/WLAN-Semantik nachweisen |
| Adresse `10.0.1.x/16` oder `/32` | Festgelegte Präfixsemantik nachweisen |
| Zwei aktive Uplinks mit widersprüchlichem Vertrauen | Egress-Regel und VPN-Zustand bleiben konsistent |
| Kein Kabel, WLAN getrennt, DHCP noch nicht fertig | Definierter Offlinezustand; keine unkontrollierte Startschleife |
| DHCP-Wechsel, Roaming und schneller Linkwechsel | Aktueller Zustand gewinnt; keine veralteten oder konkurrierenden Aktionen |
| VPN-Server nicht erreichbar oder Profil fehlerhaft | Sichtbarer Fehler und kontrollierte Wiederholung; gewähltes fail-open/fail-closed-Verhalten |
| Tunnel stirbt bei unverändertem WLAN | Wiederanlauf beziehungsweise Sperre zuverlässig |
| NetworkManager-Neustart, Reboot und Resume | Policy wird erneut wirksam |
| Namen/SSIDs mit Leerzeichen, `:`, `\` oder Umlauten; doppelte Profilnamen | Korrekte Zuordnung ohne Parsingfehler |
| Deutsche/englische Locale; extern verwaltete Verbindung | Zustandsauswertung nachvollziehbar; nicht unterstützte Fälle erkennbar |
| Bridge/Bond/VLAN oder virtuelle Ethernet-Geräte | Gewünschte Interface-Auswahl, keine falsche Heimnetzidentifikation |
| DNS, IPv6, Split-/Full-Tunnel | Routen und Namensauflösung entsprechen der festgelegten Policy |
| Optionaler Kill-Switch: Boot, Aufbau, Abbruch, Reconnect | Kein unerlaubter Direktverkehr; notwendige Aufbauausnahmen funktionieren |
| Reale NetCore-TBS mit bestehendem Funkbetrieb | Netzwechsel erzeugt keinen unbeabsichtigten Funkdienst-Neustart; getrennte End-to-End-Abnahme |
| Rollback | Vorherige Konfiguration und Verwaltungserreichbarkeit wiederherstellbar |

Als zusätzliche spätere Diagnose sind beispielsweise `nmcli --version`, `openvpn --version`, `nmcli connection show --active`, `ip -4 addr`, `ip -4 route`, `ip -6 route` und `journalctl -u NetworkManager` sinnvoll. Sie wurden hier nicht auf einem Pi ausgeführt. Ausgaben vor Weitergabe auf Zugangsdaten und sensible Standortinformationen prüfen; keine Profile mit Secrets in das Repository exportieren.

## 10. Zusätzlich geprüfte offizielle Quellen

Abrufdatum: 06.10.2026. Diese Quellen wurden für die geprüfte technische Einordnung herangezogen. Sie sind keine rekonstruierten Originalquellenmarker und kein Ersatz für die Versionsprüfung auf dem Zielgerät.

| Quelle | Unterstützte Aussage |
|---|---|
| [Raspberry Pi: Netzwerkkonfiguration](https://www.raspberrypi.com/documentation/computers/configuration.html) | Ab Raspberry Pi OS Bookworm ist NetworkManager das Standardwerkzeug zur Netzwerkkonfiguration; daraus folgt nicht, dass er auf dem konkreten Pi aktiv ist |
| [Debian Trixie: network-manager-openvpn](https://packages.debian.org/trixie/network-manager-openvpn) | OpenVPN-Plugin ist verfügbar, unter anderem für ARM64; Paketverfügbarkeit ist kein Installationsbeleg |
| [NetworkManager: nmcli](https://networkmanager.dev/docs/api/latest/nmcli.html) | VPN-Import benötigt das passende Plugin; UUID-Auswahl, Ausgabeformat, Locale und Warteverhalten sind dokumentiert; `-w 0` wartet nicht auf den Abschluss |
| [NetworkManager: connection-Einstellungen](https://networkmanager.dev/docs/api/latest/settings-connection.html) | Allgemeines VPN-Autoconnect ist nicht implementiert; `secondaries` aktiviert VPN-UUIDs mit einer Basisverbindung |
| [NetworkManager: Dispatcher](https://networkmanager.dev/docs/api/latest/NetworkManager-dispatcher.html) | Dateirechte, Aktionen, asynchrone Verarbeitung, begrenzte Laufzeit und möglicherweise überholte wartende Ereignisse |

Keine aktuelle Paketversionsnummer, VPN-Portnummer oder Systemversion wurde als angebliche Zielgerätekonfiguration ergänzt.

## 11. Offene Aufgaben, Roadmap-Kandidaten und nächste Schritte

Die nachfolgende Reihenfolge ist eine Empfehlung aus der Archivprüfung. Im ursprünglichen Fachentwurf wurden keine Prioritäten oder Termine vereinbart. Die Roadmap-Kandidaten bleiben ausschließlich in dieser Datei.

| Kandidat | Inhalt | Abhängigkeit / Abschlusskriterium |
|---|---|---|
| VPN-01: Standort und gewünschtes Verhalten festlegen | Tatsächliche OS-/Toolversionen, NetworkManager-Verwaltung, SSIDs, physische LAN-Interfaces, VPN-Profil und Endpunkt erfassen; LAN-/WLAN- und Mehr-Uplink-Semantik klären | Keine Platzhalter mehr in der freigegebenen Konfiguration |
| VPN-02: Einen Steuerungsweg wählen | NM-VPN + Dispatcher/Oneshot gegen kontrollierte Übernahme der dokumentierten Timer-/OpenVPN-Client-Variante abgleichen | Genau ein Verantwortlicher für VPN-Lebenszyklus; historische Quellbasis erst separat verifizieren |
| VPN-03: Policy robust implementieren | UUIDs, Locale/Parsing, Fehlerzustände, Offline, Wiederholung, Ereignisbehandlung und Statuslogging | Nachvollziehbare Entscheidung samt erfolgreich geprüftem Tunnelzustand |
| VPN-04: Deployment und Rückweg | Profile/Dateien sichern, Rechte setzen, Startreihenfolge definieren, Rücknahme dokumentieren | Installation und Rollback auf einem Test-Pi reproduzierbar |
| VPN-05: Optionalen Kill-Switch entscheiden | fail-open oder fail-closed, DNS/IPv6, Endpoint-Ausnahmen und lokaler/weitergeleiteter Verkehr | Bewusste Freigabe des Sicherheitsverhaltens; anschließend Regelentwurf und Leaktests |
| VPN-06: NetCore-Integration abnehmen | WLAN-Bedienung, Funk-/Core-Verbindungen, Netzwechsel und Dauerbetrieb prüfen | Reale Pi-/TBS-Testprotokolle mit Datum, Versionen und Ergebnissen |

Konkreter nächster technischer Schritt ist VPN-01, danach die Architekturentscheidung VPN-02. Erst dann ist eine belastbare Implementierung sinnvoll. Ein Rollout auf das Zielgerät steht aus.

## 12. Offene Belege und Fortsetzungsgrenzen

1. Die erhaltene Ausarbeitung wurde vollständig geprüft; darüber hinausgehende frühere Festlegungen sind nicht belegt.
2. Die fünf historischen Quellenmarker sind ohne Ziel-URLs überliefert. Neue offizielle Quellen sind separat kenntlich gemacht.
3. Keine Bilder, Anhänge, `.ovpn`-Dateien, Zielgeräteausgaben oder Testlogs sind im erhaltenen Material vorhanden.
4. Tatsächliche SSIDs, VPN-Zugangsdaten, Endpunkt und installierte Versionen fehlen; keine wurden erfunden oder aus anderen Arbeitsphasen übernommen.
5. Historischer Vorschlag, im Branch vorhandene Dokumentation und zum Prüfstand vom 06.10.2026 vorhandener Code sind verschiedene Evidenzebenen. Der genannte historische `bbf0397…`-Stand, seine PR-/CI-Angaben und geprüfte andere Branches wurden hier nicht erneut geprüft.
6. Kein Laufzeit-/Hardware-/End-to-End-Test dieses VPN-Toggles ist belegt. Der nftables-Kill-Switch bleibt eine unbeschlossene Idee.
7. Die Archivierung ändert ausschließlich diese Datei und den Index. Codeintegration, Roadmapänderungen außerhalb des Archivs und Deployment sind offene Folgearbeit.
