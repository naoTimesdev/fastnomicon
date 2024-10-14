use pyo3::{exceptions::PyValueError, prelude::*};

/// Execute a math expression using Shunting yard algorithm
#[pyfunction]
fn execute_math_expr(input: String) -> PyResult<f64> {
    ::fastnomicon::math::execute_math_expr(&input)
        .map_err(|err| PyValueError::new_err(format!("Failed to parse expression: {}", err)))
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Function
    m.add_function(wrap_pyfunction!(execute_math_expr, m)?)?;

    Ok(())
}
