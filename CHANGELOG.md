# Changelog

The following file contains all the changes made in `fastnomicon`

## Unreleased

Nothing yet!

## [0.2.0] (2025-02-07)
### Features
- Add function to easily format `TimeTuple` into timestring format again (`expand_timestring`)
- Add `short_form`, `long_form`, and `long_form_plural` method to `TimeScale`

### Build
- [BREAKING CHANGES] Make math related function behind `math` feature-gate.
- Bump `nom` to `8.0.0`

## [0.1.0] (2024-10-16)

First version release 🎉

### Features
- Added `parse_timestring_as_timedelta` function to convert `timestring` to `timedelta`
- Added `parse_timestring` function to convert `timestring` to `list[TimeTuple]`
- Added `execute_math_expr` function to execute mathematical expression
