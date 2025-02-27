use crate::errors::ErrorHandling;
use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::owned_key::PyOwnedKey;
use crate::types::symbol::PySymbol;
use crate::types::value::PyJSValue;
use ion::conversions::{ToPropertyKey, ToValue};
use ion::{Context, PropertyKey};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use pyo3_stub_gen::{PyStubType, TypeInfo};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "PropertyKey", module = "js_exec.js_exec")]
pub struct PyPropertyKey(pub PropertyKey<'static>);

impl From<PyPropertyKey> for PropertyKey<'static> {
    fn from(key: PyPropertyKey) -> Self {
        key.0
    }
}

impl From<PropertyKey<'static>> for PyPropertyKey {
    fn from(key: PropertyKey<'static>) -> Self {
        PyPropertyKey(key)
    }
}

impl<'py> FromPyObject<'py> for PyPropertyKey {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(prop_key) = ob.downcast::<PyPropertyKey>() {
            with_js_cx(|cx| {
                let borrowed = prop_key.borrow();
                let obj = PropertyKey::from_value(cx, &borrowed.0.as_value(cx))
                    .ok_or(PyValueError::new_err("Failed to convert to PropertyKey"))?;
                Ok(obj.extend_lifetime().into())
            })
        } else {
            Err(PyValueError::new_err("Failed to convert to PropertyKey"))
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPropertyKey {
    /// Creates a [PropertyKey] from an integer.
    #[staticmethod]
    pub fn with_int(value: i32) -> Self {
        with_js_cx(|cx| PyPropertyKey(PropertyKey::with_int(cx, value).extend_lifetime()))
    }

    /// Creates a [PropertyKey] from a string.
    #[staticmethod]
    pub fn with_string(value: &str) -> Option<Self> {
        with_js_cx(|cx| {
            PropertyKey::with_string(cx, value).map(|key| PyPropertyKey(key.extend_lifetime()))
        })
    }

    #[staticmethod]
    pub fn with_symbol(symbol: &PySymbol) -> Self {
        with_js_cx(|cx| PyPropertyKey(PropertyKey::with_symbol(cx, &symbol.0).extend_lifetime()))
    }

    #[staticmethod]
    pub fn from_value(value: &PyJSValue) -> Option<Self> {
        with_js_cx(|cx| {
            PropertyKey::from_value(cx, &value.0).map(|key| PyPropertyKey(key.extend_lifetime()))
        })
    }

    pub fn to_owned_key(&self) -> PyResult<PyOwnedKey> {
        with_js_cx(|cx| {
            self.0
                .to_owned_key(cx)
                .map(|key| PyOwnedKey::from(key.extend_lifetime()))
                .to_value_err("Failed to convert to owned key")
        })
    }

    // handle methods
    pub fn is_void(&self) -> bool {
        self.0.handle().is_void()
    }

    pub fn is_int(&self) -> bool {
        self.0.handle().is_int()
    }

    pub fn is_string(&self) -> bool {
        self.0.handle().is_string()
    }

    pub fn is_symbol(&self) -> bool {
        self.0.handle().is_symbol()
    }

    /// Garbage Collected Thing
    pub fn is_gcthing(&self) -> bool {
        self.0.handle().is_gcthing()
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| {
            let value = self.0.as_value(cx);
            value
                .to_source(cx)
                .to_owned(cx)
                // remove the quotes
                .map(|s| s[1..s.len() - 1].to_string())
                .unwrap_or("undefined".to_string())
        })
    }

    pub fn __repr__(&self) -> String {
        with_js_cx(|cx| {
            let value = self.0.as_value(cx);
            let str = value
                .to_source(cx)
                .to_owned(cx)
                // remove the quotes
                .map(|s| s[1..s.len() - 1].to_string())
                .unwrap_or("undefined".to_string());
            format!("PropertyKey({})", str)
        })
    }
}

pub enum PropertyKeyTypes {
    Typed(PropertyKey<'static>),
    Raw(String),
}

impl PyStubType for PropertyKeyTypes {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Union[builtins.str, PropertyKey]".to_string(),
            import: {
                let mut import = std::collections::HashSet::new();
                import.insert("builtins".into());
                import.insert("typing".into());
                import
            },
        }
    }
}

impl<'py> FromPyObject<'py> for PropertyKeyTypes {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(prop_key) = ob.extract::<PyPropertyKey>() {
            return Ok(PropertyKeyTypes::Typed(prop_key.0));
        }

        if let Ok(s) = ob.extract::<String>() {
            return Ok(PropertyKeyTypes::Raw(s));
        }

        Err(PyValueError::new_err(
            "Expected PropertyKey, string or integer",
        ))
    }
}

impl<'cx> ToPropertyKey<'cx> for PropertyKeyTypes {
    fn to_key(&self, cx: &'cx Context) -> Option<PropertyKey<'cx>> {
        match self {
            PropertyKeyTypes::Typed(key) => PropertyKey::from_value(cx, &key.as_value(cx)),
            PropertyKeyTypes::Raw(key) => key.to_key(cx),
        }
    }
}
