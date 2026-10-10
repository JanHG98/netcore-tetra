# HAT-Identifikations-EEPROM vorbereiten und schreiben

Diese Schritte betreffen die Hardwarekennung des SXceiver-HAT, nicht die NetCore-Dienstkonfiguration. Wenn das EEPROM bereits korrekt beschrieben ist, ist kein neuer Schreibvorgang nötig. Die tatsächliche Boardrevision muss zum gewählten Image passen. [Treiberinstallation](../README.md) · [NetCore-Hardwareleitfaden](../../Docs/wiki/hardware-sdr-und-hf-aufbau.md).

## EEPROM-Werkzeuge bauen

```
sudo apt-get install --no-install-recommends git make gcc device-tree-compiler alsa-utils
cd
git clone "https://github.com/raspberrypi/hats.git"
cd hats/eepromutils
make
sudo make install
```

## Image für die Boardrevision erstellen

Vom SXceiver-Quellverzeichnis aus in diesen Ordner wechseln:
```
cd dts
make clean
```

Den zur tatsächlichen Hardwareversion passenden Befehl wählen:
```
# Für Hardwareversion 1.0:
make HAT_VERSION=0x0100
# Für Hardwareversion 1.1:
make HAT_VERSION=0x0101
# Für Hardwareversion 1.2:
make HAT_VERSION=0x0102
```

Den EEPROM-Schreibschutz mit der vorgesehenen WP-Brücke auf der Platine deaktivieren. Anschließend das erzeugte Image schreiben:
```
sudo make write_eeprom
```

Raspberry Pi neu starten:
```
sudo reboot
```

Nach dem Neustart prüfen, ob Hardwarekennung und Audiogerät erkannt werden:
```
ls -l /proc/device-tree/hat
aplay -L|grep SX1255
arecord -L|grep SX1255
```
