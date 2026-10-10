# Häufige Fragen

### Muss ich bei Dual Carrier beide Frequenzen als Kontrollkanal ins Funkgerät eintragen?

Nicht automatisch. Der Hauptträger führt im beschriebenen Aufbau den primären Kontrollpfad. Ob und wie der zweite Träger vom Endgerät benutzt wird, hängt von Zellinformation, Slot-Plan und Geräteprogrammierung ab. Die belegte Kontrollfrequenz, Netzparameter und das beobachtete Gerätelog prüfen; [Zwei Funkträger mit Dual Carrier betreiben](zwei-funktraeger.md).

### Wieso sind Node Gateway und TBS-Dashboard beide auf Port 8080?

Es sind zwei Listener **auf unterschiedlichen Hosts** in der Beispieltopologie. Host plus Port ist der Endpunkt; [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md).

### Warum sehe ich eine ISSI im Directory, aber keinen Ruf?

Der Name ist Metadatum. Teilnehmerpolicy, Registrierung, aktuelle Serving-TBS, Affiliation und erreichbarer Traffic-Pfad sind eigene Voraussetzungen; [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) · [Gruppen- und Einzelrufe](gruppen-und-einzelrufe.md).

### Fünf MQTT-Clients sind verbunden, aber HA empfängt keine SDS. Was fehlt?

Eine Verbindung bestätigt keine Ereignisweitergabe. Dieselbe Test-SDS vom TBS-Eingang über SDS Router, IoT Gateway, Broker-Topic und HA-Trigger verfolgen. Für die System-ISSI `4010001` kann die TBS die Nachricht selbst behandeln; [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md).

### Das Dashboard meldet Healthy; ist der Backend-Dienst bereit?

Liveness, Readiness und Fachfunktion sind verschiedene Prüfungen. Ein laufender Dienst kann bei fehlender Abhängigkeit `ready=503` liefern; [Bedienoberflächen und Rollen](bedienoberflaechen-und-zustaendigkeiten.md) · [Fehlersuche](fehlersuche.md).

### Welche Version soll ich verwenden?

Für den tatsächlich laufenden Host zuerst Commit/Tag, lokale Änderungen, `Cargo.toml`, Buildlog und Dienstkonfiguration erfassen. Die Workspace-Version `1.3.0` ist kein Beweis für einen gleichnamigen Release. Diese Überarbeitung bezieht sich auf `main@c3ccdb4` vom 09.10.2026; [Projektstand und Nachweisgrenzen](projektstand-und-nachweise.md).

### Ist der Imagebuilder schon fertig?

Der Imagebuilder, das TBS-Profilformular, Discovery und die optionale VPN-Heimnetzregel sind implementiert. Der vollständige Build wurde in einem Betreiberbefund bestätigt; Download, Wiederherstellung und physischer Pi-/SXceiver-/VPN-Test sind eigene Abnahmen. Für den aktuellen Stand siehe [Imagebuilder-Nachweis](../integration/Z01-2026-10-07/imagebuilder-vm119.md) und [Gesamtroadmap](../roadmaps/gesamtroadmap.md).

## Quellen zur Pflege dieser Seite

[Dienstregistry](../../system-backend/services.toml) · [Image-API](../../system-backend/deployment-core/main.py).
