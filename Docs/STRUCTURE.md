# Ablage der Markdown-Dokumentation

[Dokumentationsindex](README.md)

- Detaildokumentation wird unter `Docs/` im passenden Themenbereich gepflegt.
- Im Repository-Root bleiben die Projekt-README, `AGENTS.md`, `CODE_OF_CONDUCT.md` und ein kurzer Roadmap-Einstieg.
- Komponenten behalten eine kurze README mit Zweck, lokalen Quellpfaden und Link zur vollständigen Dokumentation. Die zentrale Fassung ist maßgeblich; lange Anleitungen werden nicht doppelt gepflegt.
- Unveränderte Fremdcode-README-, Changelog- und Hardwaredateien bleiben bei ihren Quellen. Neue projektspezifische Anleitungen kommen zentral nach `Docs/`.
- `Docs/archive/` bewahrt historische Gesprächsstände, Importfassungen und deren Assets. Datierte Belege unter `Docs/integration/` bleiben zusammen mit ihren Skripten und Daten.
- Generierte Dokumente werden mit dem zugehörigen Generator aktualisiert; Ausgabeziele und Verbraucher müssen dieselben Pfade verwenden.
- Links innerhalb der Dokumentation sind relativ zum jeweiligen Dokument. Quellpfade in Befehlen beziehen sich auf das bezeichnete Quellverzeichnis oder den ausdrücklich genannten Repository-Root.
- Wiki-Navigation verwendet reguläre Markdown-Links. Die frühere Repo-Ablage `wiki/` enthält noch einen Einstieg und verwendete Assets.

Die Umzüge vom 09.10.2026 sind in [documentation-paths.json](documentation-paths.json) nachvollziehbar. Aufgaben-IDs, Status und Historie der Fachroadmaps bleiben erhalten.
