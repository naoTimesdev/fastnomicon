//! Math parser with Shunting algorithm

use shunting::{MathContext, MathOp, ShuntingParser};
use std::f64::consts;

pub fn execute_math_expr(input: &str) -> Result<f64, String> {
    let expr = ShuntingParser::parse_str(input)?;
    let context = MathContext::new();
    context.setvar("tau", MathOp::Number(consts::TAU));
    context.eval(&expr)
}
