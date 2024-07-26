use pyo3::prelude::*;

#[pyfunction]
pub fn double(x: usize) -> usize {
    x * 2
}

pub mod mult {
    use super::*;

    #[pyfunction] // This will be part of the module
    pub fn triple(x: usize) -> usize {
        x * 3
    }

    #[pyclass] // This will be part of the module
    struct Unit;

    #[pymodule]
    mod submodule {
        // This is a submodule
    }

    #[pymodule]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let submodule = PyModule::new_bound(m.py(), "mult")?;
        submodule.add("triple", wrap_pyfunction!(triple, m)?)?;
    
        m.add_submodule(&submodule)
    }
}