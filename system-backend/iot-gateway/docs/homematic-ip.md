# Homematic IP Adapter

## Variante A – Homematic IP Access Point

Der Access Point bleibt in Home Assistant eingebunden. NetCore übernimmt ausgewählte Zustände über den Home-Assistant-State-Ingress. Dies ist der Standardmodus:

```toml
[homematic]
enabled = false
mode = "home_assistant_mqtt"
allow_writes = false
```

## Variante B – CCU3 oder RaspberryMatic

```toml
[homematic]
enabled = true
mode = "ccu_xml_rpc"
ccu_host = "10.0.1.50"
ccu_port = 2010
poll_interval_ms = 2000
request_timeout_ms = 2500
allow_writes = false
```

Ein Datenpunkt wird explizit eingetragen:

```toml
[[homematic_datapoints]]
id = "rack_temperature"
name = "Rack Temperatur"
enabled = true
address = "000A1B2C3D4E5F:1"
parameter = "ACTUAL_TEMPERATURE"
value_type = "float"
platform = "sensor"
device_class = "temperature"
unit = "°C"
writable = false
```

Unterstützte skalare Typen:

```text
bool
integer
float
string
```

Unterstützte Discovery-Plattformen:

```text
sensor
binary_sensor
switch
```

## Schreibschutz

Ein `setValue` erfolgt nur, wenn alle Sperren geöffnet wurden:

```text
homematic.enabled = true
mode = ccu_xml_rpc
homematic.allow_writes = true
Datenpunkt writable = true
aktive Allow-Policy
Command nicht retained, nicht abgelaufen und nicht doppelt
```
