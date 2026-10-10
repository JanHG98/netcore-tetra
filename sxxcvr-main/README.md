# SXceiver und SoapySX auf dem Raspberry Pi

Dieser Ordner enthält den SDR-Treiber und die zugehörigen Hardwarewerkzeuge. Für die gesamte NetCore-TBS-Installation die [Basisstationsanleitung](../Docs/wiki/basisstation-installieren.md) und für Boardrevision, Verkabelung und HF den [Hardwareleitfaden](../Docs/wiki/hardware-sdr-und-hf-aufbau.md) verwenden. Die folgenden Befehle betreffen nur dieses Treibermodul; ab diesem Verzeichnis ausführen.

## Abhängigkeiten und Treiber

Abhängigkeiten aus den eingerichteten Paketquellen installieren:
```
sudo apt-get install --no-install-recommends git make g++ cmake libsoapysdr-dev libasound2-dev soapysdr-tools python3-soapysdr
```

Bei manchen Prototypplatinen ist das HAT-Identifikations-EEPROM noch unbeschrieben. Die [EEPROM-Anleitung](dts/README.md) beschreibt die dafür vorgesehenen Schritte; die tatsächliche Boardrevision bestimmt das Image.

SoapySDR-Modul bauen und installieren:
```
cd SoapySX
mkdir build
cd build
cmake ..
make
sudo make install
sudo ldconfig
```

Prüfen, ob SoapySDR das Modul findet:
```
SoapySDRUtil --probe=driver=sx
```

## Zeitstempel und Beispiele

SoapySX unterstützt Zeitstempel, mit denen Anwendungen Sende- und Empfangssignale zeitlich zuordnen können. Das [Repeaterbeispiel](example/linear_repeater.py) zeigt einen solchen Ablauf. Ein erfolgreiches Probe-Kommando bestätigt die Treibererkennung; es ersetzt keine TBS-/Funkabnahme.
