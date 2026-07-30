# Codec-Helfervertrag

Die Media Library behauptet keinen eingebauten TETRA-Sprachcodec. Optional konfigurierte Helfer werden direkt mit `Command`, also ohne Shell, gestartet.

## Encoder

```toml
encoder_command = ["/usr/local/bin/netcore-tetra-encode", "--input", "{input}", "--output", "{output}"]
```

Eingabe: 8 kHz, mono, PCM16 WAV. Ausgabe: positive Anzahl gepackter Frames mit exakt 35 Byte je Frame.

## Decoder

```toml
decoder_command = ["/usr/local/bin/netcore-tetra-decode", "--input", "{input}", "--output", "{output}"]
```

Eingabe: gepacktes TACELP. Ausgabe: 8 kHz, mono, PCM16 WAV.

Der Prozess gilt nur dann als erfolgreich, wenn Exit-Code und Ausgabedatei stimmen und die Media Library das Ergebnis erneut validiert.

## Basisstations-Playout

Bei `playout.mode = "basisstation"` bleibt `encoder_command = []` zulässig. Die
Media Library übergibt die freigegebene WAV-Vorschau an die TBS. Dort wird sie mit
dem bereits in der Basisstation eingebundenen nativen `TetraSpeechEncoder` in
vollständige TETRA-Sprachblöcke umgewandelt. Der Helfervertrag dieses Dokuments gilt
nur für den optionalen direkten `media_switch`-Pfad und für zentral importierte
`.tacelp`-Medien.
