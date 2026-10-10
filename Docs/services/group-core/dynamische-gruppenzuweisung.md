# DGNA

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/group-core/src/state.rs) · [src/http.rs](../../../system-backend/group-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Eine DGNA-Operation wird mit Node-ID, ISSI, GSSI und Attach/Detach angelegt. Der Group Core korreliert Node-Gateway-Request-ID, TBS-Command-ID und die abschließende `GroupDgnaApplied`-Antwort.

Die TBS verwendet bei einem zentral verwalteten Gruppenprofil dessen `class_of_usage`. Ohne zentrale Richtlinie bleibt der bisherige lokale Standardwert erhalten.

`force = true` ist ein bewusster Operator-Override und umgeht die zentrale Gruppen- und Mitgliedschaftsfreigabe sowie die entsprechende lokale Policy-Prüfung. Nicht umgangen werden:

- die Registrierung des Zielteilnehmers auf der TBS
- die technische Gültigkeit der GSSI
- die Fähigkeit der TBS, DGNA zu verarbeiten

Der Open-Lab-Modus besitzt noch kein RBAC; deshalb darf die WebUI nur in einem isolierten Testnetz erreichbar sein.

## Annahme und Funkwirkung

`POST /api/v1/dgna` erzeugt einen korrelierten Auftrag; die Annahme im Gateway ersetzt weder `GroupDgnaApplied` noch die Beobachtung des Funkgeräts. Die Timeoutgrenze steht in `policy.dgna_timeout_secs` (Beispiel: 30 Sekunden). Bei einem Fehler zuerst den Operationseintrag und die TBS-Antwort prüfen, bevor dieselbe Zuweisung wiederholt wird.
