from fastnomicon import TimeScale, TimeTuple, parse_timestring
from pytest import raises


def test_basic_timestring():
    result = parse_timestring("1h30s")
    assert result == [TimeTuple(time=1, scale=TimeScale.Hours), TimeTuple(time=30, scale=TimeScale.Seconds)]


def test_complex_timestring():
    result = parse_timestring("1 jam 30 menit 500 millis")
    assert result == [
        TimeTuple(time=1, scale=TimeScale.Hours),
        TimeTuple(time=30, scale=TimeScale.Minutes),
        TimeTuple(time=500, scale=TimeScale.Milliseconds),
    ]


def test_invalid_timestring():
    with raises(ValueError) as err:
        parse_timestring("1h30xxxx")
    assert 'Error { input: "xxxx", code: Tag }' in str(err.value)


def test_timestring_attributes():
    data_def = TimeTuple(time=20)
    assert data_def.time == 20
    assert data_def.scale == TimeScale.Seconds
    data_override = TimeTuple(time=15, scale=TimeScale.Minutes)
    assert data_override.time == 15
    assert data_override.scale == TimeScale.Minutes
