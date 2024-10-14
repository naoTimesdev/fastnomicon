"""
fastnomicion
~~~~~~~~~~~~
A collection of fast utilities used by naoTimes bot

:copyright: (c) 2024-present naoTimesdev
:license: MPL 2.0, see LICENSE for more details.
"""

from __future__ import annotations

from ._fastnomicon import (  # type: ignore
    TimeScale,
    TimeTuple,
    __version__,
    execute_math_expr,
    parse_timestring,
    parse_timestring_as_timedelta,
)

__all__ = (
    "__version__",
    "execute_math_expr",
    "parse_timestring",
    "parse_timestring_as_timedelta",
    "TimeScale",
    "TimeTuple",
)
