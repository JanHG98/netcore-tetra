# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für wait.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import time
from typing import Callable, TypeVar

T = TypeVar("T")


# Was: Bündelt Daten und Verhalten für wait timeout.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class WaitTimeout(RuntimeError):
    pass


# Was: Diese Funktion wartet for.
# Warum: Nachfolgende Schritte laufen dadurch erst weiter, wenn ihre Voraussetzung wirklich erfüllt ist.
def wait_for(
    description: str,
    callback: Callable[[], T],
    predicate: Callable[[T], bool],
    *,
    timeout: float = 20.0,
    interval: float = 0.4,
) -> T:
    deadline = time.monotonic() + timeout
    last_value: T | None = None
    last_error: BaseException | None = None
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    while time.monotonic() < deadline:
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            last_value = callback()
            if predicate(last_value):
                return last_value
        except BaseException as error:  # keep transient HTTP failures for the final explanation
            last_error = error
        time.sleep(interval)
    detail = f"; last_error={last_error}" if last_error is not None else f"; last_value={last_value!r}"
    raise WaitTimeout(f"timed out waiting for {description}{detail}")
