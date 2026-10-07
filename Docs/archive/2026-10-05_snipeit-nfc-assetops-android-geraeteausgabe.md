# Brainstorming: Android AssetOps, Snipe-IT und NFC-Geräteausgabe

## Rahmen

- **Thema:** Android-NFC-Assetverwaltung auf Basis von Snipe-IT für Funkgeräte- und Geräteausgabe
- **Notizstand:** 2026-10-05
- **Repository:** `JanHG98/netcore-tetra`
- **Zielbranch:** `Archiving`
- **Vor dem Schreiben geprüfter Branch-Head:** `83cdd56e1450be16437f1b94bb4f2db035149460`
- **Vor dem Schreiben geprüfter Tree:** `68b221e4ea958e34a8ae26f02da880952ff60a15`
- **Repository-Abgleich:** Code-Suchen auf dem Branch nach `Snipe-IT`, `NFCBrigde` und `AssetOps` ergaben jeweils **0 Treffer**. Der lokal entwickelte Android-Code ist damit am geprüften Stand **nicht Bestandteil dieses Repositories**.
- **Tokenwechsel offen:** Ein echter Snipe-IT-API-Token wurde offengelegt und nicht übernommen. Er ist als kompromittiert zu behandeln; ein Austausch ist noch nicht bestätigt.

> Statusbegriffe in diesem Dokument:
> - **Idee** = diskutiert, noch nicht zugesagt.
> - **beschlossen/geplant** = gewünschter Zielzustand.
> - **implementiert** = Code im lokalen Entwicklungsstand vorhanden beziehungsweise laut Betriebsrückmeldung in die Android-App eingebaut.
> - **getestet** = konkretes Ergebnis, Screenshot oder API-Resultat liegt vor.
> - **im Betrieb bestätigt** = Funktion auf dem realen Android-Gerät gegen die reale Snipe-IT-Instanz ausdrücklich bestätigt.

---

## 1. Ziel und Ausgangslage

Ausgangsziel war, vorhandenes **Snipe-IT** als Geräteverwaltung zu behalten und eine möglichst reibungslose NFC-Bedienung für Funkgeräte und Benutzerkarten zu erhalten. Der ursprüngliche Wunsch war ausdrücklich **kein Browser-Workflow** und möglichst **keine zusätzliche eigene App**. Es sollte sich wie ein OOBE-/native-App-Workflow anfühlen.

Ausgangspunkt:

- Snipe-IT läuft im lokalen LAN.
- Der DNS-Name der Instanz wurde als `CT-H-APP-02` genannt; die WebUI war damit im Browser erreichbar.
- Für die spätere Eigen-App wurde als direkte LAN-Adresse `http://10.0.1.15` verwendet.
- Es existiert eine Android-App „Assets Manager“ für Snipe-IT mit Paketname:
  `com.diegogarciadev.assetsmanager.snipeit`.
- Parallel wurde in Android Studio eine eigene App mit Package
  `de.jan.nfcbrigde` erstellt.
- Primärer Geräte-Identifier sollte die NFC-Tag-UID ohne Leerzeichen sein und direkt als **Snipe-IT Asset Tag** genutzt werden.
- Benutzerkarten sollten zunächst über die **Mitarbeiternummer / employee_num** eindeutig einem Snipe-IT-Benutzer zugeordnet werden.

Die Deep-Link-/Patchversuche mit der vorhandenen App wurden durch eine eigene kleine Android-Oberfläche abgelöst. Diese nutzt direkt die **Snipe-IT REST API**.

---

## 2. Chronologie und endgültige Entscheidungen

### 2.1 Versuch: vorhandene Snipe-IT-Android-App direkt öffnen

**Getestet / verworfen.**

Zunächst wurde versucht, die vorhandene Snipe-IT-App aus einer kleinen NFC-Bridge-App heraus direkt auf die Detailansicht eines Assets zu öffnen.

Ermittelte Appdaten:

- App-Name: „Assets Manager“
- Version: `0.7.7-SnipeIT`
- Paket: `com.diegogarciadev.assetsmanager.snipeit`
- MainActivity:
  `com.diegogarciadev.assetsmanager.MainActivity`
- Weitere durch Dekompilierung identifizierte relevante Activities:
  - `SearchActivity`
  - `EditActivity`
  - `AssetsListActivity`
  - `AssetsListAllActivity`
  - `CheckOutActivity`
  - `MaintenancesActivity`
  - `ManageDataActivity`

ADB-Aufruf zur Auflösung der MainActivity war erfolgreich:

```powershell
.\adb shell cmd package resolve-activity --brief com.diegogarciadev.assetsmanager.snipeit
```

Ergebnis:

```text
com.diegogarciadev.assetsmanager.snipeit/com.diegogarciadev.assetsmanager.MainActivity
```

Die MainActivity ließ sich aus der Bridge-App öffnen, landete aber immer im Hauptmenü.

Die Manifest-/Package-Analyse zeigte nur zwei exportierte Activity-Einstiegspunkte im Resolver:

- `MainActivity`
- ZXing `CaptureActivity`

Direktes Öffnen von `EditActivity` via ADB:

```powershell
.\adb shell am start -n com.diegogarciadev.assetsmanager.snipeit/com.diegogarciadev.assetsmanager.EditActivity --es ASSET_ID 2
```

führte korrekt zu:

```text
java.lang.SecurityException:
Permission Denial ... EditActivity ... not exported
```

**Endgültige Entscheidung:** Die bestehende Snipe-IT-App sollte nicht mehr als Ziel für Deep Links verwendet werden.

---

### 2.2 Dekompilierung der Snipe-IT-App

**Getestet.**

Die App wurde mit JADX untersucht. Wesentliche Befunde:

#### MainActivity

Die Button-Handler öffneten intern `SearchActivity` mit Extras wie:

- `TYPE=EDIT`
- `TYPE=CHECKIN`
- `TYPE=CHECKOUT`
- `TYPE=MAINTENANCE`
- `TYPE=AUDIT`
- `MODE=USER|LOCATION|ASSET`

#### SearchActivity

Die interne Suche setzt den eingegebenen Asset Tag in `f3969a0` und ruft über `x0.x` folgende Snipe-IT-Pfade auf:

1. Wenn Eingabe URL-artig ist:
   `/api/v1/hardware/{id}`
2. sonst:
   `/api/v1/hardware/bytag/{asset_tag}`
3. Fallback:
   `/api/v1/hardware/byserial/{serial}`

Die App befüllt danach Modell, Seriennummer, Name, Eigentümer und Notizen.

#### Interne Edit-/Checkout-Flows

Aus `x0.t`:

- EditActivity bekommt `ASSET_ID`
- CheckOutActivity bekommt:
  - `MODE`
  - `ASSET_ID`
  - `ASSET_NAME`

Diese Analyse bestätigte, dass die vorhandene App intern genau den gewünschten Workflow hat, die relevanten Activities aber nicht exportiert sind.

---

### 2.3 Patch-Versuch der bestehenden APK

**Teilweise implementiert / letztlich verworfen.**

Die App wurde mit apktool dekompiliert, Manifest-Anpassungen wurden versucht und eine gepatchte APK gebaut.

Arbeitsverzeichnis:

```text
C:\temp\snipepatch
```

Vorhandene Dateien laut PowerShell:

- `snipeit.apk`
- `snipeit_dec/`
- `snipeit_patched_unsigned.apk`
- `snipeit_patched_aligned.apk`
- `snipepatch.jks`
- `apktool.bat`
- `apktool_3.0.1.jar`

Signaturprüfung mit Android Build Tools 36.1.0:

```powershell
& "$env:LOCALAPPDATA\Android\Sdk\build-tools\36.1.0\apksigner.bat" verify --verbose .\snipeit_patched_aligned.apk
```

Ergebnis:

- v1: true
- v2: true
- v3: true
- v3.1: false
- v4: false
- 1 Signer

Die APK stammte aus einem Split-APK-Paket. Originale Package-Pfade enthielten:

- `base.apk`
- `split_config.de.apk`
- `split_config.xxhdpi.apk`

Ein Einzelinstallationsversuch der gepatchten Base-APK scheiterte erwartbar mit:

```text
INSTALL_FAILED_MISSING_SPLIT
```

Zwischenzeitlich war die Original-App deinstalliert, weshalb alte `/data/app/.../`-Pfade ungültig wurden; nach Neuinstallation wurden neue Split-Pfade ermittelt.

Nach Installation/Start der gepatchten Variante zeigte die App einen Fehler:

```text
Error -1 : no protocol: /api/v1/statuslabels?sort=name&order=asc
```

Damit war die App-Konfiguration bzw. der gespeicherte Server-Kontext durch das Patch-/Neuinstallationsverfahren nicht stabil.

**Endgültige Entscheidung:** APK-Patching wird verworfen. Weiterentwicklung erfolgt in der eigenen App direkt gegen die Snipe-IT-API.

---

## 3. NFC-Grundfunktion der eigenen Android-App

### 3.1 NFC-Lesen

**Implementiert und im Betrieb bestätigt.**

Die erste eigene App zeigte zunächst „Hello Android“. Nach Korrektur durch Android Studio wurde NFC erkannt.

Erster Befund mit Tag:

- `android.nfc.tech.NfcA`
- `android.nfc.tech.Ndef`
- UID-Beispiel wurde angezeigt.
- Bei einem Tag ohne passende Nutzdaten erschien „Keine NDEF-Daten gefunden“.

Später wurde die UID ohne Leerzeichen formatiert:

```kotlin
val tagId = tag.id.joinToString("") { byte ->
    "%02X".format(byte)
}
```

Die UID wurde damit als stabile Zeichenfolge, z. B. `5A8519E0064189`, genutzt.

### 3.2 Bekannter NDEF-Tag

**Getestet.**

Ein vorhandener Tag enthielt einen NDEF-URI-Record (TNF 1, Type `U`) mit Payload, die auf eine lokale URL verwies. Dieser Test bestätigte NDEF-Lesen, war später für die Asset-Zuordnung aber nicht mehr erforderlich.

### 3.3 Snipe-IT API-Test

**Im Betrieb bestätigt.**

Ein erster API-Request lieferte:

```json
{"total":0,"rows":[]}
```

Nachdem die NFC-UID als `asset_tag` eines vorhandenen Assets eingetragen wurde, lieferte derselbe Suchansatz `total:1` und Assetdaten wie:

- ID
- Asset Tag
- Seriennummer
- Modell
- Status
- Kategorie
- Hersteller
- Bildpfad
- QR-/Barcodepfade

Danach zeigte die App bereits eine kompakte Assetansicht mit:

- ID
- Name
- Asset Tag
- Seriennummer
- Modell
- Status

Damit war der Kernnachweis erbracht:

> **NFC-UID → Snipe-IT Asset Tag → REST-Suche → Assetdaten** funktioniert real im LAN.

---

## 4. Snipe-IT API und Netzparameter

### 4.1 Server

Verwendet wurden im Verlauf:

- DNS: `CT-H-APP-02`
- direkte App-URL: `http://10.0.1.15`

**Im Betrieb bestätigt:** Die direkte IP mit hardcodierter URL und Token funktionierte.

### 4.2 Authentifizierung

REST-Aufrufe verwenden:

```http
Authorization: Bearer <SNIPE_API_TOKEN>
Accept: application/json
```

Teilweise zusätzlich:

```http
Content-Type: application/json
```

### 4.3 Relevante Endpunkte

Im lokalen Entwicklungsstand implementiert bzw. verwendet:

- Assets suchen:
  `GET /api/v1/hardware?search={tag}`
- Assetliste:
  `GET /api/v1/hardware?limit=100&sort=asset_tag&order=asc`
- Assetdetails:
  `GET /api/v1/hardware/{id}`
- Statuslabels:
  `GET /api/v1/statuslabels?sort=name&order=asc`
- Benutzer:
  `GET /api/v1/users?search={tag}`
- Modelle:
  `GET /api/v1/models?limit=500&sort=name&order=asc`
- Asset anlegen:
  `POST /api/v1/hardware`
- Assetänderung:
  historisch per `POST` plus
  `X-HTTP-Method-Override: PATCH`
  an `/api/v1/hardware/{id}`
- Rücknahme:
  `POST /api/v1/hardware/{id}/checkin`

### 4.4 Token-Auslagerung

**Versucht / nicht erfolgreich stabilisiert / aktuell verworfen.**

Es wurde versucht, URL und Token über `local.properties` → Gradle → `BuildConfig` zu beziehen.

Vorgesehener Zugriff:

```kotlin
private val snipeBaseUrl = BuildConfig.SNIPE_BASE_URL
private val apiToken = BuildConfig.SNIPE_API_TOKEN
```

Fehlerbilder:

1. `no protocol: /api/v1/hardware...`
   - Ursache: `SNIPE_BASE_URL` war leer.
2. Nach URL-Korrektur:
   - HTTP 401
   - mit demselben Token hart in der MainActivity funktionierte die API sofort.

**Festlegung für die Entwicklungsfassung:** Für die laufende Entwicklungsfassung vorerst wieder hardcoded, damit die Funktionsentwicklung weitergehen kann.

**Sicherheitsstatus:** nicht produktionsreif. Ein API-Token in der APK ist auslesbar. Für einen späteren produktiven Stand ist ein Proxy/Backend oder ein besseres Secret-/Credential-Modell erforderlich.

---

## 5. Geräte- und Statusworkflow

### 5.1 Endgültige Statuswerte

Als relevante Statuswerte wurden festgelegt:

- Archiviert
- Ausstehend
- Einsetzbar
- Nicht Einsetzbar

Zusätzlich ist **„ausgegeben / deployed“ kein Status**.

Das war eine wichtige spätere Korrektur.

### 5.2 Ausgabe an Benutzer

**Implementiert und im Betrieb erprobt.**

Endgültiger Flow:

1. Gerätetag scannen oder Asset in Liste antippen.
2. Aktionsdialog öffnen.
3. `Ausgeben` wählen.
4. Popup:
   **„Benutzerkarte scannen“**
5. Benutzertag scannen.
6. Benutzer über `employee_num` ermitteln.
7. Asset dem Benutzer zuweisen.
8. **Assetstatus bleibt unverändert**, z. B. weiterhin `Einsetzbar`.

Frühere Fehlentscheidung:

- „Ausgegeben“ wurde zeitweise wie ein Status bzw. als `Ausstehend` behandelt.
- Dies wurde ausdrücklich korrigiert.

### 5.3 Zurücknehmen

**Implementiert.**

Wenn ein Asset bereits `assigned_to` besitzt:

- Aktionsdialog zeigt **„Zurücknehmen“** statt „Ausgeben“.
- Kein Benutzer-Scan erforderlich.
- Vor Ausführung kommt eine Sicherheitsabfrage:
  „Gerät zurücknehmen?“
- Danach wird der Snipe-IT-Checkin-Endpunkt verwendet.

### 5.4 Nicht Einsetzbar

**Implementiert; wichtige Logikkorrektur.**

Endgültiger Flow:

1. Status `Nicht Einsetzbar` wählen.
2. Benutzerkarte scannen.
3. Pflichtnotiz eingeben, warum das Gerät nicht einsetzbar ist.
4. Status und Notiz speichern.

Wichtig:

- Die Benutzerkarte dient hier als **Bestätigung/Verantwortlicher**.
- Der Benutzer darf **nicht** als `assigned_user` auf dem Asset gesetzt werden.
- Ein nicht einsetzbares Gerät darf im Aktionsdialog **keine Option „Ausgeben“** erhalten.

Früherer Fehler:

- Die erste Implementierung setzte bei „Nicht Einsetzbar“ gleichzeitig `assigned_user`.
- Das wurde als fachlich falsch erkannt und korrigiert.

---

## 6. Benutzerkarten

### 6.1 Erste Lösung: UID in Mitarbeiternummer

**Beschlossen und implementiert für einfache NFC-Tags.**

Die Tag-UID wird als Hexstring ohne Leerzeichen gelesen und in Snipe-IT im Feld `employee_num` des Benutzers hinterlegt.

Suchlogik:

- `GET /api/v1/users?search={tagId}`
- anschließend exakter Vergleich:
  `employee_num.equals(tagId, ignoreCase = true)`

### 6.2 Benutzer-Tag im normalen Scanbetrieb

**Implementiert.**

Wenn bei normalem Scan kein Asset zur UID gefunden wird:

1. App versucht Benutzerauflösung.
2. Wird ein Benutzer gefunden, zeigt die App die Geräte an, die aktuell an diesen Benutzer ausgegeben sind.

Die Filterung erfolgte pragmatisch über `assignedToName == user.name`.

Die Filterung wurde wegen der eindeutigen Usernamen akzeptiert.

### 6.3 MIFARE Plus / Random UID

**Offenes Architekturproblem / Roadmap-Kandidat.**

Für eine Veranstaltung sind NXP MIFARE Plus-Karten geplant, die gleichzeitig für:

- Prepaid-/Getränkezahlung über echte EC-/Payment-Terminals
- Zugangsberechtigungen
- weitere Eventfunktionen

eingesetzt werden sollen.

Problem:

- Die Karte kann eine wechselnde/randomisierte UID präsentieren.
- Die Karten können für AssetOps nicht einfach mit eigenen NDEF-/Zusatzfeldern beschrieben werden.
- Damit ist die reine NFC-UID für eine belastbare Funkgeräteausgabe ungeeignet.

Ziel bleibt:

> Bei ca. 30 Funkgeräten muss eindeutig nachvollziehbar bleiben, wer welches Funkgerät besitzt.

Diskutierte Optionen:

- Event-/Access-Backend als autoritative Kartenauflösung nutzen.
- eigenes Identity-Mapping-Backend vor AssetOps schalten.
- einmalige Koppelung / manuelle Bestätigung als Fallback.

**Endgültiger Stand:** Ein eigenes kleines Mapping-Backend wurde als sinnvollste nächste Architekturidee erkannt; es ist **noch nicht implementiert**.

Mögliche Minimalfunktionen:

- Kartenbeobachtung entgegennehmen
- Karte ↔ User-Zuordnung verwalten
- unsichere/wechselnde Identitäten erkennen
- Asset-Ausgaben protokollieren
- später Snipe-IT-Zugriff serverseitig kapseln

---

## 7. AssetOps UI

### 7.1 Grundlayout

**Implementiert und mehrfach auf realem Gerät gezeigt.**

Die App wurde von einem einfachen TextView-Prototyp zu einer UI mit folgenden Bereichen erweitert:

- Titel `AssetOps`
- Status-/Hinweiszeile
- Buttons:
  - Tag scannen
  - Assets laden
  - Reset
  - Neues Gerät
- Asset-Sektion
- Suchfeld
- Kartenartige Assetdarstellung

Der obere Abstand wurde angepasst, weil die ursprüngliche Ansicht zu weit unter die Statusleiste gerutscht war.

### 7.2 Immersive Mode

In der MainActivity wurde die Android-Navigationsleiste versteckt:

```kotlin
controller.hide(WindowInsets.Type.navigationBars())
```

Für die Splash-Video-Activity führte aggressiveres Fullscreen-/Insets-Handling zu Abstürzen.

**Endgültiger Stand:**

- MainActivity: Navigationsleiste verstecken funktionierte.
- SplashVideoActivity: Fullscreen zunächst wieder entfernt, weil dies die Activity auf dem Testgerät unmittelbar beendete.

### 7.3 Meldungen unten

Gewünscht waren kurze, systemartige Meldungen unten („Doppelt tippen zum Beenden“-Stil), nur für wichtige Ereignisse.

Ein Snackbar-Umbau verursachte Abstürze bei Aktionen wie „Assets laden“.

**Gewählte Lösung:** wieder einfache Android-`Toast`-Meldungen verwenden.

Wichtige Toasts:

- Tag nicht gefunden
- Benutzerkarte nicht gefunden
- Fehler beim Laden
- Fehler beim Speichern
- Rücknahme fehlgeschlagen
- Notiz ist Pflicht
- Gerät angelegt

### 7.4 Assettitel

Spätere Korrektur:

- Haupttitel der Assetkarte soll **Asset Name** sein.
- Fallback:
  1. Name
  2. Seriennummer
  3. Asset Tag

### 7.5 Statusanzeige

Karten zeigen:

- Assetname
- Asset Tag
- Modell
- Statusbadge

Wenn ausgegeben:

- Badge: `Herausgegeben an <Name>`

Wenn `Nicht Einsetzbar`:

- roter Rand um die Assetkarte

### 7.6 Suche

**Implementiert.**

Live-Suche über:

- Assetname
- Asset Tag
- Modell
- Seriennummer

Assets werden automatisch beim Appstart geladen, damit die Suche sofort nutzbar ist.

### 7.7 Pull-to-refresh / Ladezustand

Diskutiert und zeitweise implementiert:

- „Lade Assets…“ während Erstladung
- Pull-to-refresh über `SwipeRefreshLayout`

Später wurde die Assetliste auf `RecyclerView` umgebaut. Als Referenz dient der zuvor als funktionierend bestätigte Stand. Die Regression nach dem Umbau ist noch abzusichern.

**Hinweis:** Der exakte finale lokale Quellstand ist nicht im Repository vorhanden und deshalb in diesem Archiv nicht verifizierbar.

### 7.8 RecyclerView

**Als Editor-Arbeitsstand implementiert; Repository nicht verifiziert.**

Ziel:

- skalierbare Assetliste
- nur sichtbare Views erzeugen
- Basis für echtes Lazy Loading

Abhängigkeit, falls im Projekt noch nicht vorhanden:

```kotlin
implementation("androidx.recyclerview:recyclerview:1.3.2")
```

Keine belastbare Rückmeldung in den vorliegenden Unterlagen bestätigt, dass genau dieser letzte RecyclerView-Stand kompiliert und auf dem Gerät lief.

---

## 8. Assetbilder

### 8.1 Wunsch

Beim Scan bzw. Öffnen eines Assets soll das Geräte-/Modellbild angezeigt werden.

Wenn kein Bild existiert:

- kein Platzhalter
- kein leerer Bildbereich
- nur Textinformationen

### 8.2 Beobachtung

Im Snipe-IT-Webinterface werden in der Assetliste Bilder korrekt dargestellt.

Die Beispiele zeigen u. a. Funkgeräte, Akkus und RSMs.

Der Android-Dialog zeigte dagegen einen großen leeren Bildbereich.

### 8.3 Vermutete Ursache

Snipe-IT kann Bilder auf mehreren Ebenen liefern:

1. Asset-eigenes Bild
2. Modellbild

Der Entwicklungscode nutzte zunächst nur `obj.image`.

Danach wurde als Fallback vorgeschlagen:

- `asset.image`
- `model.image`
- `model.image_url`

Auch ein Loader mit Bearer-Authorization und URL-Normalisierung wurde vorgeschlagen.

### 8.4 Aktueller Stand

**Noch offen / nicht erfolgreich getestet.**

Auch nach diesen Vorschlägen wurde weiter kein Bild angezeigt.

Wichtig:

- JPG und WebP sind grundsätzlich von Android dekodierbar.
- Der konkrete API-Response bzw. tatsächliche Bildpfad im Detail-Endpoint wurde noch nicht abschließend ausgelesen und gegen den erfolgreichen WebUI-Bildrequest verglichen.
- Dieser Punkt bleibt offen und sollte beim Fortsetzen mit einem echten JSON-Dump eines bildbehafteten Assets und HTTP-Status/Content-Type des Bildrequests diagnostiziert werden.

---

## 9. Neues Gerät per NFC anlegen

**Im lokalen Entwicklungsstand implementiert; keine Repository-Übernahme belegt.**

Gewünschter Flow:

1. Button `Neues Gerät`
2. App wartet auf Gerätetag
3. UID wird direkt als `asset_tag` verwendet
4. Formular öffnet sich
5. Felder:
   - Gerätename
   - Seriennummer optional
   - Modell
   - Status
6. Defaultstatus wenn vorhanden: `Einsetzbar`
7. `POST /api/v1/hardware`
8. Liste neu laden

Modell- und Statusdaten werden vor dem Formular aus Snipe-IT geladen.

---

## 10. Benutzer-Scan-Popup

**Implementiert / später verfeinert.**

Gewünscht:

- Während die App auf eine Benutzerkarte wartet, soll ein echtes Popup sichtbar sein:
  „Benutzerkarte scannen …“
- Sobald ein **gültiger** Tag gefunden wurde, soll das Popup automatisch verschwinden.

Wichtige Korrektur:

- Der Dialog darf nicht bereits beim bloßen NFC-Ereignis geschlossen werden.
- Er soll erst geschlossen werden, wenn die Benutzerauflösung tatsächlich erfolgreich war.

Der Dialog wurde schließlich in `searchUserByTag` nach `foundUser != null` geschlossen.

---

## 11. App-Branding und Startvideo

### 11.1 Logo

Ein NetCore-Tetra-Logo mit:

- Mesh-/Netzknoten-Symbol
- blauen Funkwellen
- Schriftzug „NetCore-Tetra“
- Slogan „digital. dezentral. skalierbar.“

wurde als Appbranding verwendet.

Entscheidung:

- Launcher-Icon idealerweise nur Symbol ohne Text.
- vollständiges Logo für Splash/Intro.

### 11.2 Android Ressourcen

Vorhandene Standardstruktur wurde gezeigt:

- `res/drawable/`
- `res/mipmap-anydpi/`
- `res/mipmap-hdpi/`
- `res/mipmap-mdpi/`
- `res/mipmap-xhdpi/`
- `res/mipmap-xxhdpi/`
- `res/mipmap-xxxhdpi/`
- `res/values/`
- `res/xml/`

Adaptive Icon Ressourcen:

- `ic_launcher_background.xml`
- `ic_launcher_foreground.xml`
- `ic_launcher.xml`
- `ic_launcher_round.xml`

Empfohlen wurde Android Studios **Image Asset**-Wizard statt manueller Größenpflege.

### 11.3 MP4-Intro

**Implementiert und auf realem Gerät sichtbar.**

Das gewünschte „Ladebildschirm“-Video wurde nicht als Android-System-Splash, sondern als separate `SplashVideoActivity` umgesetzt.

Eigenschaften:

- `res/raw/intro.mp4`
- `VideoView`
- Audio stumm:
  `mediaPlayer.setVolume(0f, 0f)`
- schwarzer Hintergrund
- automatische Weiterleitung zu `MainActivity` nach Videoende
- Fehlerfallback öffnet MainActivity

Das Video wurde später mittig gesetzt über:

```kotlin
FrameLayout.LayoutParams(
    WRAP_CONTENT,
    WRAP_CONTENT
).apply {
    gravity = Gravity.CENTER
}
```

Ein aggressiver Fullscreen-Versuch zum Verstecken der unteren Android-Navigationsleiste verursachte einen sofortigen App-Abbruch. Nach Rückkehr zur einfachen Variante lief das Video wieder.

**Endgültige Entscheidung:** Splashvideo vorerst stabil lassen; Fullscreen dort nicht weiter forcieren.

---

## 12. Fehler- und Diagnosechronik

| Fehler | Diagnose | Lösung / Stand |
|---|---|---|
| „Keine NDEF-Daten gefunden“ | Tag hatte keine erwarteten NDEF-Textdaten; UID war aber lesbar | UID direkt verwenden |
| Snipe-Suche `total:0` | UID noch nicht als Asset Tag hinterlegt | UID als Asset Tag gesetzt; danach Treffer |
| Snipe-App „nicht gefunden“ | zunächst falscher/angenommener Paketname bzw. Intent | echten Paketnamen über App-Info/ADB ermittelt |
| Nur Snipe-Hauptmenü öffnet | interne Ziel-Activity nicht exportiert | APK analysiert; Deep-Link-Ansatz verworfen |
| SecurityException bei EditActivity | `android:exported=false` | bestätigte Manifest-Grenze |
| apktool/keytool/adb nicht gefunden | PATH/PowerShell-Syntax | absolute Android-SDK-/Toolpfade verwendet |
| gepatchte APK ungültig / Missing split | Play-Store-App war Split APK | Splits identifiziert; Patchpfad letztlich verworfen |
| gepatchte Snipe-App: `no protocol: /api/v1/statuslabels` | gespeicherte Server-Konfiguration fehlte / leer | Patchansatz beendet |
| Eigen-App: `no protocol: /api/v1/hardware` | BuildConfig-BaseURL leer | direkte URL bzw. korrekte BuildConfig nötig |
| Eigen-App: HTTP 401 | BuildConfig-Token ungültig/leer/falsch übernommen | Hardcode funktionierte; Secret-Auslagerung zunächst verworfen |
| Snackbar-Umbau führt zu Absturz | konkrete Ursache im Verlauf nicht per Logcat belegt | auf Toast zurückgegangen |
| Splash-Fullscreen führt zu sofortigem Abbruch | Insets/Fullscreen-Code gerätespezifisch problematisch | stabile VideoActivity ohne aggressiven Fullscreen |
| Assetbild bleibt leer | Bildquelle/API-/Auth-/Pfad noch nicht abschließend bestimmt | offen; Detail-JSON + HTTP-Bildrequest debuggen |
| „Nicht Einsetzbar“ markiert als herausgegeben | Statusworkflow übergab fälschlich `assigned_user` | Userkarte nur Bestätigung; kein Asset-Assignment |
| ausgegebenes Asset wurde `Ausstehend` | Ausgabe fälschlich als Status interpretiert | Ausgabe ist Assignment, Status bleibt z. B. Einsetzbar |

---

## 13. Tests und bestätigte Ergebnisse

### Im Betrieb bestätigt

- NFC-Tag wird auf Android erkannt.
- UID kann ohne Leerzeichen als Hexstring gelesen werden.
- Snipe-IT ist aus der App im LAN erreichbar.
- Asset-Suche nach als Asset Tag hinterlegter UID liefert real Daten.
- Assetdaten wurden in der App angezeigt.
- vorhandene Snipe-IT-App kann per Package/MainActivity geöffnet werden.
- MainActivity der Snipe-App landet im Hauptmenü.
- interne nicht-exportierte Activities lassen sich extern nicht starten.
- gepatchte Snipe-APK war signierbar, Split-APK-Verhalten wurde bestätigt.
- eigene AssetOps-Oberfläche lief auf dem Gerät und zeigte Assetkarten/Aktionsdialoge.
- MP4-Intro lief visuell und stumm.
- direkte API-Konfiguration mit hardcodierter IP und Token funktionierte.
- BuildConfig-Variante produzierte erst fehlende URL, danach 401.

### Historisch getestet, aber nicht vollständig regressionsgesichert

- Statuswechsel
- Ausgabe an Benutzer
- Rücknahme
- User-Suche via employee_num
- neues Asset anlegen
- Auto-Load / Suche
- Benutzerkarte → Liste der ausgegebenen Geräte

### Nicht abschließend bestätigt

- letzter RecyclerView-Umbau kompiliert/läuft auf realem Gerät
- Pull-to-refresh im letzten konsolidierten Stand
- Modellbild-Fallback in Android
- produktionssichere Secret-Verwaltung
- MIFARE-Plus-Identitätsauflösung
- Backend-Mapping
- vollständige Fehler-/Auditprotokollierung

---

## 14. Relevante Dateien und lokale Pfade

### Android-Projekt

Im Verlauf genannter lokaler Projektpfad:

```text
C:\Users\janho\AndroidStudioProjects\NFCBrigde
```

Package:

```text
de.jan.nfcbrigde
```

Wesentliche Dateien:

- `MainActivity.kt`
- `SplashVideoActivity.kt`
- `AndroidManifest.xml`
- `app/build.gradle.kts`
- `local.properties` (Secret-Versuch)
- `res/raw/intro.mp4`
- Launcher-Icon-Ressourcen unter `res/mipmap-*`
- `res/drawable/ic_launcher_background.xml`
- `res/drawable/ic_launcher_foreground.xml`
- `res/values/themes.xml`

### APK-Analyse

```text
C:\temp\snipepatch
```

ADB-Tools:

```text
%LOCALAPPDATA%\Android\Sdk\platform-tools
```

Build Tools Beispiel:

```text
%LOCALAPPDATA%\Android\Sdk\build-tools\36.1.0
```

---

## 15. Relevante Befehle

### ADB Package-Analyse — erfolgreich ausgeführt

```powershell
.\adb shell cmd package resolve-activity --brief com.diegogarciadev.assetsmanager.snipeit
```

```powershell
.\adb shell dumpsys package com.diegogarciadev.assetsmanager.snipeit | findstr /i activity
```

```powershell
.\adb shell pm path com.diegogarciadev.assetsmanager.snipeit
```

### Direkter Activity-Test — bewusst fehlgeschlagen / Diagnose erfolgreich

```powershell
.\adb shell am start -n com.diegogarciadev.assetsmanager.snipeit/com.diegogarciadev.assetsmanager.EditActivity --es ASSET_ID 2
```

Ergebnis: SecurityException wegen nicht exportierter Activity.

### APK-Signaturprüfung — erfolgreich ausgeführt

```powershell
& "$env:LOCALAPPDATA\Android\Sdk\build-tools\36.1.0\apksigner.bat" verify --verbose .\snipeit_patched_aligned.apk
```

### APK-Installation — Fehler diagnostisch relevant

```powershell
& "$env:LOCALAPPDATA\Android\Sdk\platform-tools\adb.exe" install -r -d .\snipeit_patched_aligned.apk
```

Ergebnis:

```text
INSTALL_FAILED_MISSING_SPLIT
```

---

## 16. Architektur des aktuellen Ansatzes

```text
[NFC Tag / User Card]
        |
        v
[Android AssetOps]
  - NfcAdapter
  - Geräte-/User-Resolver
  - UI / Workflow
        |
        | HTTP REST + Bearer
        v
[Snipe-IT @ 10.0.1.15]
  - hardware
  - users
  - models
  - statuslabels
        |
        v
[Asset / Assignment / Status]
```

Geplanter späterer Ausbau für MIFARE Plus:

```text
[MIFARE Plus / Event Card]
        |
        v
[Android AssetOps]
        |
        v
[Identity Mapping Backend]
  - card observations
  - event/session mapping
  - user resolution
  - audit
        |
        +------> [Snipe-IT API]
        |
        +------> optional Event/Access Backend
```

Dieser Backend-Layer könnte gleichzeitig das derzeit hardcodierte Snipe-IT-API-Secret aus der Android-App entfernen.

---

## 17. Verworfene / ersetzte Ansätze

1. **Nur bestehende Snipe-IT-App benutzen**
   - scheiterte am fehlenden externen Deep Link zu internen Activities.

2. **Nur AndroidManifest.xml der Snipe-App ändern**
   - technisch möglich, aber Split-APK, Signierung, Updates und Appkonfiguration machen dies unnötig fragil.

3. **Gepatchte Snipe-App weiterbetreiben**
   - verworfen nach Split-/Konfigurationsproblemen und `no protocol`-Fehler.

4. **„Ausgegeben“ als Status**
   - fachlich falsch; ersetzt durch Snipe-IT-Zuweisung bei unverändertem Status.

5. **Nicht Einsetzbar + assigned_user**
   - fachlich falsch; Userkarte dient nur Bestätigung.

6. **Snackbar für alle wichtigen Meldungen**
   - führte im Teststand zu Abstürzen; Toast als robuste Lösung.

7. **Aggressiver Fullscreen im Splashvideo**
   - verursachte sofortigen App-Abbruch; vorerst verworfen.

8. **UID als dauerhafte Identität für MIFARE Plus**
   - wegen Random UID für verantwortliche Geräteausgabe ungeeignet.

---

## 18. Roadmap-Kandidaten und offene Aufgaben

### Priorität A — Funktionssicherheit

- **[beschlossen/geplant]** Aktuellen funktionierenden Android-Source-Stand als vollständiges Projekt sichern und versionieren.
- **[offen]** Alle Status-/Ausgabe-/Rücknahme-Flows mit mehreren realen Assets regressionsprüfen.
- **[offen]** Audit-/Historienlog für:
  - Asset
  - User
  - Aktion
  - Zeit
  - Status
  - Notiz
  ergänzen.
- **[offen]** Fehlerantworten der Snipe-IT API zentral und konsistent behandeln.
- **[offen]** Bild-Problem mit realem Detail-JSON und Bild-HTTP-Response diagnostizieren.

### Priorität B — Security

- **[beschlossen/geplant]** offengelegten API-Token rotieren.
- **[offen]** Snipe-IT-Token aus der APK entfernen.
- **[Idee / empfohlen]** kleines Backend/Proxy vor Snipe-IT.
- **[offen]** TLS statt reinem HTTP im LAN prüfen.
- **[offen]** rollenbasierte Berechtigungen für Ausgabe/Rücknahme/Statusänderung.

### Priorität B — MIFARE Plus / Eventkarten

- **[offen]** Karten-/Terminalsystem und verfügbare Backend-API des Eventsystems klären.
- **[Idee]** Identity-Mapping-Backend:
  - User
  - Card identity / observation
  - Event / Session
  - confidence / explicit confirmation
  - device assignments
- **[offen]** festlegen, welche stabile Kennung aus dem Eventsystem genutzt werden kann.
- **[offen]** Fallback bei nicht eindeutig auflösbarer Karte definieren.
- **[offen]** nachvollziehbaren Ausgabeprozess für ca. 30 Funkgeräte testen.

### Priorität C — UI

- **[offen]** RecyclerView-Stand auf realem Gerät testen.
- **[offen]** Pull-to-refresh nach RecyclerView-Umbau sauber integrieren.
- **[offen]** Assetbild nur bei erfolgreichem Laden anzeigen.
- **[Idee]** Bild-Caching über Coil/Glide.
- **[Idee]** dunkles Theme passend zum schwarzen NetCore-Splash, um den „Flashbang“-Übergang zu vermeiden.
- **[Idee]** Trefferanzahl bei Suche.

### Priorität C — Splash/Branding

- **[implementiert/getestet]** stummes MP4-Intro.
- **[offen]** optional Navigationbar im Splash auf gerätekompatible Weise ausblenden.
- **[offen]** Launcher-Icon final nur mit NetCore-Symbol erzeugen.

---

## 19. Repository-Abgleich am 2026-10-05

Der Branch `Archiving` wurde vor dem Schreiben bei Commit

`83cdd56e1450be16437f1b94bb4f2db035149460`

geprüft.

Code-Suchen nach:

- `Snipe-IT`
- `NFCBrigde`
- `AssetOps`

lieferten **keine Treffer**.

Daraus folgt:

- Der hier beschriebene Android-Code ist **lokaler Entwicklungsstand**, nicht verifizierter Repository-Stand.
- Dieses Archiv dokumentiert den Entwicklungsverlauf und die Entscheidungen.
- Es bestätigt **nicht**, dass die Android-App bereits in `netcore-tetra` eingecheckt, gebaut oder aus diesem Repository deployed wird.

---

## 20. Referenzbilder / Anhänge

17 Bilddateien waren als Referenzen verfügbar, darunter:

1. erster NFC-Scan ohne NDEF-Nutzdaten
2. frühe Assetliste
3. MP4-/Splashansicht
4. Snipe-IT-Web-Assetliste mit Gerätebildern
5. NDEF-URI-Dump
6. erstes „Treffer gefunden“-Ergebnis
7. Android-Resource-Ordneransicht
8. frühes Scan-Popup ohne Aktionsbuttons
9. AssetOps-Fehleranzeige `no protocol`
10. NetCore-Tetra Logo mit Text und Slogan
11. Fehler beim Starten der nicht exportierten Snipe-Activity
12. AssetOps-HTTP-/URL-Debugansicht
13. gepatchte Snipe-App mit `no protocol`-Fehler
14. Snipe-App-Info mit Paketname
15. AssetOps-Scan-Popup mit Status/Ausgeben
16. Android-Studio-Ressourcenbaum
17. rohe Snipe-IT-API-Antwort

**Bildnachweis:** Die Originalbilder wurden ausgewertet, liegen aber nicht als verlässlich übertragene Repository-Binärdateien vor. Die fehlenden Originalkopien bleiben eine Dokumentationslücke.

---

## 21. Offene Nachweise

- Der vollständige lokale Android-Projektordner liegt nicht als Datei/ZIP vor. Verfügbar sind Codeausschnitte und Editor-Zwischenstände; der finale Source-Stand ist noch zu sichern.
- Die genaue letzte **kompilierende** Version von `MainActivity.kt` ist daher nicht mit Hash/Datei belegbar.
- Mehrere Canvas-Umbauten waren Zwischenstände und wurden später wieder korrigiert; sie sind nicht automatisch als „funktionierend“ zu behandeln.
- Die Originalbilder sind im Runtime-Dateisystem vorhanden, konnten mit dem verfügbaren GitHub-Schreibconnector aber nicht binär hochgeladen werden.
- Die ETSI-Referenzsammlung ist keine fachliche Grundlage der Snipe-IT/NFC-App.

---

## 22. Nächste Schritte

1. **Letzten tatsächlich laufenden Android-Source-Stand hochladen oder in ein eigenes Repo/Verzeichnis einchecken.**
2. API-Token rotieren.
3. Assetbildproblem mit einem einzigen bildbehafteten Asset diagnostizieren:
   - `GET /api/v1/hardware/{id}`
   - vollständige `image`- und `model`-Felder prüfen
   - tatsächlichen Bildrequest mit HTTP-Code/Content-Type protokollieren.
4. RecyclerView nur auf Basis des bestätigten laufenden Stands integrieren und bauen.
5. MIFARE-Plus-Backend als eigenes, kleines Identity-Modul spezifizieren, bevor die Eventkarten produktiv für Funkgeräteausgabe verwendet werden.
6. Erst danach weitere UI-/Caching-/Offline-Funktionen ergänzen.

---

## 23. Zusammenfassung

Die Entwicklung führte zu einem klaren Architekturwechsel:

**von:** „NFC-Bridge soll die bestehende Snipe-IT-App direkt auf das Asset öffnen“

**zu:** „eigene schlanke AssetOps-Android-App nutzt NFC direkt und spricht die Snipe-IT-REST-API“.

Der Kernflow **NFC → Asset finden → Status/Ausgabe/Rücknahme** ist real nachgewiesen. Die wichtigeren fachlichen Regeln wurden im Verlauf korrigiert:

- Ausgabe ist **kein Status**.
- Ein ausgegebenes Gerät kann weiterhin `Einsetzbar` sein.
- `Nicht Einsetzbar` darf nicht gleichzeitig an einen Benutzer ausgegeben werden.
- Userkarten dienen je nach Flow zur Zuweisung oder nur zur Bestätigung.
- MIFARE-Plus-Random-UID erfordert für belastbare Ausgabe ein zusätzliches Identitäts-/Backend-Konzept.

Damit ist die App funktional bereits weit über einen NFC-Prototyp hinaus, aber für produktiven Eventbetrieb fehlen vor allem **Secret-Handling, Auditierung, MIFARE-Identitätsauflösung, finale Bilddiagnose und ein gesicherter/versionierter Source-Stand**.
