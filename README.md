# fastnomicon

A python library for fast parser used in naoTimes bot.

## Requirements
- Python 3.10+
- Rust 1.80.0+

## Functions
- `parse_timestring`
- `parse_timestring_as_timedelta`

A quick an easy [timestring](https://naoti.me/docs/referensi/timestring) parsing, easily convert text like
`1h30m` into a proper [`timedelta`](https://docs.python.org/3/library/datetime.html#timedelta-objects) object.

```py
from fastnomicon import parse_timestring_as_timedelta

t_delta = parse_timestring_as_timedelta("1h30m")  # or you can also write it with space
print(t_delta)  # -> 1:30:00
```

The standard `parse_timestring` will be a custom class called `TimeTuple` which contains the stack information about each timestring (e.g. `1h30m` will be `[TimeTuple(time=1, scale=Hours), TimeTuple(time=30, scale=Minutes)]`).

- `execute_math_expr` (available with `math` feature)

A quick and easy way to parse complex mathematical expression using **Shunting-yard** algorithm.

```py
from fastnomicon import execute_math_expr

result = execute_math_expr("6/2*(1+2)")  # or you can also write it with space
print(result)  # -> 9.0
```

The following function are supported in the expression, extending the standard PEMDAS expression:
- `abs(x)` — Absolute number of x
- `atan2(x, y)` — 2-argument arctangent
- `cos(x)` — Cosine
- `log(x)` — Logarithmic 10
- `max(x, y, z, ...)` — Largest number, can be repeated
- `min(x, y, z, ...)` — Smallest number, can be repeated
- `nCr(x, y)` — Combination (order not important)
- `nMCr(x, y)` — Multi combination
- `nMPr(x, y)` — Power of number (order not important)
- `nPr(x, y)` — Permutation
- `rand(x)` — Random number multiplied by x
- `sin(x)` — Sine
- `normal(mu, sigma)` — Normal distribution
- `uniform(mu, sigma)` — Uniform distribution
- `lognormal(mu, sigma)` — Log-normal distribution

We also provide the following approximation constants:
- `pi` — `3.141592653589793`, Archimedes' constant (π)
- `tau` — `6.283185307179586`, The full circle constant (τ)
- `e` — `2.718281828459045`, The Euler's number constant (e)

## License

The fastnomicon crate and python bindings are licensed under the [MPL 2.0](https://mozilla.org/MPL/2.0/) license.
