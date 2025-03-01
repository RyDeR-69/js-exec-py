use crate::errors::ErrorHandling;
use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use ion::format::Config;
use ion::format::primitive::format_primitive;
use ion::{BigInt as JSBigInt, Value as JSValue};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSBigInt", module = "js_exec.js_exec")]
pub struct PyJSBigInt(pub JSBigInt<'static>);

impl From<PyJSBigInt> for JSBigInt<'static> {
    fn from(value: PyJSBigInt) -> Self {
        value.0
    }
}

impl From<JSBigInt<'static>> for PyJSBigInt {
    fn from(value: JSBigInt<'static>) -> Self {
        PyJSBigInt(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyJSBigInt {
    /// Creates a [JSBigInt] from a boolean.
    #[staticmethod]
    pub fn bool(value: bool) -> Self {
        with_js_cx(|cx| JSBigInt::from_bool(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSBigInt] from a 64-bit signed integer.
    #[staticmethod]
    pub fn i64(value: i64) -> Self {
        with_js_cx(|cx| JSBigInt::from_i64(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSBigInt] from a 64-bit unsigned integer.
    #[staticmethod]
    pub fn u64(value: u64) -> Self {
        with_js_cx(|cx| JSBigInt::from_u64(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSBigInt] from a double.
    /// Returns an error if `number` is `NaN`, `Infinity`, `-Infinity` or contains a fractional component.
    #[staticmethod]
    pub fn f64(value: f64) -> PyResult<Self> {
        with_js_cx(|cx| {
            Ok(Self::from(
                JSBigInt::from_f64(cx, value)
                    .to_value_err("Failed to create BigInt from f64")?
                    .extend_lifetime(),
            ))
        })
    }

    /// Creates a [JSBigInt] from a string.
    #[staticmethod]
    pub fn string(value: &str) -> PyResult<Self> {
        if let Err(err) = value.parse::<f64>() {
            return Err(pyo3::exceptions::PyValueError::new_err(err.to_string()));
        }
        with_js_cx(|cx| {
            Ok(Self::from(
                JSBigInt::from_string(cx, value)
                    .to_value_err("Failed to create BigInt from string")?
                    .extend_lifetime(),
            ))
        })
    }

    /// Converts a [JSBigInt] to a 64-bit signed integer if possible.
    pub fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    /// Converts a [JSBigInt] to a 64-bit unsigned integer if possible.
    pub fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    /// Converts a [JSBigInt] to a double.
    /// Returns `Infinity` or `-Infinity` if it does not fit in a double.
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64()
    }

    /// Converts a [JSBigInt] to a double if it fits in a double.
    pub fn fits_f64(&self) -> Option<f64> {
        self.0.fits_f64()
    }

    /// Converts a [JSBigInt] to a string.
    /// Returns `None` if the radix is not within the range (2..=36).
    pub fn to_string(&self, radix: u8) -> PyResult<Option<String>> {
        with_js_cx(|cx| {
            let result = self
                .0
                .to_string(cx, radix)
                .map(|s| s.to_owned(cx))
                .transpose()
                .to_value_err("Failed to convert BigInt to string")?;
            Ok(result)
        })
    }

    /// Checks if the [JSBigInt] is negative.
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    // handle methods
    pub fn is_null(&self) -> bool {
        self.0.handle().is_null()
    }

    pub fn is_aligned(&self) -> bool {
        self.0.handle().is_aligned()
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| {
            format_primitive(cx, Config::default(), &JSValue::bigint(cx, &self.0)).to_string()
        })
    }

    pub fn __repr__(&self) -> String {
        format!("JSBigInt({})", self.__str__())
    }
}
