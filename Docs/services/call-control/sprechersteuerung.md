# Floor Control

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/call-control/src/state.rs) · [src/http.rs](../../../system-backend/call-control/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Floor-Anforderungen werden zentral an alle aktiven Legs eines logischen Rufs verteilt. Die lokale TBS verwendet dafür ihre vorhandenen `U-TX DEMAND`-, Queue- und `U-TX CEASED`-Prozeduren.

Ohne Force entscheidet die lokale CMCE-State-Machine über Grant oder Queue. Mit `force = true` kann der offene Laboroperator einen vorhandenen Sprecher kontrolliert ablösen. Diese Funktion ist standardmäßig konfigurierbar und wegen des fehlenden RBAC deutlich als Laborfunktion markiert.

Call Control führt den zusammengefassten Floor Holder und die aktuell gemeldete Queue. Maßgeblich bleiben die bestätigten Zustände der TBS.

## Voraussetzung für eine Operator-Anforderung

Alle nicht-terminalen Legs müssen aktiv sein und lokale Call-ID sowie Timeslot besitzen. Außerdem muss der Media Switch die für den Ruf erforderliche Revision mit `POST /api/v1/media/route-ready` bestätigen. Ohne diese Voraussetzungen wird die Floor-Anforderung abgewiesen. `calls.allow_operator_force_floor = false` sperrt den Force-Override auch im Labor.
