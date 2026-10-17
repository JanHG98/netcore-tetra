# Telefonievermittlung: Funktionsprüfung im Labor

Die Befehle auf dem SIP-Switch-LXC ausführen; `IP-DES-SIP-SWITCH` und ISSI ersetzen. Eine erfolgreiche Route oder SIP-Registrierung belegt noch keine Sprache in beiden Richtungen. Audio, Anruferanzeige, Abbruch und zentralen Ausfall separat mit Lab-Telefon und Funkgerät abnehmen.


```bash
systemctl status asterisk netcore-sip-switch --no-pager --full
ss -lntup | grep -E ':8300|:5060|:10000'
curl -fsS http://IP-DES-SIP-SWITCH:8300/api/v1/status | python3 -m json.tool
curl -fsS 'http://IP-DES-SIP-SWITCH:8300/api/v1/resolve?direction=inbound&number=4010001&check_contact=false' | python3 -m json.tool
asterisk -rx 'pjsip show endpoints'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'dialplan show netcore-from-pbx'
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/sip-switch) · [Konfigurationsvorlagen](../../../../system-backend/sip-switch/config).
## Rufabbruch mit CHANUNAVAIL eingrenzen (17.10.2026)

Jans beobachteter Ruf an ISSI 5102 wird vom zentralen Resolver der TBS
SRV-M-TBS-01 zugeordnet und endet anschließend mit `CHANUNAVAIL`.
`CongestionInInfrastructure` im TBS-CMCE-Protokoll ist die bereits übersetzte
TETRA-Ursache; sie benennt nicht den fehlschlagenden SIP-Hop. Die aktuellen
Hosts sind aus der Entwicklungssitzung nicht erreichbar. Eine Anlagenreparatur
oder erfolgreiche Rufabnahme ist damit nicht belegt.

Im nativen SIP-Client wird ein unabhängig davon nachgewiesener Fehler behoben:
ACK auf 401/407 und andere 3xx–6xx-Antworten muss zur ursprünglichen INVITE-
Transaktion gehören. Bislang wurde dafür eine neue Via-Branch erzeugt; bei
401/407 war auch die CSeq bereits erhöht, und der To-Tag der Antwort fehlte.
Die Korrektur sendet zuerst den passenden ACK und danach höchstens einen
authentifizierten INVITE mit neuer CSeq/Branch. Ein begrenzter ACK-Cache bedient
wiederholte Fehlerantworten für 32 Sekunden auch nach Dialogabbau. Erneut
abgewiesene oder unvollständige Authentifizierungsaufforderungen beenden den
Ruf, statt ihn bis zum Timeout stehenzulassen.
[SIP-Vorgabe: RFC 3261, Abschnitt 17.1.1.3](https://www.rfc-editor.org/rfc/rfc3261.html#section-17.1.1.3).
Sechs Rust-Komponententests bestehen mit dem unverändert eingebundenen
geänderten SIP-Client und echten lokalen UDP-Datagrammen. Das Medienbackend
ist im isolierten lokalen Lauf nachgebildet. Die neue Authentifizierungs-
regression scheitert am vorherigen Client aus main@6d3b481 wie erwartet.
Der vollständige lokale Lauf scheitert am fehlenden SoapySDR-SDK; der
Linux-Lauf mit echtem Sprachcodec ist vorbereitet, aber noch nicht gestartet.
Dies ist keine Live-Abnahme des SIP-Switches, der FreePBX oder des Funkwegs.
[Datierter Prüfumfang](../../../integration/Z01-2026-10-07/evidence/sda-sip-korrekturen-2026-10-17.json).

Für den beobachteten eingehenden Ruf nach einem erneuten Fehlversuch auf
**SIP-Switch und TBS** ausführen:

```bash
sudo asterisk -rx 'pjsip show contacts'
sudo asterisk -rx 'pjsip show registrations'
sudo asterisk -rx 'core show channeltypes'
sudo journalctl -u asterisk --since '10 minutes ago' --no-pager -n 120
```

Auf der **TBS** zusätzlich:

```bash
sudo asterisk -rx 'database get netcore failover_mode'
```

Falls Asterisk seine Fehler ausschließlich in die Datei schreibt:

```bash
sudo tail -n 160 /var/log/asterisk/full
```

Damit prüfen: Kontakt am zentralen Switch, nativer TBS-Kontakt am lokalen
Asterisk, aktiver Vermittlungsmodus und konkrete SIP-/Kanalfehlermeldung.
Den Zentralweg oder den exklusiven FreePBX-Fallback erst auf Grundlage
dieser Befunde ändern. Die ACK-Korrektur ersetzt diese Diagnose nicht.
