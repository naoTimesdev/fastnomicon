//! Math parser with Shunting algorithm

use shunting::MathContext;
use shunting::ShuntingParser;

pub fn execute_math_expr(input: &str) -> Result<f64, String> {
    let expr = ShuntingParser::parse_str(input)?;
    let context = MathContext::new();
    context.eval(&expr)
}
