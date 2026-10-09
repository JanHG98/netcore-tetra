# Jitter-Puffer und Kaltstart-Vorpuffer

Der normale Zielstream besitzt weiterhin einen begrenzten FIFO-Puffer. Die nominelle
Startverzögerung ergibt sich aus:

```text
frame_duration_ms * jitter_buffer_frames
```

Mit den neuen Standardwerten sind dies `60 ms * 2 = 120 ms`. Der adaptive Regler darf
bei einem sehr stabilen lokalen Netz bis auf `min_jitter_buffer_frames = 1` absenken und
bei messbaren Laufzeitschwankungen schrittweise bis zur konfigurierten Obergrenze erhöhen.
Er reagiert ausschließlich auf die Paketankunftszeiten eines Quellstreams; vorgepufferte
Kaltstart-Frames verändern die Jittermessung nicht.

## Kaltstart eines neuen Rufes

Sprachframes, für die Call Control den vollständigen Routinggraphen noch nicht geliefert
hat, werden nicht mehr sofort als `unknown_stream` verworfen. Pro Quellstream hält der
Media Switch bis zu `cold_start_buffer_frames` Frames zurück. Standardmäßig sind dies
fünf 60-ms-Pakete, also maximal ungefähr 300 ms Sprache.

Sobald alle erwarteten Call-Legs aktiv, mit lokaler Call-ID sowie Timeslot bekannt und
die zugehörigen Node-Gateway-Medienwege verbunden sind, werden die Frames in ihrer
ursprünglichen Reihenfolge mit 60-ms-Abstand in die Zielpuffer übernommen. Der Media
Switch meldet diesen Zustand revisionsgebunden als
`RouteReady` an Call Control. Solange noch ein erwartetes Leg fehlt, wird **nicht** auf
einen unvollständigen Zielgraphen ausgesendet; fehlgeschlagene oder beendete Legs müssen
zuerst von Call Control aus dem erwarteten Graphen entfernt werden.
`cold_start_buffer_max_age_ms` begrenzt die Lebensdauer zurückgehaltener Frames, damit ein
dauerhaft unvollständiger Ruf keinen Altbestand in den Medienpfad trägt.

Beim normalen Zielpuffer wird bei Überlauf der älteste Frame entfernt, damit die
Latenz nicht unbegrenzt wächst. Der Kaltstartpuffer verhält sich absichtlich anders:
Er behält die ersten Frames und verwirft bei voller Kapazität neuere Eingänge. So bleiben
die ersten gesprochenen Wörter erhalten, bis ein nutzbares Ziel-Leg bereit ist. Duplikate
oder rückwärts laufende Sequenznummern werden vor dem Zielpuffer verworfen.
