# Retention, Legal Hold und Integrität

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/recorder/src/state.rs) · [src/http.rs](../../../system-backend/recorder/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Retention

Beim Finalisieren wird `retention_until` aus Rufende plus `retention_days` berechnet. Der Wartungsworker prüft abgelaufene Aufnahmen regelmäßig und entfernt sie nur, wenn kein Legal Hold aktiv ist.

Eine Änderung über die API setzt die Frist erneut ausgehend vom Rufende. Der erlaubte Bereich liegt bei 1 bis 3650 Tagen.

## Legal Hold

`legal_hold = true` blockiert sowohl die automatische Retention-Löschung als auch manuelle Löschung. Das Lösen des Holds löscht die Aufnahme nicht sofort; sie wird beim nächsten Retention-Lauf entfernt, wenn die Frist bereits abgelaufen ist.

## Löschung

Löschung ist endgültig und entfernt:

- das komplette Aufnahmeverzeichnis
- einen eventuell erzeugten TAR-Export
- den Eintrag im Recorder-Zustand

`security.allow_delete = false` sperrt die **manuelle API-Löschung**. Die automatische Retention-Auswertung prüft diesen Schalter nicht und läuft weiter. Soll eine Aufnahme unabhängig von der Frist erhalten bleiben, `legal_hold = true` setzen. Das Flag ist eine technische Löschsperre; eine rechtliche Aufbewahrungsregel wird dadurch nicht automatisch festgelegt.

## Integritätsprüfung

Die API berechnet SHA-256 über Audio und Index neu und vergleicht beide Werte mit `integrity.json`. Ein Fehler setzt den Status auf `failed` und erzeugt ein Ereignis. Die API liefert dann absichtlich einen Fehlerstatus zurück.
