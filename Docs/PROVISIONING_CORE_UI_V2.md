# Provisioning Core WebUI – Layout-Überarbeitung

Diese Revision überarbeitet ausschließlich Darstellung und Bedienbarkeit der bestehenden Provisioning-Core-WebUI. API-Verträge, Datenmodell und Persistenz bleiben unverändert.

## Behobene Probleme

- Tabellenköpfe liegen nicht mehr mit festem 85-Pixel-Abstand über den Datenzeilen.
- Geräte- und Gruppentabellen besitzen eigene Scrollbereiche mit korrekt am oberen Rand fixierten Kopfzeilen.
- Aktionsschaltflächen bleiben in einer gemeinsamen Aktionsspalte und überdecken keine Nachbarzeilen.
- Die Mitgliedschaftsmatrix verwendet kompakte, feste Gruppenspalten statt gleichmäßig über die gesamte Fensterbreite gestreckter Zellen.
- Die Gerätespalte bleibt beim horizontalen Scrollen sichtbar.
- Checkbox, Detailaktion sowie Kennzeichen für Auto-Attach und fixierte Mitgliedschaften werden innerhalb einer klar abgegrenzten Matrixzelle dargestellt.
- Geräte und Gruppen können in der Matrix getrennt gefiltert werden.
- Leere Suchergebnisse erhalten eine verständliche Statuszeile.
- Desktop-, Tablet- und Mobilansicht verwenden passende Höhen, Spaltenbreiten und Werkzeugleisten.

## Aktualisierung eines vorhandenen Provisioning-LXC

Nach Übernahme des vollständigen Repository-Standes in den vorhandenen Git-Clone:

```bash
cd /opt/netcore-tetra
sudo bash system-backend/provisioning-core/install/update.sh
```

Danach den Browser-Cache mit `Strg+F5` umgehen. Da das HTML in der Binary eingebettet ist, reicht ein Browser-Neuladen ohne aktualisierte Binary nicht aus.
