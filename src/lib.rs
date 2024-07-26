mod test_mod;
mod channel;
mod connection;
mod node;
mod machine;
mod token;
mod util;

use pyo3::prelude::*;

#[pymodule]
fn elvos(m: &Bound<'_, pyo3::types::PyModule, >) -> PyResult<()> {
    connection::PyConnection(m)?;
    machine::PyMyCoolMachine(m)?;
    node::Nodes(m)?;
    util::util_wrappers(m)?;
    Ok(())
}