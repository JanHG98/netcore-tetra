# Bildarchiv: Home Assistant, MQTT und SDS

Bildbelege vom 18.09.2026, gesichert am 05.10.2026. Zugehörige [Projektnotizen](../../2026-10-05_home-assistant-proxmox-mqtt-sds-diagnose.md).

Die ersten vier Dateien sind vollständige Ansichten des geöffneten Bildviewers. Der Originaldownload war über die verfügbaren Browser-Exportfunktionen nicht verlässlich möglich; die Ansichtsreproduktionen erhalten den sichtbaren Bildinhalt und den Viewer-Rahmen. Es wurden keine Bilder nachgezeichnet, generiert oder inhaltlich verändert. Die letzten drei Dateien wurden bytegetreu aus den bereitgestellten PNG-Originalen kopiert. Ihr Dateiname war dort jeweils image.png. Die Auflösung der schmalen Gateway-Datei ist bereits die der bereitgestellten Datei.

Die Herkunftsreihenfolge wurde für den technischen Ablauf sortiert. Eine Prüfsumme belegt die Integrität der hier gespeicherten Datei, bei Ansichtsreproduktionen ausdrücklich nicht die des nicht exportierten Originals. Die Bilder wurden auf sichtbare Zugangsdaten geprüft; sie zeigen technische Konfiguration, Testmeldungen und Zustände. Der unveränderte SDS-Bildbeleg enthält auch die historische LIP-Positionszeile.

| Nr. | Datei / Inhalt | Herkunft und Grenze | Gespeicherte Pixel | Bytes | SHA-256 |
|---|---|---|---|---|---|
| 1 | [01-vm-bootfehler-ansicht.jpg](01-vm-bootfehler-ansicht.jpg) – Bootfehler, iPXE und nicht bootfähige Disk | Ansichtsreproduktion; Bildvorschau 73 %, Original im DOM 1286 x 714 | 1280 x 720 | 76479 | `89ea9e47a78126f5af07522d8f46796fb13ef1a2bbfb2a4116745695d671a501` |
| 2 | [02-proxmox-vm148-hardware-ansicht.jpg](02-proxmox-vm148-hardware-ansicht.jpg) – VM 148: OVMF, 8 GiB, 4 Kerne, scsi0 und IMG als CD-Medium | Ansichtsreproduktion; Bildvorschau 100 %, Original im DOM 580 x 282 | 1280 x 720 | 52692 | `cdd57968a0fe865946d1cfaebecd919a508175dd948465dd2679775d11f938f9` |
| 3 | [03-homeassistant-laeuft-ansicht.jpg](03-homeassistant-laeuft-ansicht.jpg) – HA-Übersicht nach Grundeinrichtung | Ansichtsreproduktion; Bildvorschau 52 %, Original im DOM 1920 x 995 | 1280 x 720 | 35435 | `84f0923c39d4b64743a5e09348516805e0f7da181ebc2ed4428f715a8586e828` |
| 4 | [04-mqtt-vier-geraete-acht-entitaeten-ansicht.jpg](04-mqtt-vier-geraete-acht-entitaeten-ansicht.jpg) – MQTT: vier NetCore-Geräte und acht Entitäten | Ansichtsreproduktion; Bildvorschau 79 %, Original im DOM 1064 x 652 | 1280 x 720 | 46858 | `7f763bdeeb89c0fc53710c26ee383fc4f10708880d84cd7f427c4e5cfd11854c` |
| 5 | [05-ha-sds-topic-leer.png](05-ha-sds-topic-leer.png) – Aktives HA-Abo auf netcore/v1/events/sds/# ohne sichtbare Nachricht | Unveränderte PNG-Datei aus dem Quellmaterial | 598 x 252 | 10708 | `31d0630777375b9beda21dfb7b10647253d61d09af8177d1cdc6abb2d56271e0` |
| 6 | [06-tbs-sds-empfang.png](06-tbs-sds-empfang.png) – TBS-Eingang: HA TEST von 5102 an 15201 und 5235960 sowie LIP an 4010001 | Unveränderte PNG-Datei aus dem Quellmaterial | 1452 x 162 | 21222 | `e8eca7eced8cd077aa7e0fd27ca704402c3a68a27fe6191d59da589f4c98fe8f` |
| 7 | [07-iot-gateway-outbox.png](07-iot-gateway-outbox.png) – Gateway Phase 5: ONLINE, 4/4 Quellen und Outbox 2858 | Unveränderte PNG-Datei aus dem Quellmaterial | 506 x 2048 | 517250 | `b9d1098e60d1225d1c5245762f40e1294c0c93e59b2e044b2f290f4e2ffe97df` |

Nicht verfügbare weitere Anlage: **Eingefügter Text.txt**. Die Quelldatei ließ sich nicht wiederherstellen; eine Rohlogdatei samt Original-Prüfsumme fehlt daher. Erhaltene Logauszüge stehen mit diesem Vorbehalt in den Projektnotizen.
