from __future__ import annotations

import time
from typing import Callable, TypeVar

T = TypeVar("T")


class WaitTimeout(RuntimeError):
    pass


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
    while time.monotonic() < deadline:
        try:
            last_value = callback()
            if predicate(last_value):
                return last_value
        except BaseException as error:  # keep transient HTTP failures for the final explanation
            last_error = error
        time.sleep(interval)
    detail = f"; last_error={last_error}" if last_error is not None else f"; last_value={last_value!r}"
    raise WaitTimeout(f"timed out waiting for {description}{detail}")
