# Leitstelle

**Quellen:** [system-backend/control-room/operator](../../../../system-backend/control-room/operator) · [Repository-Root](../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../../roadmaps/gesamtroadmap.md).

Die CLI bietet Lageübersicht, Live-Dashboard, Teilnehmer-, Ruf- und Positionslisten
sowie typisierte Operatoraktionen. Der ausgelieferte Server läuft im OPEN LAB mit
`--no-auth`; dabei werden weder Benutzername noch Passwort benötigt.

Vom Repository-Hauptverzeichnis:

```bash
cargo build --locked --release -p netcore-control-room-operator
./target/release/netcore-control-room-operator --api http://CONTROL-ROOM-IP:9010 overview
./target/release/netcore-control-room-operator --help
```

## Optionaler Benutzerzugang

Bei aktivierter Serverauthentisierung unterstützt die CLI HTTP Basic Auth.
Die folgenden Beispiele betreffen dieses geschützte Profil; HTTP Basic braucht
zusätzlich einen passend eingerichteten HTTPS-Zugang, um Zugangsdaten vertraulich
zu übertragen.

Beispiel:

```bash
./target/release/netcore-control-room-operator \
  --api http://10.0.1.25:9010 \
  --username jan \
  --password '<passwort>' \
  overview
```

Passwort per `--password-file` oder Umgebungsvariable bereitstellen:

```bash
export NETCORE_CONTROL_ROOM_USER=jan
export NETCORE_CONTROL_ROOM_PASSWORD='<passwort>'
./target/release/netcore-control-room-operator --api http://10.0.1.25:9010 overview
```

Benutzerverwaltung erfordert im geschützten Serverprofil Adminrechte:

```bash
./target/release/netcore-control-room-operator users list
./target/release/netcore-control-room-operator users create --username operator1 --password '<pw>' --role operator --display-name 'Operator 1'
./target/release/netcore-control-room-operator users disable --username operator1
./target/release/netcore-control-room-operator users enable --username operator1
./target/release/netcore-control-room-operator users password --username operator1 --password '<neu>'
./target/release/netcore-control-room-operator users delete --username operator1
```

Profile initialisieren oder effektive Komfortwerte anzeigen:

```bash
./target/release/netcore-control-room-operator profiles init --api http://CONTROL-ROOM-IP:9010
./target/release/netcore-control-room-operator profiles show
```

Profile speichern API-Adresse, Benutzernamen, bevorzugte Node und Operator-ID,
keine Passwörter. Details: [Bedienplatzprofile](../bedienplatzprofile.md).
