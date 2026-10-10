# Versteckte Integrationen

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Versteckte Integrationen. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Sichtbarkeit und die Integrationsrouten ergeben sich aus [Dashboard-Quellen](../../../crates/tetra-entities/src/net_dashboard) und [Integrationen](../../wiki/integrationen-und-datenwege.md) . Ein versteckter Menüpunkt ist kein Sicherheitsmechanismus und keine Freigabe eines externen Dienstes.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Versteckte Integrationen

Die folgenden Integrationen sind im normalen Dashboard ausgeblendet und nur über einen direkten Aufruf erreichbar.

## Geheimmodus aktivieren

Ersetze `<BASISSTATION-IP>` und `<PORT>` durch die Adresse deiner Basisstation.

### DAPNET

```text
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=dapnet
```

### EchoLink

```text
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=echolink
```

### MeshCom

```text
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=meshcom
```

### GeoAlarm

```text
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=geoalarm
```

Nach dem Aufruf wird der Geheimmodus für den aktuellen Browser-Tab aktiviert. Anschließend sind alle versteckten Integrationen im Dashboard sichtbar.

Der Parameter wird danach automatisch aus der Adresszeile entfernt.

## Geheimmodus deaktivieren

```text
http://<BASISSTATION-IP>:<PORT>/?intern=off
```

Alternativ kann der betreffende Browser-Tab geschlossen werden.

## Sicherheitshinweis

Der Geheimmodus ist lediglich eine versteckte Darstellung innerhalb des Dashboards und keine echte Zugriffskontrolle.

Die zugehörigen Backend-Funktionen und API-Endpunkte bleiben erreichbar, sofern sie nicht zusätzlich durch Firewall, Reverse Proxy oder Authentifizierung geschützt werden.
