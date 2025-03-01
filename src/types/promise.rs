use crate::errors::ErrorHandling;
use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::value::PyJSValue;
use ion::Promise as JSPromise;
use ion::conversions::FromValue;
use ion::format::Config;
use ion::format::promise::format_promise;
use macros::enum_original_mapping;
use mozjs::jsapi::PromiseState;
use pyo3::{PyResult, pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};

/// Represents a [JSPromise] in the JavaScript Runtime.
/// Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) for more details.
#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSPromise", module = "js_exec.js_exec")]
pub struct PyJSPromise(JSPromise<'static>);

impl From<PyJSPromise> for JSPromise<'static> {
    fn from(value: PyJSPromise) -> Self {
        value.0
    }
}

impl From<JSPromise<'static>> for PyJSPromise {
    fn from(value: JSPromise<'static>) -> Self {
        PyJSPromise(value)
    }
}

#[gen_stub_pyclass_enum]
#[enum_original_mapping(PromiseState)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[pyclass(
    unsendable,
    name = "JSPromiseState",
    module = "js_exec.js_exec",
    eq,
    eq_int
)]
pub enum PyJSPromiseState {
    Pending = 0,
    Fulfilled = 1,
    Rejected = 2,
}

#[allow(clippy::new_without_default)]
#[gen_stub_pymethods]
#[pymethods]
impl PyJSPromise {
    /// Creates a new [JSPromise] which never resolves.
    #[new]
    pub fn new() -> Self {
        with_js_cx(|cx| PyJSPromise(JSPromise::new(cx).extend_lifetime()))
    }

    /// Creates a new [JSPromise] from a [JSValue].
    #[staticmethod]
    pub fn from_value(value: PyJSValue) -> PyResult<Self> {
        with_js_cx(|cx| {
            let promise = JSPromise::from_value(cx, &value.0, true, ())
                .to_value_err("Failed to convert to JSPromise")?;
            Ok(promise.extend_lifetime().into())
        })
    }

    /// Creates a new [Promise], that is resolved to the given value.
    /// Similar to `Promise.resolve`
    #[staticmethod]
    pub fn resolved(value: &PyJSValue) -> Self {
        with_js_cx(|cx| PyJSPromise(JSPromise::resolved(cx, &value.0).extend_lifetime()))
    }

    /// Returns the ID of the [JSPromise].
    pub fn id(&self) -> u64 {
        self.0.id()
    }

    /// Returns the state of the [JSPromise].
    ///
    /// The state can be `Pending`, `Fulfilled` and `Rejected`.
    pub fn state(&self) -> PyJSPromiseState {
        self.0.state().into()
    }

    /// Returns the result of the [JSPromise].
    pub fn result(&self) -> PyJSValue {
        with_js_cx(|cx| self.0.result(cx).extend_lifetime().into())
    }

    /// Resolves the [JSPromise] with the given [JSValue].
    pub fn resolve(&self, value: &PyJSValue) -> bool {
        with_js_cx(|cx| self.0.resolve(cx, &value.0))
    }

    /// Rejects the [JSPromise] with the given [JSValue].
    pub fn reject(&self, value: &PyJSValue) -> bool {
        with_js_cx(|cx| self.0.reject(cx, &value.0))
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_promise(cx, Config::default(), &self.0).to_string())
    }
}
