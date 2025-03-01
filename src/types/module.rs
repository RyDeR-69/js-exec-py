use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::promise::PyJSPromise;
use ion::module::{Module as JSModule, Module};
use pyo3::exceptions::PyRuntimeError;
use pyo3::{PyResult, pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSModule", module = "js_exec.js_exec")]
pub struct PyJSModule(JSModule<'static>);

impl From<PyJSModule> for JSModule<'static> {
    fn from(value: PyJSModule) -> Self {
        value.0
    }
}

impl From<JSModule<'static>> for PyJSModule {
    fn from(value: JSModule<'static>) -> Self {
        PyJSModule(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyJSModule {
    /// Compiles and evaluates a [Module] with the given source and filename.
    /// On success, returns the compiled module object and a promise. The promise resolves with the return value of the module.
    /// The promise is a byproduct of enabling top-level await.
    #[pyo3(signature = (source, filename = "inline.js", path = Some("inline.js")))]
    pub fn compile_and_evaluate(
        &self,
        source: &str,
        filename: &str,
        path: Option<&str>,
    ) -> PyResult<(PyJSModule, Option<PyJSPromise>)> {
        with_js_cx(|cx| {
            let (module, promise) =
                Module::compile_and_evaluate(cx, filename, path.map(AsRef::as_ref), source)
                    .map_err(|e| PyRuntimeError::new_err(e.format(cx)))?;

            Ok((
                module.extend_lifetime().into(),
                promise.map(|p| p.extend_lifetime().into()),
            ))
        })
    }

    /// Returns `true` if the module has been linked.
    pub fn is_linked(&self) -> bool {
        self.0.is_linked()
    }
}
