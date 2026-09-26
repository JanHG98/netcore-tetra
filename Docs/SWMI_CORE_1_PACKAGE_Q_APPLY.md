# SWMI Core 1 – Paket Q anwenden

## Neue Dateien

- `tests/e2e/` – Runner, Mock TBS, Szenarien, Reports, Unit Tests und On-Air-Evidenz;
- `deploy/open-lab/netcore-e2e.py` – direkter Wrapper;
- `tools/check_e2e_integration.py` – statischer Paketchecker;
- `.github/workflows/swmi-core-e2e-integration.yml` – CI-Selbsttest;
- `Docs/OPEN_LAB_E2E_RUNBOOK.md` – Betrieb und Abnahme;
- `Docs/SWMI_CORE_1_PACKAGE_Q_E2E_INTEGRATION.md` – Architektur und Umfang.

## Einmalige Vorbereitung

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
```

## Ausführung

```bash
# nur lesen
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke

# Fachflüsse mit temporären Testdaten
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --allow-mutations

# Neustarts und absichtliche Dependency-Ausfälle
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile fault --allow-mutations --allow-restarts
```

## Rückbau

Der Runner entfernt selbst erzeugte Subscriber-, Group- und Membership-Fixtures standardmäßig. Bei einem abgebrochenen Fault-Lauf müssen gestoppte Dienste anhand des Inventories wieder gestartet werden. Testartefakte unter `tests/e2e/artifacts/` können nach Sicherung der Evidenz gelöscht werden.


## Zusätzliche Managementebenen-Prüfung

Der vollständige Lauf enthält außerdem `control-room-federation` und `platform-services`. Damit werden das zentrale Lagebild sowie die redaktierten Managementansichten von Security Core, KMF, Transit, Application Gateway und Media Library geprüft, ohne Schlüssel- oder Mediendaten offenzulegen.
