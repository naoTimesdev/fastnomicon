use pyo3::prelude::*;

mod timestring;

#[pymodule]
#[pyo3(name = "fastnomicon")]
fn entrypoint(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    timestring::register(m)?;

    Ok(())
}
