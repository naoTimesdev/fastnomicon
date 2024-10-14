from fastnomicon import execute_math_expr
from pytest import raises


def test_basic_math():
    result = execute_math_expr("1 + 1")
    assert result == 2.0


def test_ambiguous_math():
    result = execute_math_expr("6/2*(1+2)")
    assert result == 9.0


def test_sin_math():
    result = execute_math_expr("sin(pi/2)")
    assert result == 1.0


def test_unknown_math():
    with raises(ValueError) as err:
        execute_math_expr("PI * 2")  # It's `pi` all lowercase and not `PI`
    assert "Unknown Variable: PI" in str(err.value)
