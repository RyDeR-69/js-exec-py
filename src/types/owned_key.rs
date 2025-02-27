use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::symbol::PySymbol;
use ion::OwnedKey;
use ion::format::Config;
use ion::format::key::format_key;
use ion::format::symbol::format_symbol;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(
    unsendable,
    frozen,
    name = "OwnedKey",
    module = "js_exec.js_exec",
    eq,
    hash
)]
#[derive(Eq, Hash, PartialEq)]
pub struct PyOwnedKey(OwnedKey<'static>);

impl From<PyOwnedKey> for OwnedKey<'static> {
    fn from(value: PyOwnedKey) -> Self {
        value.0
    }
}

impl From<OwnedKey<'static>> for PyOwnedKey {
    fn from(value: OwnedKey<'static>) -> Self {
        PyOwnedKey(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyOwnedKey {
    /// Creates a new OwnedKey representing an integer value
    #[staticmethod]
    pub fn int(value: i32) -> Self {
        PyOwnedKey(OwnedKey::Int(value))
    }

    /// Creates a new OwnedKey representing a string value
    #[staticmethod]
    pub fn string(value: String) -> Self {
        PyOwnedKey(OwnedKey::String(value))
    }

    /// Creates a new OwnedKey representing a symbol
    #[staticmethod]
    pub fn symbol(symbol: &PySymbol) -> Self {
        with_js_cx(|cx| {
            let cloned_symbol: ion::Symbol = cx.root(symbol.0.get()).into();
            PyOwnedKey(OwnedKey::Symbol(cloned_symbol.extend_lifetime()))
        })
    }

    /// Creates a new void OwnedKey
    #[staticmethod]
    pub fn void() -> Self {
        PyOwnedKey(OwnedKey::Void)
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| {
            let formatted = format_key(cx, Config::default(), &self.0).to_string();

            // Safely remove only the surrounding quotes if they exist
            if formatted.starts_with('"') && formatted.ends_with('"') && formatted.len() >= 2 {
                formatted[1..formatted.len() - 1].to_string()
            } else {
                formatted
            }
        })
    }

    pub fn __repr__(&self) -> String {
        match &self.0 {
            OwnedKey::Int(i) => format!("OwnedKey::Int({})", i),
            OwnedKey::String(s) => format!(r"OwnedKey::String({})", s),
            OwnedKey::Symbol(s) => with_js_cx(|cx| {
                format!(
                    "OwnedKey::Symbol({})",
                    format_symbol(cx, Config::default(), s)
                )
            }),
            OwnedKey::Void => "OwnedKey::Void".to_string(),
        }
    }
}
