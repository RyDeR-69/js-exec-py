use mozjs::jsapi::ESClass;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "ESClass", module = "js_exec.js_exec", eq)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PyESClass(pub ESClass);

impl From<PyESClass> for ESClass {
    fn from(value: PyESClass) -> Self {
        value.0
    }
}

impl From<ESClass> for PyESClass {
    fn from(value: ESClass) -> Self {
        PyESClass(value)
    }
}

/// Enumeration describing possible values of the [JSClass] internal property value of objects.
#[gen_stub_pymethods]
#[pymethods]
impl PyESClass {
    #[staticmethod]
    pub fn object() -> Self {
        Self(ESClass::Object)
    }

    #[staticmethod]
    pub fn array() -> Self {
        Self(ESClass::Array)
    }

    #[staticmethod]
    pub fn number() -> Self {
        Self(ESClass::Number)
    }

    #[staticmethod]
    pub fn string() -> Self {
        Self(ESClass::String)
    }

    #[staticmethod]
    pub fn boolean() -> Self {
        Self(ESClass::Boolean)
    }

    #[staticmethod]
    pub fn regexp() -> Self {
        Self(ESClass::RegExp)
    }

    #[staticmethod]
    pub fn array_buffer() -> Self {
        Self(ESClass::ArrayBuffer)
    }

    #[staticmethod]
    pub fn shared_array_buffer() -> Self {
        Self(ESClass::SharedArrayBuffer)
    }

    #[staticmethod]
    pub fn date() -> Self {
        Self(ESClass::Date)
    }

    #[staticmethod]
    pub fn set() -> Self {
        Self(ESClass::Set)
    }

    #[staticmethod]
    pub fn map() -> Self {
        Self(ESClass::Map)
    }

    #[staticmethod]
    pub fn promise() -> Self {
        Self(ESClass::Promise)
    }

    #[staticmethod]
    pub fn map_iterator() -> Self {
        Self(ESClass::MapIterator)
    }

    #[staticmethod]
    pub fn set_iterator() -> Self {
        Self(ESClass::SetIterator)
    }

    #[staticmethod]
    pub fn arguments() -> Self {
        Self(ESClass::Arguments)
    }

    #[staticmethod]
    pub fn error() -> Self {
        Self(ESClass::Error)
    }

    #[staticmethod]
    pub fn bigint() -> Self {
        Self(ESClass::BigInt)
    }

    #[staticmethod]
    pub fn function() -> Self {
        Self(ESClass::Function)
    }

    #[staticmethod]
    pub fn other() -> Self {
        Self(ESClass::Other)
    }

    pub fn __repr__(&self) -> String {
        format!("ESClass({:?})", self.0)
    }
}
