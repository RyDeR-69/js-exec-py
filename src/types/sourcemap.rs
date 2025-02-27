use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use sourcemap::SourceMap;

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "SourceMap", module = "js_exec.js_exec")]
pub struct PySourceMap(pub SourceMap);

impl From<PySourceMap> for SourceMap {
    fn from(value: PySourceMap) -> Self {
        value.0
    }
}
impl From<SourceMap> for PySourceMap {
    fn from(value: SourceMap) -> Self {
        PySourceMap(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySourceMap {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SourceMap({:?})", self.0))
    }
}
