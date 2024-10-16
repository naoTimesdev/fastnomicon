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


def test_permutation():
    result = execute_math_expr("nPr(5,2)")
    assert result == 20.0


def test_combination():
    result = execute_math_expr("nCr(10,2)")
    assert result == 45.0


def test_power_of():
    result = execute_math_expr("2 ** 3")
    assert result == 8.0
    alter = execute_math_expr("nMPr(2, 3)")
    assert alter == 8.0


def test_min():
    result = execute_math_expr("min(4, 2, 2, 4, 5)")
    assert result == 2.0


def test_max():
    result = execute_math_expr("max(10, 8, 20, 9, 1, 5)")
    assert result == 20.0


def test_absolute():
    result = execute_math_expr("abs(-5.0)")
    assert result == 5.0


def test_approx_math_constants():
    result = execute_math_expr("pi")
    assert result == 3.141592653589793

    result = execute_math_expr("e")
    assert result == 2.718281828459045

    result = execute_math_expr("tau")
    assert result == 6.283185307179586



def test_unknown_math():
    with raises(ValueError) as err:
        execute_math_expr("PI * 2")  # It's `pi` all lowercase and not `PI`
    assert "Unknown Variable: PI" in str(err.value)
