# Häufige Fragen

### Muss ich bei Dual Carrier beide Frequenzen als Kontrollkanal ins Funkgerät eintragen?

Nicht automatisch. Der Hauptträger führt im beschriebenen Aufbau den primären Kontrollpfad. Ob und wie der zweite Träger vom Endgerät benutzt wird, hängt von Zellinformation, Slot-Plan und Geräteprogrammierung ab. Die belegte Kontrollfrequenz, Netzparameter und das beobachtete Gerätelog prüfen; [[Dual-Carrier]].

### Wieso sind Node Gateway und TBS-Dashboard beide auf Port 8080?

Es sind zwei Listener **auf unterschiedlichen Hosts** in der Beispieltopologie. Host plus Port ist der Endpunkt; [[Netzwerk-und-Ports]].

### Warum sehe ich eine ISSI im Directory, aber keinen Ruf?

Der Name ist Metadatum. Teilnehmerpolicy, Registrierung, aktuelle Serving-TBS, Affiliation und erreichbarer Traffic-Pfad sind eigene Voraussetzungen; [[Provisioning]] · [[Calls]].

### Fünf MQTT-Clients sind verbunden, aber HA empfängt keine SDS. Was fehlt?

Eine Verbindung bestätigt keine Ereignisweitergabe. Dieselbe Test-SDS vom TBS-Eingang über SDS Router, IoT Gateway, Broker-Topic und HA-Trigger verfolgen. Für die System-ISSI `4010001` kann die TBS die Nachricht selbst behandeln; [[MQTT-und-Home-Assistant]].

### Das Dashboard meldet Healthy; ist der Backend-Dienst bereit?

Liveness, Readiness und Fachfunktion sind verschiedene Prüfungen. Ein laufender Dienst kann bei fehlender Abhängigkeit `ready=503` liefern; [[Bedienoberflaechen]] · [[Troubleshooting]].

### Welche Version soll ich verwenden?

Für den tatsächlich laufenden Host zuerst Commit/Tag, lokale Änderungen, `Cargo.toml`, Buildlog und Dienstkonfiguration erfassen. Die Workspace-Version `1.3.0` ist kein Beweis für einen gleichnamigen Release. Dieses Wiki bezieht sich auf `main` `d518c97`; [[Projektstand]].

### Ist der Imagebuilder schon fertig?

Die aktuelle Open-Lab-Deploymentvorlage rollt Quellcode und Konfiguration auf LXCs aus. Das pro TBS fertige Pi-Image mit Discovery und Wizard steht als Vorhaben unter [[Roadmap]].
