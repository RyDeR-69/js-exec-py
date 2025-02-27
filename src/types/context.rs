use ion::Context as JSContext;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSContext", module = "js_exec.js_exec")]
pub struct PyJSContext(pub JSContext);

impl From<PyJSContext> for JSContext {
    fn from(value: PyJSContext) -> Self {
        value.0
    }
}

impl From<JSContext> for PyJSContext {
    fn from(value: JSContext) -> Self {
        PyJSContext(value)
    }
}
