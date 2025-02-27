use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use ion::Symbol;
use ion::format::Config;
use ion::format::symbol::format_symbol;
use ion::symbol::{SymbolCode, WellKnownSymbolCode};
use macros::enum_original_mapping;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pymethods};
use std::fmt::Debug;

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "Symbol", module = "js_exec.js_exec")]
pub struct PySymbol(pub Symbol<'static>);

impl From<PySymbol> for Symbol<'static> {
    fn from(value: PySymbol) -> Self {
        value.0
    }
}

impl From<Symbol<'static>> for PySymbol {
    fn from(value: Symbol<'static>) -> Self {
        PySymbol(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySymbol {
    /// Creates a new unique Symbol with the given description.
    #[new]
    pub fn new(description: &str) -> Self {
        with_js_cx(|cx| Symbol::new(cx, description).extend_lifetime()).into()
    }

    /// Gets a [Symbol] from the symbol registry with the given key.
    #[staticmethod]
    pub fn for_key(key: &str) -> Self {
        with_js_cx(|cx| Symbol::for_key(cx, key).extend_lifetime()).into()
    }

    /// Creates a well-known symbol with its corresponding code.
    #[staticmethod]
    pub fn well_known(code: PyWellKnownSymbolCode) -> Self {
        with_js_cx(|cx| Symbol::well_known(cx, code.into()).extend_lifetime()).into()
    }

    /// Returns the description of a [Symbol].
    /// Returns [None] for well-known symbols.
    pub fn description(&self) -> Option<String> {
        with_js_cx(|cx| self.0.description(cx))
    }

    /// Returns the identifying code of a [Symbol].
    pub fn code(&self) -> PySymbolCode {
        self.0.code().into()
    }

    // handle methods
    pub fn is_null(&self) -> bool {
        self.0.handle().is_null()
    }

    pub fn is_aligned(&self) -> bool {
        self.0.handle().is_aligned()
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_symbol(cx, Config::default(), &self.0).to_string())
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[gen_stub_pyclass_enum]
#[enum_original_mapping(WellKnownSymbolCode)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[pyclass(
    unsendable,
    name = "WellKnownSymbolCode",
    module = "js_exec.js_exec",
    eq,
    eq_int
)]
pub enum PyWellKnownSymbolCode {
    IsConcatSpreadable,
    Iterator,
    Match,
    Replace,
    Search,
    Species,
    HasInstance,
    Split,
    ToPrimitive,
    ToStringTag,
    Unscopables,
    AsyncIterator,
    MatchAll,
}

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "SymbolCode", module = "js_exec.js_exec")]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct PySymbolCode(pub SymbolCode);

impl From<PySymbolCode> for SymbolCode {
    fn from(value: PySymbolCode) -> Self {
        value.0
    }
}
impl From<SymbolCode> for PySymbolCode {
    fn from(value: SymbolCode) -> Self {
        PySymbolCode(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PySymbolCode {
    /// Creates a [SymbolCode] from a [WellKnownSymbolCode].
    #[staticmethod]
    pub fn well_known(code: PyWellKnownSymbolCode) -> Self {
        PySymbolCode(SymbolCode::WellKnown(code.into()))
    }

    /// Creates a [SymbolCode] from a [PrivateNameSymbol].
    #[staticmethod]
    pub fn private_name() -> Self {
        PySymbolCode(SymbolCode::PrivateNameSymbol)
    }
    /// Creates a [SymbolCode] from a [InSymbolRegistry].
    #[staticmethod]
    pub fn in_symbol_registry() -> Self {
        PySymbolCode(SymbolCode::InSymbolRegistry)
    }
    /// Creates a [SymbolCode] from a [UniqueSymbol].
    #[staticmethod]
    pub fn unique_symbol() -> Self {
        PySymbolCode(SymbolCode::UniqueSymbol)
    }

    /// Returns the [WellKnownSymbolCode] of a [SymbolCode].
    pub fn get_well_known(&self) -> Option<PyWellKnownSymbolCode> {
        self.0.well_known().map(|code| code.into())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SymbolCode({:?})", self.0))
    }
}
