#!/usr/bin/env python3
"""Build the single-sheet NetCore overview using Xmind's free topic features.

The checked-in Git inventory fixes the scope and source revision. Roadmap tables,
IAM/Drive milestones, service metadata and MS feature tables are read from their
maintained documents. No Xmind subscription, SDK, network or third-party package
is required. Run --check to check generated files without writing them.
"""
from __future__ import annotations

import argparse
import hashlib
import html
import io
import json
from pathlib import Path, PurePosixPath
import re
import textwrap
import tomllib
import uuid
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / 'Docs/roadmaps'
INVENTORY = DEST / 'xmind/repository-inventar.json'
MAP = DEST / 'netcore-tetra-gesamtuebersicht.xmind'
PREVIEW = DEST / 'netcore-tetra-gesamtuebersicht.svg'
MASTER = 'Docs/roadmaps/gesamtroadmap.md'
IAM = 'Docs/roadmaps/zentrale-anmeldung-und-berechtigungen.md'
DRIVE = 'Docs/roadmaps/dateicloud-und-ordnerfreigaben.md'
NS = uuid.UUID('025d9500-c864-4967-8d50-336b42c43dba')
CONTENT_NS = 'urn:xmind:xmap:xmlns:content:2.0'
MANIFEST_NS = 'urn:xmind:xmap:xmlns:manifest:1.0'


def identifier(key: str) -> str:
    return uuid.uuid5(NS, key).hex[:26]


def clean(value: str) -> str:
    value = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'\1 (\2)', value)
    value = value.replace('**', '').replace('`', '')
    return value.strip()


def short(value: str, limit: int = 88) -> str:
    value = re.sub(r'\s+', ' ', clean(value))
    return value if len(value) <= limit else value[:limit - 1].rstrip() + '…'


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding='utf-8')


def section(path: str, heading: str) -> str:
    lines = read(path).splitlines()
    begin = next(i for i, line in enumerate(lines) if line.startswith(heading))
    level = len(lines[begin]) - len(lines[begin].lstrip('#'))
    end = next((i for i in range(begin + 1, len(lines)) if re.match(r'^#{1,' + str(level) + r'} ', lines[i])), len(lines))
    return '\n'.join(lines[begin + 1:end]).strip()


def table_rows(path: str, key_pattern: str) -> list[list[str]]:
    result = []
    for line in read(path).splitlines():
        if not line.startswith('|'):
            continue
        cols = [clean(col) for col in re.split(r'(?<!\\)\|', line.strip())[1:-1]]
        if cols and re.match(key_pattern, cols[0]):
            result.append(cols)
    return result


class MapBuilder:
    def __init__(self, inventory: dict):
        self.inventory = inventory
        self.files = {item['path']: item for item in inventory['files']}
        self.nodes: dict[str, dict] = {}
        self.relationships: list[dict] = []
        self.referenced: set[str] = set()
        self.commit = inventory['commit']
        self.url = 'https://github.com/' + inventory['repository']

    def source_note(self, sources: list[str]) -> str:
        notes = []
        for path in sources:
            if path not in self.files and not any(p.startswith(path.rstrip('/') + '/') for p in self.files):
                raise ValueError('Unknown snapshot source: ' + path)
            self.referenced.add(path)
            kind = 'blob' if path in self.files else 'tree'
            object_note = ('Git-Objekt: ' + self.files[path]['sha']) if path in self.files else ('Quellcommit: ' + self.commit)
            notes.append(path + '\n' + object_note + '\n' + self.url + '/' + kind + '/' + self.commit + '/' + path)
        return '\n\n'.join(notes)

    def node(self, key: str, title: str, children: list[dict] | None = None,
             note: str = '', sources: list[str] | None = None, folded: bool = True) -> dict:
        if key in self.nodes:
            raise ValueError('Duplicate topic key: ' + key)
        item = {'id': identifier('topic:' + key), 'title': title}
        if children:
            item['children'] = {'attached': children}
            if folded:
                item['branch'] = 'folded'
        if sources:
            note += ('\n\n' if note else '') + 'Quellen der Momentaufnahme:\n' + self.source_note(sources)
        if note:
            item['notes'] = {'plain': {'content': note}}
        self.nodes[key] = item
        return item

    def leaves(self, key: str, titles: list[str], sources: list[str] | None = None) -> list[dict]:
        return [self.node(key + ':' + str(i), title, sources=sources) for i, title in enumerate(titles)]

    def feature(self, key: str, title: str, sources: list[str], details: list[str], note: str = '') -> dict:
        return self.node(key, title, self.leaves(key, details), note=note, sources=sources)

    def relate(self, start: str, end: str, title: str) -> None:
        self.relationships.append({
            'id': identifier('relationship:' + start + ':' + end),
            'title': title,
            'end1Id': self.nodes[start]['id'],
            'end2Id': self.nodes[end]['id'],
            'controlPoints': {'0': {'angle': 90, 'amount': 50}, '1': {'angle': 270, 'amount': 50}},
        })

    def roadmap(self) -> dict:
        titles = {
            'Z01': 'P0 · Z01 – Installation, Upgrade & Recovery',
            'Z02': 'P0 · Z02 – gezielte Funk- & Routing-Stabilität',
            'Z03': 'P0 · Z03 – Einzelzelle & Core abnehmen',
            'Z04': 'P1 · Z04 – gemeinsame Anmeldung & RBAC',
            'Z05': 'P1 · Z05 – Alltagspfade durchgehend schließen',
            'Z06': 'P1 · Z06 – NetCore Drive mit lokalem Login',
            'Z07': 'P1 · Z07 – zweite TBS & laufende Rufmigration',
            'Z08': 'P1 · Z08 – Leitstellen-Audio, Aktoren & Sensorik',
            'Z09': 'P1 · Z09 – Drive-Plugins & Dateinutzung',
            'Z10': 'P1/P2 · Z10 – gesicherter Betrieb & Skalierung',
            'Z11': 'P2 · Z11 – weitere Clients & Funkbrücken',
        }
        jobs = {}
        for row in table_rows(MASTER, r'^Z\d{2}$'):
            task, priority, work, dependency, gate = row
            sub = [
                self.node(task + ':scope', 'Arbeitsumfang', note=work),
                self.node(task + ':dependency', 'Voraussetzungen / Abhängigkeiten', note=dependency),
                self.node(task + ':gate', 'Abnahme / Übergang', note=gate),
            ]
            if task == 'Z01':
                for detail in table_rows(MASTER, r'^Z01\.[1-4]'):
                    task_id = re.match(r'^Z01\.\d', detail[0]).group()
                    state = 'IN ARBEIT' if task_id == 'Z01.4' else 'QUELLE/CI ERLEDIGT'
                    sub.append(self.node(task_id, short(detail[0]) + ' · ' + state,
                        note='Status / datierter Nachweis:\n' + detail[1] + '\n\nErgebnis / Abnahme:\n' + detail[2], sources=[MASTER]))
            if task == 'Z02':
                pattern = r'^- \*\*(Z02\.[1-5]) ([^*]+)\*\*:?\s*(.*)$'
                for line in section(MASTER, '### Z02 –').splitlines():
                    match = re.match(pattern, line)
                    if match:
                        task_id, name, detail = match.groups()
                        if task_id == 'Z02.5':
                            detail += '\n\n' + clean(section(MASTER, '### Z02.5 –'))
                        sub.append(self.node(task_id, task_id + ' · ' + short(name) + ' · OFFEN', note=clean(detail), sources=[MASTER]))
            if task == 'Z11':
                sub.append(self.node('Z11.1', 'Z11.1 · TETRA-Empfangsgateway · P2 / GEPLANT',
                    note=clean(section(MASTER, '### Z11.1:')), sources=[MASTER, 'crates/tetra-entities/src/net_brew/protocol.rs']))
            status = 'IN ARBEIT / TEILABNAHMEN' if task == 'Z01' else 'OFFEN / GEPLANT'
            jobs[task] = self.node(task, titles[task], sub, note='Gesamtstatus: ' + status + '\nZeitpunkt: ' + priority + '\nKalendertermine und Prozentwerte sind nicht zugesagt.', sources=[MASTER])
        if set(jobs) != set(titles):
            raise ValueError('The master roadmap no longer has exactly Z01–Z11; review the generator grouping.')
        for start, end in [('Z01', 'Z03'), ('Z02', 'Z03'), ('Z03', 'Z05'), ('Z03', 'Z07'), ('Z05', 'Z07'), ('Z06', 'Z09')]:
            self.relate(start, end, 'Voraussetzung; Details im Zielknoten')
        return self.node('roadmap', '01 · Arbeitsfolge – Jetzt / Parallel / Danach', [
            self.node('roadmap:now', 'Jetzt · P0', [jobs[x] for x in ['Z01', 'Z02', 'Z03']]),
            self.node('roadmap:parallel', 'Eigene Stränge · Architektur parallel möglich', [jobs[x] for x in ['Z04', 'Z06']]),
            self.node('roadmap:next', 'Danach · abhängig vom jeweiligen Pilot', [jobs[x] for x in ['Z05', 'Z07', 'Z08', 'Z09', 'Z10', 'Z11']]),
        ], sources=[MASTER], note='Die Z-IDs und deren Reihenfolge stammen aus NETCORE-MASTER-01. Fachphasen und historische Paketnummern ersetzen diese Gesamtfolge nicht.', folded=False)

    def milestones(self, path: str, prefix: str, key: str, title: str) -> dict:
        result = []
        for row in table_rows(path, '^' + prefix + r'\d$'):
            task, priority, name, dependency, gate = row
            detail = ''
            if prefix == 'M':
                detail = clean(section(path, '### ' + task + ' —'))
            result.append(self.node(key + ':' + task, task + ' · ' + priority + ' · ' + name,
                self.leaves(key + ':' + task, ['Status: GEPLANT', 'Voraussetzungen: ' + short(dependency)]),
                note='Voraussetzungen:\n' + dependency + '\n\n' + ('Status:\n' if prefix == 'M' else 'Abnahme:\n') + gate + ('\n\n' + detail if detail else ''), sources=[path]))
        expected = 9 if prefix == 'M' else 10
        if len(result) != expected:
            raise ValueError(f'{prefix}: Expected {expected} milestones, got {len(result)}; review grouping.')
        return self.node(key, title, result, sources=[path], note='Planung; kein Implementierungs- oder Standortnachweis.')

    def radio(self) -> dict:
        modules = []
        names = ['phy', 'lmac', 'umac', 'llc', 'mle', 'mm', 'cmce', 'sndcp']
        explanations = {
            'phy': ['SoapySDR / SXceiver, RX-/TX-Streaming', 'Modulator, Demodulator und Slotter', 'TDMA-Zeit, Kontinuitätsdiagnose und RX-Recovery'],
            'lmac': ['Kanalcodierung, Scrambling und Burstverarbeitung', 'TCH/S und Signalisierung'],
            'umac': ['Zugriff, Timeslots und Fragmentierung', 'Carrierbelegung und Downlink-Ressourcen'],
            'llc': ['Linkkontrolle, ACK und Wiederholung', 'Verbindungen zwischen MAC und höherem Protokoll'],
            'mle': ['Zellinformationen, Nachbarn und Zellwechsel', 'TLMC-/TLPD-Routing und Break/Resume'],
            'mm': ['Registrierung, Re-Registration und Gruppen', 'Lokale Whitelist; zentrale Handler-Lücken prüfen'],
            'cmce': ['Einzel-, Gruppen- und Duplexruf', 'Sprechrecht, Hangtime, Release und Restore', 'SDS und Supplementary Services'],
            'sndcp': ['PDP-/SNDCP-Kontexte und Paketdaten', 'Uplink, Downlink und Mehrkontext-Abnahme'],
        }
        for name in names:
            prefix = 'crates/tetra-entities/src/' + name
            count = sum(p.startswith(prefix + '/') for p in self.files)
            modules.append(self.feature('layer:' + name, name.upper() + ' · Quellpfade vorhanden', [prefix], explanations[name], note=f'{count} versionierte Dateien. Vorhandener Code ist kein Nachweis vollständiger aktiver Laufzeitpfade.'))
        ms_path = 'Docs/components/ms-mode/ms-architektur-und-funktionsstand.md'
        ms_features = []
        current = None
        table = []
        for line in read(ms_path).splitlines():
            if re.match(r'^### 3\.[0-9]+ ', line):
                if current:
                    ms_features.append(self.node('ms-table:' + current, clean(current), table, sources=[ms_path]))
                current, table = line[4:], []
            elif current and line.startswith('## '):
                ms_features.append(self.node('ms-table:' + current, clean(current), table, sources=[ms_path]))
                current, table = None, []
            elif current and line.startswith('|'):
                cells = [clean(x) for x in line.split('|')[1:-1]]
                if len(cells) >= 4 and cells[0] not in ('Feature', '---') and not cells[0].startswith('-'):
                    state = {'✅': 'CODE / HIST. RF-BERICHT', '🧪': 'TESTPFAD VORHANDEN', '🚧': 'TEILWEISE', '📋': 'GEPLANT', '⛔': 'AUSSERHALB UMFANG'}.get(cells[1], cells[1])
                    table.append(self.node('ms-feature:' + current + ':' + str(len(table)), short(cells[0]) + ' · ' + state,
                        note='Quelle nennt: ' + cells[1] + '\nNormreferenz der Quelle: ' + cells[2] + '\nDetails: ' + ' | '.join(cells[3:]) + '\nGeerbte RF-Aussagen benötigen einen identifizierten getesteten Stand; dieser Generator führt keinen MS-/RF-Test aus.'))
        ms = self.node('ms', 'MS-Modus · eigener Workspace, eigene Nachweisgrenzen', ms_features,
            note='Separater Workspace ms-mode/; RX-getriebene Uhr und diskontinuierlicher Uplink. Vocoder/UI in einem externen Referenzclient. DMO, MS-Paketdaten und Funk-Security sind in dieser Quelle ausdrücklich außerhalb des implementierten MS-Umfangs.', sources=[ms_path, 'ms-mode/Cargo.toml'])
        crates = [self.node('crate:' + name, name, sources=['crates/' + name]) for name in ['tetra-core', 'tetra-config', 'tetra-pdus', 'tetra-saps', 'tetra-entities']]
        return self.node('radio', '02 · Funkplattform – Basisstation & Mobile Station', [
            self.node('bs', 'TBS-Funkstack · PHY bis SNDCP', modules, sources=['bins/bluestation-bs', 'crates/tetra-entities']),
            self.feature('dual', 'Dual Carrier & Nachbarzellen · Einzelzellenabnahme', ['Docs/wiki/zwei-funktraeger.md', 'Docs/wiki/mehrzellenbetrieb-und-ausfallverhalten.md'], ['Zwei Träger: Slotbelegung und MS-Verhalten prüfen', 'Carrierzahl belegt keine unabhängigen Kontrollkanäle', 'Nachbar-CRUD, Aussendung und Endgerätewirkung getrennt prüfen', 'Z02 / Z03 vor Seamless-Handover-Abnahme']),
            ms,
            self.node('crates', 'Gemeinsame Rust-Bausteine', crates, sources=['Cargo.toml']),
            self.feature('protocol-foundation', 'TLMC / TLPD / MLE / MM / CMCE · Fachfundament', ['Docs/roadmaps/protokollfundament-arbeitsliste.md', 'Docs/protocols/sap-primitive-und-routingmatrix.md'], ['Typfundament und SAP-Verdrahtung', 'Aktive Handler und Downlinkwege abgleichen', 'Historische Paketabschlüsse mit eigener Nachweisstufe', 'PDU-Treffer belegen keine vollständige ETSI-Konformität']),
        ], note='Funk-Echtzeitprozeduren bleiben lokal. Lokaler BS- und separater MS-Workspace haben unterschiedliche Runtime- und Abnahmegrenzen.')

    def services(self) -> dict:
        registry = tomllib.loads(read('system-backend/services.toml'))
        inventory = tomllib.loads(read('deploy/open-lab/inventory.example.toml'))
        rows = table_rows('Docs/services/README.md', r'^\w|^\[')
        purpose = {}
        for row in rows:
            match = re.match(r'^([a-z-]+) ', row[0])
            if len(row) >= 5 and match:
                purpose[match.group(1)] = row[1]
        groups = {
            'Funksteuerung & Teilnehmer': ['node-gateway', 'subscriber-core', 'group-core', 'mobility-core', 'call-control'],
            'Medien, Sprache & Telefonie': ['media-switch', 'recorder', 'media-library', 'sip-switch'],
            'SDS, Paketdaten & Anwendungen': ['sds-router', 'packet-core', 'ip-gateway', 'application-gateway'],
            'Bedienung & Arbeitsabläufe': ['control-room', 'iot-gateway', 'hardware-gateway', 'rf-monitor', 'alarm-workflow', 'task-workflow', 'asset-management', 'alert-service'],
            'Sicherheit, Regionen & Betrieb': ['security-core', 'kmf', 'transit', 'observability', 'deployment-core'],
        }
        lookup = {service['name']: service for service in registry['services']}
        children = []
        for group, names in groups.items():
            items = []
            for name in names:
                service = lookup[name]
                path = 'Docs/services/' + name + '/README.md'
                heads = [clean(line.lstrip('# ')) for line in read(path).splitlines() if line.startswith('## ') or line.startswith('### ')]
                details = ['Management-Beispielport: ' + str(service.get('management_port', '–')),
                           'Registry-Abhängigkeiten: ' + (', '.join(service.get('depends_on', [])) or 'keine deklariert'),
                           'Standort-/E2E-Abnahme: siehe Z01 / Z03 / Z05']
                details.extend('Dokumentierter Bereich: ' + short(h) for h in heads[:10])
                note = 'Aufgabe: ' + purpose.get(name, name) + '\nRegistry-Zugriff: ' + service.get('security_mode', 'dienstspezifisch') + '\nHost: ' + ('vollständige VM' if name == 'deployment-core' else 'eigene Rolle / geeigneter Servicehost')
                if name == 'subscriber-core':
                    note += '\nAktive TBS kündigt subscriber_policy=false an; lokale Whitelist bleibt maßgeblich.'
                if name == 'mobility-core':
                    note += '\nCore-Kontextmodell vorhanden; vollständige aktive MM-Anbindung und laufender Rufhandover offen.'
                if name == 'group-core':
                    note += '\nZ02.5 schließt die noch offenen zentralen Gruppenhandler auf der aktiven TBS.'
                if name == 'transit':
                    note += '\nNetCore-eigenes netcore-transit-v1; kein ETSI-ISI-Nachweis.'
                items.append(self.feature('service:' + name, name + ' · CODE VORHANDEN', [path, 'system-backend/' + name], details, note))
            children.append(self.node('service-group:' + group, group, items))
        runtime = [s['name'] for s in registry['services'] if s.get('runtime', True)]
        if set(runtime) != {name for names in groups.values() for name in names} or len(runtime) != 26:
            raise ValueError('Runtime registry changed; review all service groups.')
        if set(runtime) != {s['name'] for s in inventory['services']}:
            raise ValueError('Deployment inventory and runtime registry differ.')
        auxiliary = []
        extras = [
            ('provisioning-core', 'Provisioning Core · zusätzliche optionale Verwaltungsrolle'),
            ('directory', 'Directory · Metadaten, Namen, Geräte & Gruppen'),
            ('tts', 'Piper/TTS · zentrale Erzeugung im Media-Library-Umfeld'),
            ('tbs-connect', 'TBS Connect · historischer Brew-Router / Port 8081'),
            ('shared', 'Shared · Bibliotheken, Verträge & WebUI, kein Runtime-LXC'),
        ]
        for name, title in extras:
            auxiliary.append(self.node('extra:' + name, title, sources=['Docs/services/' + name + '/README.md', 'system-backend/' + name]))
        auxiliary.append(self.node('extra:brew', 'Python-Brew-Server · eigener Integrationspfad', sources=['Docs/components/misc/brew-server/README.md', 'misc/brew-server']))
        children.append(self.node('service-extras', 'Zusatzkomponenten & gemeinsame Bibliotheken', auxiliary))
        return self.node('services', '03 · SwMI – 26 reguläre Dienste + Zusatzkomponenten', children,
            note='26 Runtime-Rollen in Registry/Beispielinventory; shared ist Bibliothek. Provisioning Core, Directory, TTS und Altconnectoren werden separat eingeordnet. Diese Zahl ist keine Live-Flottenzählung.', sources=['Docs/services/README.md', 'system-backend/services.toml', 'deploy/open-lab/inventory.example.toml'])

    def operator(self) -> dict:
        return self.node('operator', '04 · Leitstelle, Bedienung & Clients', [
            self.feature('operator-native', 'Native Leitstellenoberfläche & Arbeitsplatzprofile', ['Docs/services/control-room/desktop-bedienoberflaeche.md', 'Docs/services/control-room/bedienplatzprofile.md', 'system-backend/control-room/ui'], ['Mehrere Fenster, Touch & adaptive Oberflächen', 'Karte, Teilnehmer-, Gruppen- und Statuslage', 'Mikrofon / Headset / PTT / Audio separat abnehmen', 'Lokale Bedienerprofile, Sitzungen und Ressourcenrechte']),
            self.feature('operator-core', 'Control Room – mehrere vorhandene Anwendungen', ['README.md', 'Docs/services/control-room/README.md', 'bins/netcore-control-room', 'system-backend/control-room/operator'], ['Python-Control-Room, nativer Client und Rust-Core unterscheiden', 'Operator-Kommandos, Lage, Incidents und Schichtbuch', 'Rufautorität bleibt im zuständigen Fachkern', 'IAM-Migration über Z04 / M5']),
            self.feature('dashboard', 'Lokales TBS-Dashboard & Desktop', ['crates/tetra-entities/src/net_dashboard', 'Docs/design/basisstations-dashboard-gestaltung.md', 'Docs/integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md'], ['Funkstatus, Teilnehmer, Gruppen, Rufe und Dienste', 'Carrier-, Telegram- und Nachbarzellenkonfiguration', 'WLAN über NetworkManager; KDE-/Lite-Auswahl im Image', 'Lokale Aufnahme / Abspielen / Playout fachlich abnehmen']),
            self.feature('directory', 'Funkverzeichnis, IDs, Gruppen & Statusmetadaten', ['Docs/services/directory/README.md', 'misc/ID-Server'], ['RadioID-kompatible Namensauflösung und SQLite', 'Metadaten von Zulassung und Serving-TBS trennen', 'Stabile Teilnehmer- und Gruppenidentitäten', 'Verzeichnispflege und Endgeräteanzeige prüfen']),
            self.feature('mobile-clients', 'Android / Hybrid / Zebra · späterer Produktstrang', ['Docs/wiki/ausbauziele-und-prioritaeten.md', MASTER], ['Z11: mobile Oberflächen und Hybrid-Anbindung', 'QR-/NFC-Ausgabe und Geräteinventur', 'Bestehende Asset- und IAM-Verträge verwenden', 'Geräte-, Offline- und Wiederkehrabnahme je Pilot'], 'Geplante bzw. archivierte Ausbauziele; keine aktuelle App-Auslieferung durch diese Map.'),
        ])

    def workflows(self) -> dict:
        modules = {
            'net_asterisk': 'Lokaler Asterisk / SIP-Fallback', 'net_audio': 'Audio-Sprechstelle',
            'net_audio_player': 'Stations-Playout', 'net_brew': 'Brew-Protokolladapter',
            'net_dapnet': 'DAPNET-Anbindung', 'net_echolink': 'EchoLink-Anbindung',
            'net_geoalarm': 'Geoalarm', 'net_media': 'Media-Verbindung',
            'net_meshcom': 'MeshCom-Anbindung', 'net_recorder': 'Lokaler Recorder',
            'net_snom': 'Snom-Anbindung', 'net_telegram': 'Telegram-Anbindung',
            'net_tts': 'Sprachausgabe', 'net_telemetry': 'Telemetrie',
            'net_control': 'Steuerverträge', 'net_control_room': 'TBS/Core-Anbindung',
        }
        local = [self.node('edge-adapter:' + name, title + ' · Quellmodul', sources=['crates/tetra-entities/src/' + name], note='Quellmodul vorhanden. Konfigurations-, Laufzeit- und Standortnachweis separat prüfen.') for name, title in modules.items()]
        return self.node('workflows', '05 · Integrationen & durchgehende Arbeitsabläufe', [
            self.feature('flow-sds', 'SDS / Status / GPS → Router → Leitstelle / HA', ['Docs/services/sds-router/README.md', 'Docs/services/iot-gateway/home-assistant-anbinden.md', MASTER], ['ISSI / GSSI / Ziel und Serving-TBS zuordnen', 'Zustellbericht, Offline-Queue und Rückweg prüfen', 'MQTT-Verbindung allein belegt keine SDS-Endzustellung', 'Deduplizierung, Neustart und Abhängigkeitenausfall']),
            self.feature('flow-iot', 'MQTT / Home Assistant / Homematic → quittierte Aktion', ['Docs/services/iot-gateway/mqtt-nachrichten.md', 'Docs/services/iot-gateway/homematic-ip-anbinden.md', 'Docs/contracts/kommandos-und-bestaetigungen-v1.md'], ['netcore-event-v1, Befehle und korrelierte ACKs', 'Default-Deny und Geräte-/Ressourcenpolicy', 'Tatsächlichen Aktorzustand zurückmelden', 'Retry, Timeout und Wiederkehr']),
            self.feature('flow-alert', 'Warnzentrale, Alarmakte & Eskalation', ['Docs/services/alert-service/README.md', 'Docs/services/alarm-workflow/README.md'], ['NINA / KATWARN / eigene standortbezogene Meldungen', 'Quittierung, Eskalation und fachlicher Abschluss', 'Sendefreigabe und API-Zugriff dienstspezifisch', 'TBS-Annahme, Zustellung und Lesebestätigung unterscheiden']),
            self.feature('flow-task', 'WAP / Status → Auftrag → Asset / Wartung', ['Docs/services/task-workflow/README.md', 'Docs/services/asset-management/README.md', 'Docs/contracts/auftragsmodell-v1.md'], ['Strukturierte Formulare und Auftragszustände', 'Geräteausgabe, Personen, Wartung und Inventur', 'Korrelations-ID über den gesamten Rückweg', 'Geschlossenen Auftrag statt bloßem UI-ACK abnehmen']),
            self.feature('flow-media', 'TTS / Medienfreigabe → Ruf → hörbare Aussendung', ['Docs/services/media-library/README.md', 'Docs/services/tts/README.md', 'Docs/services/recorder/README.md'], ['Audio-Assets, Vorschau, TTS und Freigabe', 'Lokaler / zentraler Recorder und NAS-Pfad unterscheiden', 'Vollständige hörbare Aussendung und Rufabbau', 'Recorder-/NAS-Ausfall darf den RF-Pfad nicht blockieren']),
            self.feature('flow-sip', 'SIP / PBX → TBS → bidirektionales RTP', ['Docs/services/sip-switch/architektur.md', 'Docs/roadmaps/zentraler-netzbetrieb.md'], ['Zentraler SIP-Switch und lokaler TBS-Asterisk', 'Private SIP-Daten und Rolleninstallation korrekt zuordnen', 'SIP-Signalisierung und RTP getrennt prüfen', 'Fallback für neue Rufe und Wiederkehr']),
            self.feature('flow-packet', 'Paketdaten / SNDCP → Packet Core → IP / WAP', ['Docs/services/packet-core/README.md', 'Docs/services/ip-gateway/README.md', 'Docs/guides/wap/wap-portalreferenz-und-laufzeitgrenze.md'], ['Mehrere PDP-Kontexte, Fragmentierung und Flow Control', 'Teilnehmer-/Kontextzuordnung in beide Richtungen', 'TUN-Routenschutz Z02.3', 'HTTP/WAP, DNS und Endgerätedatenpfad']),
            self.node('edge-adapters', 'Vorhandene lokale Adapter & Anschlüsse', local),
            self.feature('future-bridges', 'FRN / Zello / RX-Gateway · spätere Brücken', [MASTER, 'Docs/wiki/ausbauziele-und-prioritaeten.md'], ['Z11: Pilot nach geprüftem Grundbetrieb', 'Z11.1: Decoder, Netz-/ISSI-/GSSI-Mapping und Brew', 'Receive-only-Gruppen mit netzseitiger Sendesperre', 'Keine bidirektionale Kopplung oder ISI aus einem RX-Stream ableiten'], 'Planung / Archivideen; vorhandene Adapter sind keine Abnahme dieser Brücken.'),
        ])

    def deployment(self) -> dict:
        return self.node('deployment', '06 · Deployment, Netzwerk & laufender Betrieb', [
            self.feature('deploy-controller', 'Deployment-Controller & Agenten · Code vorhanden', ['Docs/services/deployment-core/README.md', 'system-backend/deployment-core'], ['Vollständige Ubuntu-VM; Agent pro bestehendem Host', 'Git-Ref auf vollständigen Commit pinnen', 'SQLite-Jobs, Serialisierung, Logs und Status', 'Plan / Deploy / Readiness und Wiederholung abnehmen']),
            self.feature('deploy-inventory', 'Open-Lab-Inventar, LXC & Ready-Schranke', ['Docs/deployment/open-lab/README.md', 'deploy/open-lab'], ['26 Rollen und gemeinsame Konfigurationsverträge', 'Abhängigkeiten, valide Pläne und gerenderte Units', 'Semantisch negatives Ready stoppt Folgeinstallation', 'Bestandskonfiguration und Daten bei Updates erhalten']),
            self.feature('deploy-image', 'Pi-Imagebuilder, Stationsprofile & Firstboot', ['Docs/services/deployment-core/README.md', 'Docs/integration/Z01-2026-10-07/imagebuilder-vm119.md', 'system-backend/deployment-core'], ['ARM64-NetCore, SoapySX und Codec vorinstallieren', 'Profil: Name / MCC / MNC / ISSI / LA / CC', 'KDE / Lite, Benutzer / SSH, WLAN und optional VPN', 'Build, Manifest, Download, SD-Boot und Funk einzeln belegen']),
            self.feature('deploy-discovery', 'Discovery, Seeds, Peers & Endpunktcache', ['Docs/services/deployment-core/README.md', 'system-backend/deployment-core/discovery.py'], ['Multicast im gleichen Layer-2-Netz', 'Unicast-Seeds über geroutete Netze / VPN', 'Rollenauflösung, Konflikte, Lease und Wiederkehr', 'Controllerausfall darf den Funkbetrieb nicht blockieren']),
            self.feature('deploy-vpn', 'Pi-VPN-Heimnetzregel & WLAN', ['Docs/services/deployment-core/README.md', 'Docs/wiki/ausbauziele-und-prioritaeten.md'], ['Auto-VPN außerhalb ausdrücklich vertrauter Netze', 'LAN-/WLAN-Erkennung und NetworkManager', 'Trennung von RF-Standortdaten und Netzkonfiguration', 'Realer VPN-/WLAN-Wechsel am Pi noch gesondert abnehmen']),
            self.feature('deploy-update', 'Flotten-/Einzelupdates & VM-Rezept', ['Docs/deployment/netcore-dienste-aktualisieren.md', 'Docs/integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md'], ['Vorbereitung / Build vor Agentenänderung', 'Neue Assets, Dateihashes und laufenden Prozess prüfen', 'Runtime-Quittung und Commitmarker nach Erfolg', 'VM119, externe Anwendungen und Pi-Images eigene Schritte']),
            self.feature('deploy-observability', 'Syslog, Metriken, Diagnose & NAS-Archiv', ['Docs/services/observability/systemlogs-sammeln-und-archivieren.md', 'Docs/integration/Z01-2026-10-07/anlagenabnahme-z01-4.md'], ['TCP-Syslog, Vorschau, Raw und gzip-NAS-Archiv', 'Mountguard, begrenzte Puffer und Fehlerwiederkehr', 'Lokale CT136-Teilabnahmen datiert erhalten', 'Echter NFS-Stall / Flottensender / Dauerlauf separat']),
            self.feature('deploy-recovery', 'Betriebsabnahme, Ausfälle & Releasegrenzen', [MASTER, 'Docs/testing/e2e/README.md', 'Docs/wiki/inbetriebnahme-und-abnahme.md'], ['Z01.4 bleibt Teilabnahme / in Arbeit', 'Dienst-, Gateway-, VPN-, Speicher- und Stromausfall', 'Installierter Commit, Konfiguration und Artefakthash', 'Gemeinsames Release erst nach passenden Fachgates']),
        ])

    def security_drive(self) -> dict:
        iam = self.milestones(IAM, 'M', 'iam', 'IAM · M0–M8 · GEPLANT')
        drive = self.milestones(DRIVE, 'D', 'drive', 'NetCore Drive · D0–D9 · GEPLANT')
        self.relate('drive:D7', 'drive:D8', 'Plugin-Vertrag / Basismodule zuerst')
        self.relate('drive:D7', 'drive:D9', 'Plugin-Vertrag / Basismodule zuerst')
        return self.node('security-drive', '07 · Sicherheit, IAM, Drive & Browser-Plugins', [
            self.feature('security-scope', 'Web-IAM, TETRA-Security & KMF trennen', ['Docs/services/security-core/README.md', 'Docs/services/kmf/README.md', 'Docs/services/transit/grenze-zu-etsi-isi.md'], ['Open-Lab-Zugriff ist dienstspezifisch', 'Security Core / Lab-Provider und KMF-Vault vorhanden', 'Normative Funkauthentisierung / HSM / Air-OTAR eigene Abnahme', 'NetCore-Transit und standardisiertes ETSI ISI unterscheiden']),
            iam,
            drive,
            self.feature('drive-contract', 'Drive-Basis · lokaler Betrieb vor späterem SSO', [DRIVE], ['Lokale Konten, Gruppen und stabile Identitäts-IDs', 'Interne / externe Ordnerfreigaben und Ressourcenrechte', 'Upload, Vorschau, Versionen, Papierkorb und Sync', 'D6: Konten-/Gruppenmigration mit Eigentums- und Freigabeerhalt', 'D0–D5 und D7–D9 warten nicht auf D6'], 'Backendentscheidung offen; Nextcloud Files + PostgreSQL ist eine Empfehlung, keine implementierte Festlegung.'),
            self.feature('drive-plugins', 'Plugin-Familien · D7 / D8 / D9', [DRIVE], ['D7: PDF, Bilder, Audio und Video', 'D8: Word, Tabellen und Präsentationen', 'D9: 3D/CAD, Schaltpläne und Vektorgrafiken', 'Manifest, Format-/Fähigkeitsmatrix und Isolation', 'Viewer, Editor und Konverter getrennt deklarieren', 'Update / Deaktivierung / Rechte / Export prüfen']),
            self.feature('secure-ops', 'Z10 · Betrieb, HA & Regionen', [MASTER], ['IAM-Pilot und Dienstmigration als Voraussetzung', 'Last, Dauerlauf und Wiederherstellung messen', 'Backend-HA ist eigener Entwurf; Edge-Fallback separat', 'Mehrregionen, Funk-Security und ISI je eigene Abnahme']),
        ])

    def hardware(self) -> dict:
        return self.node('hardware', '08 · Hardware, HF & spätere Produktideen', [
            self.feature('hw-sx', 'SXceiver / SoapySX / Pi · Hardware-Referenzen', ['sxxcvr-main', 'Docs/wiki/hardware-sdr-und-hf-aufbau.md', 'Docs/integration/Z01-2026-10-07/imagebuilder-vm119.md'], ['Treiberrevision, SPI, GPIO und Dienstbenutzer', 'GPIO-Neustartpersistenz und SXceiver-Initialisierung', 'Frequenzen, Kanäle, Gains und Profilabgleich', 'Streaming, RX-Recovery und On-Air getrennt prüfen']),
            self.feature('hw-pa', 'Little-PA-V2 · Schaltplan, Platine & Fertigungsdaten', ['PA', 'Docs/components/PA/README.md'], ['TCXO-Insel, Bandpass und PA-/Bypass-Variante', 'Revision und tatsächliche Bestückung abgleichen', 'Elektrische / thermische / HF-Abnahme eigenständig', 'Dateibestand ist keine neue Fertigungsfreigabe']),
            self.feature('hw-rack', 'GPIO-/Rack-Baseboard · geplantes Ausbauziel', ['Docs/wiki/ausbauziele-und-prioritaeten.md', MASTER], ['Sensorik, Lüfter und Statusanzeigen', 'Watchdog, Stromversorgung und echte HAT-Pinbelegung', 'Footprints / PCB / EMV und Netzspannungsbereich', 'Mit Hardware Gateway / RF Monitor verbinden']),
            self.feature('hw-measure', 'Reale Aktoren & RF-Messhardware · Z08', ['Docs/services/hardware-gateway/README.md', 'Docs/services/rf-monitor/README.md', MASTER], ['Rack- und Umgebungstelemetrie', 'Kalibrierte HF-Messung und Grenzwerte', 'Virtuelle Rückmeldung vs. physische Wirkung', 'Timeout, Wartung und Wiederkehr']),
            self.feature('hw-workplace', 'Leitstellen-Audio / USB-PTT / mobile Racks', ['Docs/wiki/ausbauziele-und-prioritaeten.md', MASTER], ['Mikro, Headset, PTT und Bedienplatz', 'WERMA-/Rackanzeige nach praktischem Pilot', 'QR-/NFC-/Zebra-Inventur über Asset-Verträge', 'Archivierte Varianten vor Umsetzung neu prüfen']),
            self.node('hw-assets', 'Archivassets & vorhandene Referenzmodelle', sources=['Docs/archive/assets'], note='Alle abgelegten CAD-, Schaltplan-, Bild- und Referenzassets sind im Repository-Inventar enthalten. Historische Entwürfe werden nicht als aktueller installierter Hardwarestand ausgegeben.'),
        ])

    def quality(self) -> dict:
        workflows = [p for p in self.files if p.startswith('.github/workflows/')]
        return self.node('quality', '09 · Qualität, ETSI, Tests & Upstream-Quellen', [
            self.feature('qa-proof', 'Vier Nachweisstufen mit getrennten Grenzen', ['Docs/wiki/projektstand-und-nachweise.md', MASTER], ['Code vorhanden', 'Statisch / softwareseitig geprüft', 'Lab / Anlagenpilot geprüft', 'On Air mit identifiziertem Build und Endgerät geprüft', 'Historische PASS-Dateien sind keine heutige Flottenabnahme']),
            self.feature('qa-protocol', 'ETSI-Quellenregister & Protokollinventuren', ['Docs/protocols/etsi-quellenregister.md', 'Docs/protocols/etsi-pdu-konformitaetsmatrix.md', 'Docs/protocols/sap-primitive-und-routingmatrix.md', 'Docs/protocols/zustandsmaschinen-inventur.md'], ['EN-/TS-Ausgaben und Kapitelbezug der Quelle', 'PDU-Codecs, Golden Vectors und Bitgrenzen', 'SAP-Typen, Router und tatsächliche Handler', 'Zustandsmaschinen, Timer und Fehlerfälle', 'Normreferenz / statischer Treffer ist kein Konformitätszertifikat']),
            self.feature('qa-gaps', 'Offene Runtime-Übergänge & Robustheit', ['Docs/protocols/implementierungsluecken-und-laufzeitrisiken.md', 'README.md', MASTER], ['Z02: Restore, Watchdog, TUN-Schutz und Gruppenhandler', 'Subscriber-Policy und MM-Kontexttransfer aktiv anbinden', 'Parser, Panic-/Unimplemented-Pfade und Fuzzing', 'Fundstellen nach Erreichbarkeit bewerten; keine Fehlerrate ableiten']),
            self.node('qa-ci', 'GitHub-Workflows & statische Gates', [self.node('qa-workflow:' + p, PurePosixPath(p).name, sources=[p]) for p in workflows], sources=['tools'], note='Vorhandene Workflowdefinitionen; kein neuer Funk- oder Anlagenlauf durch diesen Dokumentationsauftrag.'),
            self.feature('qa-e2e', 'Laborrunner & echte Endgeräteabnahme', ['Docs/testing/e2e/README.md', 'tests/e2e', 'Docs/wiki/inbetriebnahme-und-abnahme.md'], ['Korrelierte Reports, Trace-/Job-/Ruf-IDs', 'Positive / negative Fälle und Wiederkehr', 'Motorola / Sepura, Gerätestand und Codeplug festhalten', 'Dual Carrier, Rufqualität und Handover-Unterbrechung messen']),
            self.node('qa-upstream', 'Upstream-/Repository-Vergleiche · datierte Quellen', sources=['Docs/comparisons', 'Docs/roadmaps/backend-entwicklung.md'], note='Vorhandene Vergleichsdokumente und Herkunftsreferenzen sind erfasst. Dieser Auftrag führt keinen neuen externen Upstreamvergleich aus. Historische fremde Funktionen werden nicht als übernommen markiert.'),
            self.node('qa-history', 'Historische Phasen & Gesprächsarchive', sources=['Docs/roadmaps/backend-entwicklung.md', 'Docs/roadmaps/projektverlauf-bis-2026-10-09.md', 'Docs/archive'], note='Historische Paketnummern und Gesprächsstände bleiben Referenzen. Globale Priorität kommt aus der zentralen Z-Roadmap; aktuelle technische Aussagen benötigen einen passenden Nachweis.'),
        ])

    def repository(self) -> dict:
        tree: dict = {}
        for path in sorted(self.files):
            cursor = tree
            parts = PurePosixPath(path).parts
            for part in parts[:-1]:
                cursor = cursor.setdefault(part, {})
            cursor[parts[-1]] = self.files[path]

        def convert(name: str, value: dict, path: str) -> dict:
            if 'sha' in value and 'type' in value and 'path' in value:
                item = value
                return self.node('file:' + path, name,
                    note='Versionierter Pfad: ' + path + '\nGit-Objekt: ' + item['sha'] + '\nGröße: ' + str(item['size']) + ' Bytes\nTyp/Modus: ' + item['type'] + '/' + item['mode'] + '\n' + self.url + '/blob/' + self.commit + '/' + path + '\nInventarnachweis; keine Aussage über Laufzeit, Abnahme oder binären Dateiinhalt.')
            children = [convert(child, content, path + '/' + child if path else child) for child, content in value.items()]
            count = sum(p == path or p.startswith(path + '/') for p in self.files)
            return self.node('directory:' + path, name + ' · ' + str(count) + ' Dateien', children,
                note='Quellpfad: ' + path + '\n' + self.url + '/tree/' + self.commit + '/' + path)

        nodes = [convert(name, value, name) for name, value in tree.items()]
        return self.node('repository', '10 · Repository-Inventar – ' + str(len(self.files)) + ' Dateien', nodes,
            note=self.inventory['scope'] + '\nDateien werden mit Pfad, Größe und Git-Objekt erfasst. Fremdcode, Archive, Binärassets und historische Belege bleiben erkennbar bei ihrer Quelle. Das Inventar ersetzt keine semantische Einzelprüfung jedes Binärassets.', sources=['README.md'])

    def build(self) -> list[dict]:
        pi_source = 'Docs/integration/Z01-2026-10-07/evidence/pi-kde-acceptance-2026-10-13.json'
        pi_report = json.loads(read(pi_source))
        pi_labels = {
            'voice_calls': 'Rufe', 'sds': 'SDS', 'local_recording': 'Lokale Aufnahme',
            'recording_rf_playback': 'Aufnahme über Funk aussenden',
            'wifi_radio_toggle': 'WLAN ein-/ausschalten', 'wifi_connection': 'WLAN verbinden',
            'neighbor_ui_edit': 'Nachbarzelle im UI bearbeiten',
        }
        pi_children = [self.node('orientation:pi:' + key, title + ' · BETREIBER BESTÄTIGT')
                       for key, title in pi_labels.items() if pi_report['operator_reported_pass'].get(key) is True]
        pi_children.append(self.node('orientation:pi:pending', 'Weitere Abnahme: MS-Zellwechsel, SIP, VPN, Dauerlauf & LXC',
            note='Noch offen laut Betreiberbeleg:\n' + '\n'.join(pi_report['pending_acceptance']) + '\nDie neue Nachbar-Ankündigungskorrektur war auf dem getesteten Pi noch nicht installiert.'))
        pi = self.node('orientation:pi', '13.10. · KDE-Pi: Rufe, SDS, Aufnahme & WLAN bestätigt', pi_children,
            note='Betreiberbericht: ' + pi_report['operator_reported_at'] + '\nStation: ' + pi_report['station'] + '\nImage-Quellcommit: ' + pi_report['image_artifact']['commit'] + '\nBuild: ' + pi_report['image_artifact']['build_id'] + '\nDie bestätigten Bedienfunktionen werden als Betreiberbeleg übernommen. Das begrenzte TX-Zeitfenster, unbekannte Ursachen der zwei Dienstneustarts und fehlende Hardware-Statusabfrage sind keine Dauerlaufabnahme. UI-Nachbarpflege und MS-Decodierung/Zellwechsel bleiben getrennte Aussagen.', sources=[pi_source])
        orientation = self.node('orientation', '00 · Einstieg & Leseschlüssel', [
            self.node('orientation:source', 'Momentaufnahme · main@' + self.commit[:8] + ' · ' + self.inventory['captured_at'], note='Vollständiger Quellcommit: ' + self.commit + '\nZeitzone: Europe/Berlin\nDie Map ist eine dateibasierte Momentaufnahme; keine automatische GitHub-Synchronisation.'),
            self.node('orientation:next', 'Aktuell: Z01.4 · Update / Image / Anlagenabnahme', sources=[MASTER], note='Der zentrale Z01-Block ist in Arbeit. Die Einzelabnahmen und verbleibenden Punkte stehen in Z01.4. Z02 kann gezielt parallel bearbeitet werden; IAM-/Drive-Architektur hat eigene Eintrittskriterien.'),
            pi,
            self.node('orientation:free', 'Xmind Free · 1 Blatt, Themen, Notizen & Beziehungen', note='Keine Premium-Tasks, Gantt, Assignees, Mehrblattplanung, Hyperlink-Elemente oder Anhänge. Status/Priorität sind normale Texte. Quellpfade und URLs stehen als kopierbarer Klartext in den Notizen. Basislayout und Einklappen verwenden die normalen Topic-Funktionen.'),
            self.node('orientation:read', 'Zuerst Überblick; dann gezielt aufklappen', note='Die Hauptäste sind sichtbar. Detailäste und das komplette Dateiinventar starten eingeklappt. Mit Themen öffnen/schließen und der Suche nach Z-/M-/D-ID, Dienstname oder Dateiname navigieren. Beziehungen markieren ausgewählte fachliche Voraussetzungen; vollständige Bedingungen stehen in den Notizen.'),
            self.node('orientation:status', 'Status ≠ Nachweisstufe ≠ aktueller Anlagenzustand', note='OFFEN / GEPLANT / IN ARBEIT sind Aufgabenstatus. Code-, statische, Lab- und On-Air-Nachweise sind getrennte Stufen. Source-Snapshot und alte Prüfzeitpunkte bleiben erkennbar; es werden keine Prozentwerte, verbindlichen Termine oder zusätzlichen Implementierungen erfunden.', sources=['Docs/wiki/projektstand-und-nachweise.md']),
        ], folded=False)
        branches = [orientation, self.roadmap(), self.radio(), self.services(), self.operator(), self.workflows(), self.deployment(), self.security_drive(), self.hardware(), self.quality(), self.repository()]
        root = self.node('root', 'NetCore-Tetra\nGesamtübersicht & Roadmap', branches,
            note='Quelle: ' + self.url + '\nQuellcommit: ' + self.commit + '\nStand: ' + self.inventory['captured_at'] + ' / Europe/Berlin\n' + str(len(self.files)) + ' Dateien im Ausgangsinventar; technische Priorität aus NETCORE-MASTER-01.\nDie Map verbindet Roadmap, Fachumfang und einen vollständigen Dateibaum. Sie ändert keine technischen Abnahmen.', sources=[MASTER, 'README.md'], folded=False)
        root['structureClass'] = 'org.xmind.ui.map.unbalanced'
        return [{'id': identifier('sheet:overview'), 'title': 'NetCore-Tetra – Gesamtübersicht', 'rootTopic': root, 'relationships': self.relationships}]


def walk(topic: dict):
    yield topic
    for child in topic.get('children', {}).get('attached', []):
        yield from walk(child)


def validate(builder: MapBuilder, sheets: list[dict]) -> dict:
    if len(sheets) != 1:
        raise ValueError('Xmind Free overview must use exactly one sheet.')
    topics = list(walk(sheets[0]['rootTopic']))
    ids = [topic['id'] for topic in topics]
    if len(ids) != len(set(ids)):
        raise ValueError('Duplicate topic IDs.')
    for topic in topics:
        if not topic.get('title') or not isinstance(topic['title'], str):
            raise ValueError('Missing title.')
        forbidden = {'task', 'taskInfo', 'href', 'attachments', 'image', 'numbering', 'style'} & topic.keys()
        if forbidden:
            raise ValueError('Not in the deliberately limited free-feature profile: ' + str(forbidden))
        if 'notes' in topic and set(topic['notes']) != {'plain'}:
            raise ValueError('Only plain notes are used.')
        if topic.get('branch') not in (None, 'folded'):
            raise ValueError('Invalid fold state.')
    for relation in builder.relationships:
        if relation['end1Id'] not in ids or relation['end2Id'] not in ids:
            raise ValueError('Dangling relationship.')
    inventory_topics = {key[5:] for key in builder.nodes if key.startswith('file:')}
    if inventory_topics != set(builder.files):
        raise ValueError('Repository inventory coverage mismatch.')
    ids_expected = {f'Z{i:02d}' for i in range(1, 12)} | {f'Z01.{i}' for i in range(1, 5)} | {f'Z02.{i}' for i in range(1, 6)} | {'Z11.1'}
    if not ids_expected <= builder.nodes.keys():
        raise ValueError('Missing central work package.')
    return {'topics': len(topics), 'relationships': len(builder.relationships), 'inventory_files': len(builder.files), 'explicit_source_paths': len(builder.referenced), 'sheets': 1}


def legacy_xml(sheets: list[dict]) -> bytes:
    ET.register_namespace('', CONTENT_NS)
    tag = lambda name: '{' + CONTENT_NS + '}' + name
    content = ET.Element(tag('xmap-content'), {'version': '2.0'})
    for sheet in sheets:
        element = ET.SubElement(content, tag('sheet'), {'id': sheet['id']})

        def add_topic(parent, topic):
            attrs = {'id': topic['id']}
            if 'branch' in topic:
                attrs['branch'] = topic['branch']
            if 'structureClass' in topic:
                attrs['structure-class'] = topic['structureClass']
            current = ET.SubElement(parent, tag('topic'), attrs)
            ET.SubElement(current, tag('title')).text = topic['title']
            if 'notes' in topic:
                note = ET.SubElement(current, tag('notes'))
                ET.SubElement(note, tag('plain')).text = topic['notes']['plain']['content']
            if topic.get('children'):
                children = ET.SubElement(current, tag('children'))
                attached = ET.SubElement(children, tag('topics'), {'type': 'attached'})
                for child in topic['children']['attached']:
                    add_topic(attached, child)
        add_topic(element, sheet['rootTopic'])
        ET.SubElement(element, tag('title')).text = sheet['title']
        if sheet['relationships']:
            relationships = ET.SubElement(element, tag('relationships'))
            for relation in sheet['relationships']:
                item = ET.SubElement(relationships, tag('relationship'), {'id': relation['id'], 'end1': relation['end1Id'], 'end2': relation['end2Id']})
                ET.SubElement(item, tag('title')).text = relation['title']
    return ET.tostring(content, encoding='utf-8', xml_declaration=True)


def map_zip(sheets: list[dict], inventory: dict) -> bytes:
    entries = {
        'content.json': json.dumps(sheets, ensure_ascii=False, separators=(',', ':')).encode('utf-8'),
        'content.xml': legacy_xml(sheets),
        'metadata.json': json.dumps({'creator': {'name': 'NetCore-Tetra', 'version': '1'}, 'sourceCommit': inventory['commit'], 'snapshotDate': inventory['captured_at']}, ensure_ascii=False, separators=(',', ':')).encode('utf-8'),
    }
    entries['manifest.json'] = json.dumps({'file-entries': {name: {} for name in sorted(entries)}}).encode('utf-8')
    manifest = ET.Element('manifest', {'xmlns': MANIFEST_NS})
    for name in ['content.xml', 'content.json', 'metadata.json', 'manifest.json']:
        ET.SubElement(manifest, 'file-entry', {'full-path': name, 'media-type': 'text/xml' if name.endswith('.xml') else 'application/json'})
    entries['META-INF/manifest.xml'] = ET.tostring(manifest, encoding='utf-8', xml_declaration=True)
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, 'w', compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for name, value in sorted(entries.items()):
            info = zipfile.ZipInfo(name, (2026, 10, 13, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            archive.writestr(info, value, compresslevel=9)
    return buffer.getvalue()


def preview_svg(sheets: list[dict], inventory: dict) -> bytes:
    # This compact review image is not an Xmind export. The editable map holds
    # all details; here only the first few category names are shown.
    width, height = 1600, 1260
    root_x, root_y = 800, 680
    colors = ['#475569', '#2563eb', '#059669', '#0f766e', '#7c3aed', '#d97706', '#0284c7', '#be185d', '#64748b', '#4338ca', '#475569']
    chunks = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="#f7f8fa"/>',
        '<style>text{font-family:Segoe UI,Arial,sans-serif;fill:#172033}.title{font-size:33px;font-weight:650}.meta{font-size:17px;fill:#556176}.head{font-size:19px;font-weight:600}.detail{font-size:15px;fill:#526074}</style>',
        '<text x="55" y="65" class="title">NetCore-Tetra · Gesamtübersicht &amp; Roadmap</text>',
        f'<text x="55" y="98" class="meta">main@{inventory["commit"][:8]} · {inventory["captured_at"]} · {len(inventory["files"])} versionierte Dateien · 1 Xmind-Blatt</text>',
        '<text x="55" y="130" class="meta">Kompakte Orientierung. Fachdetails, Aufgaben, Nachweise und der vollständige Dateibaum liegen in der editierbaren Map.</text>']
    branches = sheets[0]['rootTopic']['children']['attached']
    for index, branch in enumerate(branches):
        left = index < 6
        column_index = index if left else index - 6
        x = 55 if left else 1105
        y = 180 + column_index * 170 + (80 if not left else 0)
        card_width, card_height = 440, 143
        cx, cy = (x + card_width if left else x), y + card_height / 2
        start_x = root_x - 112 if left else root_x + 112
        color = colors[index]
        chunks.append(f'<path d="M {start_x} {root_y} C {start_x + (-135 if left else 135)} {root_y}, {cx + (110 if left else -110)} {cy}, {cx} {cy}" fill="none" stroke="{color}" stroke-width="2" opacity="0.5"/>')
        chunks.append(f'<rect x="{x}" y="{y}" width="{card_width}" height="{card_height}" rx="12" fill="white" stroke="#dce2ea"/>')
        chunks.append(f'<rect x="{x}" y="{y+14}" width="4" height="{card_height-28}" rx="2" fill="{color}"/>')
        lines = textwrap.wrap(branch['title'], 43)
        for offset, line in enumerate(lines[:2]):
            chunks.append(f'<text x="{x+19}" y="{y+28+offset*24}" class="head">{html.escape(line)}</text>')
        details = branch.get('children', {}).get('attached', [])[:3]
        baseline = y + (68 if len(lines) == 2 else 56)
        for offset, child in enumerate(details):
            chunks.append(f'<text x="{x+19}" y="{baseline+offset*21}" class="detail">{html.escape(short(child["title"], 52))}</text>')
    chunks.extend([
        f'<rect x="{root_x-112}" y="{root_y-62}" width="224" height="124" rx="20" fill="#18263e"/>',
        f'<text x="{root_x}" y="{root_y-15}" text-anchor="middle" style="font-size:25px;font-weight:650;fill:white">NetCore-Tetra</text>',
        f'<text x="{root_x}" y="{root_y+17}" text-anchor="middle" style="font-size:17px;fill:#dce5f5">Gesamtfolge + Fachumfang</text>',
        f'<text x="{root_x}" y="{root_y+41}" text-anchor="middle" style="font-size:15px;fill:#bac8dd">Z01–Z11 · M0–M8 · D0–D9</text>',
        '<text x="55" y="1218" class="meta">Status und Nachweisstufe bleiben getrennt. Kalendertermine, Prozentwerte und zusätzliche Abnahmen werden nicht erfunden.</text>', '</svg>'])
    return '\n'.join(chunks).encode('utf-8')


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='Check generated map and preview without writing.')
    args = parser.parse_args()
    inventory = json.loads(INVENTORY.read_text(encoding='utf-8'))
    if inventory.get('schema_version') != 1 or not re.fullmatch(r'[0-9a-f]{40}', inventory['commit']):
        raise ValueError('Invalid source inventory version or commit.')
    paths = [entry['path'] for entry in inventory['files']]
    if len(paths) != len(set(paths)):
        raise ValueError('Duplicate inventory paths.')
    builder = MapBuilder(inventory)
    sheets = builder.build()
    stats = validate(builder, sheets)
    outputs = {MAP: map_zip(sheets, inventory), PREVIEW: preview_svg(sheets, inventory)}
    if args.check:
        stale = [str(path.relative_to(ROOT)) for path, data in outputs.items() if not path.is_file() or path.read_bytes() != data]
        if stale:
            raise SystemExit('Outdated generated files: ' + ', '.join(stale))
    else:
        for path, data in outputs.items():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
    with zipfile.ZipFile(io.BytesIO(outputs[MAP])) as archive:
        if archive.testzip() is not None:
            raise ValueError('ZIP CRC check failed.')
        json_ids = [node['id'] for node in walk(json.loads(archive.read('content.json'))[0]['rootTopic'])]
        xml = ET.fromstring(archive.read('content.xml'))
        xml_ids = [node.attrib['id'] for node in xml.iter('{' + CONTENT_NS + '}topic')]
        if json_ids != xml_ids:
            raise ValueError('JSON/XML topic order or coverage differs.')
    print(json.dumps({'result': 'PASS', 'check_only': args.check, 'source_commit': inventory['commit'], **stats,
        'xmind_bytes': len(outputs[MAP]), 'xmind_sha256': hashlib.sha256(outputs[MAP]).hexdigest()}, ensure_ascii=False))


if __name__ == '__main__':
    main()
