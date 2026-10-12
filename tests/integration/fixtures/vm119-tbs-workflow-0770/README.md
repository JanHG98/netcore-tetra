# Historische UI-Quelle des VM119-Workflow-Updaters

Diese beiden unveränderten Dateien stammen aus `0770e903b7dfc2f8410ad6f32cdc35e014adbbc7` und werden nur für den Quellpin-Test des historischen sieben-Dateien-Updaters gelesen. Das KDE-Imageformular ersetzt sie in der laufenden Produktion. Der alte Updater und seine SHA-Freigaben bleiben unverändert.

- `static/index.html`: SHA-256 `c2621415484332ec50b4d9254d500ad53f8e7895245ab3ac051b625ca13e8771`
- `static/app.js`: SHA-256 `8c9d0fdd37028d084e0f44d614f886446394c9baba210cd6e46f4d550fda1adf`

Die übrigen fünf weiterhin unveränderten Quellpins werden gegen ihre Repositorystände geprüft. Die Verhaltenstests benutzen weiterhin temporäre Dateien, echte SQLite-Datenbanken und isolierte Dienste.
