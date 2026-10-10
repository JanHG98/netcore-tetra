# Arbeiten am NetCore-Tetra-Repository

## Gesamtroadmap und Fortsetzung

- Bei Fragen nach Projektstand, Prioritäten oder „Was machen wir als Nächstes?“ zuerst die aktuelle `Docs/roadmaps/gesamtroadmap.md` lesen. Sie ist der zentrale Einstieg für die Gesamtfolge; die dort verlinkten Fachroadmaps enthalten den Detailumfang.
- Den aktuellen main-Stand und relevante Änderungen / Arbeitsnachweise prüfen. Datierte gültige Belege bei unverändertem Inhalt wiederverwenden; geplanten Umfang, vorhandenen Code, statische Prüfung, Lab- und On-Air-Abnahme getrennt bewerten.
- Die höchstpriorisierte offene Aufgabe mit erfüllten Abhängigkeiten innerhalb des aktuellen Nutzerauftrags wählen. Aufgaben-ID, Grund, konkreten Umfang und Abnahmekriterium nennen. Blockierte P0-Arbeit erklären und mögliche unabhängige Arbeit angeben.
- Nach tatsächlichem Fortschritt Status / Nachweise und den nächsten Schritt in `Docs/roadmaps/gesamtroadmap.md` aktualisieren; Aufgaben-IDs und Historie erhalten. Eine abgeschlossene Analyse ist kein Nachweis der nachfolgenden Implementierung.
- Ausdrückliche aktuelle Nutzerentscheidungen und bereits erteilte Freigaben haben Vorrang vor der Planungsreihenfolge. Die Roadmap allein ist keine pauschale Implementierungs- oder Deploymentautorisierung; eine Frage nach dem nächsten Schritt zunächst als Auskunft beantworten. Für bereits autorisierte Arbeit keine zusätzliche allgemeine Bestätigungspflicht ableiten.

## Dokumentationsablage

- Detaildokumentation zentral unter `Docs/` pflegen; Einstieg und Themenzuordnung stehen in `Docs/README.md` und `Docs/dokumentationsablage.md`.
- Im Quellbaum kurze Komponenten-READMEs beibehalten und auf die zentrale Anleitung verlinken. Keine neuen verteilten Detailanleitungen oder doppelten Langfassungen anlegen.
- Fachdateien mit verständlichen deutschen Namen benennen; IDs im Inhalt erhalten. Inhalte gegen die konkrete Quelle prüfen, historische Befunde datieren und Generatorausgaben über ihren Generator pflegen.
- Funktionale Markdown-Dateien und unveränderte Fremdcode-Dokumentation bleiben bei ihren Quellen. Bei Umzügen Links, Installer, Prüfscripts und Generator-Ausgabeziele gemeinsam aktualisieren.
