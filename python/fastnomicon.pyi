"""
fastnomicion
~~~~~~~~~~~~
A collection of utilities used by naoTimes bot

:copyright: (c) 2024-present naoTimesdev
:license: MPL 2.0, see LICENSE for more details.
"""

from datetime import timedelta

__version__: str

class TimeScale:
    Milliseconds: TimeScale
    """Milliseconds timescale, 1/1000 of a second"""
    Seconds: TimeScale
    """Seconds timescale, a.k.a we will have no changes"""
    Minutes: TimeScale
    """Minutes timescale, multiplied by 60 seconds"""
    Hours: TimeScale
    """Hours timescale, multiplied by 3600 seconds"""
    Days: TimeScale
    """Days timescale, assumed to be 24 hours long"""
    Weeks: TimeScale
    """Weeks timescale, assumed to be 7 days long"""
    Months: TimeScale
    """Months timescale, assumed to be 30 days long"""
    Years: TimeScale
    """Years timescale, asssumed to be 365 days long"""

    def __eq__(self, value: TimeScale) -> bool:
        ...
    def __ne__(self, value: TimeScale) -> bool:
        ...
    def __repr__(self) -> str:
        ...
    def __str__(self) -> str:
        ...

    @property
    def name(self) -> TimeScale:
        """Get the name of the TimeScale, this will return the same thing"""

    @property
    def value(self) -> str:
        """Get the value of the TimeScale, this will return the string representation of it"""

class TimeTuple:
    """
    A simple time tuple that wraps a number in seconds and the time scale.
    """

    def __init__(self, time: int, scale: TimeScale) -> None:
        """Create a new instance of TimeTuple"""
        ...

    def __repr__(self) -> str:
        ...

    @property
    def time(self) -> int:
        """Get the time value of the class"""
        ...

    @property
    def scale(self) -> TimeScale:
        """Get the time scale of the class"""
        ...

    def as_duration(self) -> timedelta:
        """Convert the time tuple into a duration with proper time scaling"""
        ...

def parse_timestring(input: str) -> list[TimeTuple]:
    """Parse a timestring formatted input into a list of parsed parts

    For more info about timestring format, please see here:
    https://naoti.me/docs/referensi/timestring

    Parameters
    ----------
    input: :class:`str`
        The input data to be parsed

    Returns
    -------
    :class:`list[TimeTuple]`
        The list of parsed input

    Raises
    ------
    ValueError
        If an invalid input is entered
    """
    ...

def parse_timestring_as_timedelta(input: str) -> timedelta:
    """Parse a timestring formatted input into a :class:`timedelta` or duration

    For more info about timestring format, please see here:
    https://naoti.me/docs/referensi/timestring

    Parameters
    ----------
    input: :class:`str`
        The input data to be parsed

    Returns
    -------
    :class:`datetime.timedelta`
        The parsed duration of the input

    Raises
    ------
    ValueError
        If an invalid input is entered
    """
    ...

def execute_math_expr(input: str) -> float:
    """Execute a mathematical expression using Shunting yard algorithm

    All the standard math operators are supported with extended support for:
    - abs
    - atan2
    - cos
    - log(10)
    - max
    - min
    - nCr
    - nMCr
    - nMPr
    - nPr
    - rand
    - sing

    Also the following math function are supported:
    - normal()
    - uniform()
    - lognormal()

    Parameters
    ----------
    input: :class:`str`
        The math expression to be executed

    Returns
    -------
    :class:`float`
        The resulting number

    Raises
    ------
    ValueError
        If an invalid input is entered
    """
