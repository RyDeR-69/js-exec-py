use crate::errors::ErrorHandling;
use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::r#typeof;
use crate::types::bigint::PyJSBigInt;
use crate::types::object::PyJSObject;
use crate::types::symbol::PySymbol;
use ion::Value as JSValue;
use ion::conversions::FromValue;
use ion::format::{Config, format_value};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

// Macro for generating type checking methods
macro_rules! define_is_methods {
    ($(($method:ident, $doc:expr)),* $(,)?) => {
        $(
            #[gen_stub_pymethods]
            #[pymethods]
            impl PyJSValue {
                #[doc = $doc]
                pub fn $method(&self) -> bool {
                    self.0.handle().$method()
                }
            }
        )*
    };
}

// Macro for generating type conversion methods
macro_rules! define_to_methods {
    ($(($method:ident, $check:ident, $type:ty, $doc:expr)),* $(,)?) => {
        $(
            #[gen_stub_pymethods]
            #[pymethods]
            impl PyJSValue {
                #[doc = $doc]
                pub fn $method(&self) -> Option<$type> {
                    if !self.$check() {
                        return None;
                    }
                    Some(self.0.handle().$method())
                }
            }
        )*
    };
}

/// Represents a JavaScript value in the Python environment.
///
/// This class wraps the underlying JavaScript value and provides methods to
/// interact with it from Python. JavaScript values can be any of the following types:
///
/// - Undefined
/// - Null
/// - Boolean
/// - Number (integer or floating-point)
/// - String
/// - Symbol
/// - Object (including Arrays and Functions)
/// - BigInt
///
/// # Examples
///
/// Creating values:
/// ```python
/// # Create a string value
/// string_val = JSValue.string("Hello, World")
///
/// # Create a numeric value
/// num_val = JSValue.i32(42)
///
/// # Create a boolean value
/// bool_val = JSValue.bool(True)
///
/// # Create undefined and null
/// undefined = JSValue.undefined()
/// null_val = JSValue.null()
/// ```
///
/// Type checking:
/// ```python
/// value = JSValue.string("Hello")
/// if value.is_string():
///     print("Value is a string")
/// elif value.is_number():
///     print("Value is a number")
/// elif value.is_object():
///     print("Value is an object")
/// ```
///
/// Converting to Python types:
/// ```python
/// # Get a boolean value
/// if value.is_boolean():
///     python_bool = value.to_boolean()
///
/// # Get a numeric value
/// if value.is_int32():
///     python_int = value.to_int32()
/// elif value.is_double():
///     python_float = value.to_double()
/// ```
#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSValue", module = "js_exec.js_exec")]
#[derive(Debug)]
pub struct PyJSValue(pub JSValue<'static>);

impl From<PyJSValue> for JSValue<'static> {
    fn from(value: PyJSValue) -> Self {
        value.0
    }
}

impl From<JSValue<'static>> for PyJSValue {
    fn from(value: JSValue<'static>) -> Self {
        PyJSValue(value)
    }
}

impl<'py> FromPyObject<'py> for PyJSValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(js_val) = ob.downcast::<PyJSValue>() {
            with_js_cx(|cx| {
                let borrowed = js_val.borrow();
                let value = JSValue::from_value(cx, &borrowed.0, true, ())
                    .to_value_err("Failed to convert to JSValue")?;
                Ok(value.extend_lifetime().into())
            })
        } else {
            Err(PyValueError::new_err("Failed to convert to JSValue"))
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyJSValue {
    /// Creates a [JSValue] from a boolean.
    #[staticmethod]
    pub fn bool(value: bool) -> Self {
        with_js_cx(|cx| JSValue::bool(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a 32-bit signed integer.
    #[staticmethod]
    pub fn i32(value: i32) -> Self {
        with_js_cx(|cx| JSValue::i32(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a 32-bit unsigned integer.
    #[staticmethod]
    pub fn u32(value: u32) -> Self {
        with_js_cx(|cx| JSValue::u32(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a 64-bit float.
    #[staticmethod]
    pub fn f64(value: f64) -> Self {
        with_js_cx(|cx| JSValue::f64(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a string.
    #[staticmethod]
    pub fn string(value: &str) -> Self {
        with_js_cx(|cx| JSValue::string(cx, value).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a [JSBigInt].
    #[staticmethod]
    pub fn bigint(value: &PyJSBigInt) -> Self {
        with_js_cx(|cx| JSValue::bigint(cx, &value.0).extend_lifetime()).into()
    }

    /// Creates a [JSValue] from a [Symbol].
    #[staticmethod]
    pub fn symbol(value: &PySymbol) -> Self {
        with_js_cx(|cx| JSValue::symbol(cx, &value.0).extend_lifetime()).into()
    }

    /// Converts a [JSValue] to an [JSObject].
    pub fn to_object(&self) -> PyResult<PyJSObject> {
        if !self.is_object() {
            return Err(PyValueError::new_err(
                "Failed to convert to JSObject, value is not an object",
            ));
        }
        Ok(with_js_cx(|cx| self.0.to_object(cx).extend_lifetime()).into())
    }
    /// Compares two values for equality using the [SameValue algorithm](https://tc39.es/ecma262/multipage/abstract-operations.html#sec-samevalue).
    /// This is identical to strict equality (===), except that NaN's are equal and 0 !== -0.
    pub fn is_same(&self, other: &Self) -> bool {
        with_js_cx(|cx| self.0.is_same(cx, &other.0))
    }

    /// Converts a [JSValue] to a string.
    pub fn to_source(&self) -> PyResult<String> {
        with_js_cx(|cx| {
            let source = self.0.to_source(cx);
            source
                .to_owned(cx)
                .to_value_err("Failed to convert to source")
        })
    }

    /// Creates an `undefined` [JSValue].
    #[staticmethod]
    pub fn undefined() -> Self {
        with_js_cx(|cx| JSValue::undefined(cx).extend_lifetime()).into()
    }

    /// Creates a `null` [JSValue].
    #[staticmethod]
    pub fn null() -> Self {
        with_js_cx(|cx| JSValue::null(cx).extend_lifetime()).into()
    }

    /// basically "typeof" 
    pub fn debug_info(&self) -> String {
        let type_str = r#typeof(self);
        with_js_cx(|cx| {
            let value_str = format_value(cx, Config::default(), &self.0).to_string();
            format!("JSValue<{}>({})", type_str, value_str)
        })
    }

    // magic methods
    pub fn __eq__(&self, other: &Self) -> bool {
        self.is_same(other)
    }
    pub fn __ne__(&self, other: &Self) -> bool {
        !self.is_same(other)
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_value(cx, Config::default(), &self.0).to_string())
    }

    pub fn __repr__(&self) -> String {
        self.debug_info()
    }
}

define_is_methods! {
    (is_undefined, "Checks if the value is undefined."),
    (is_null, "Checks if the value is null."),
    (is_null_or_undefined, "Checks if the value is null or undefined."),
    (is_boolean, "Checks if the value is a boolean."),
    (is_int32, "Checks if the value is a 32-bit integer."),
    (is_double, "Checks if the value is a double."),
    (is_number, "Checks if the value is a number."),
    (is_primitive, "Checks if the value is a primitive."),
    (is_string, "Checks if the value is a string."),
    (is_object, "Checks if the value is an object."),
    (is_object_or_null, "Checks if the value is an object or null."),
    (is_magic, "Checks if the value is a magic value."),
    (is_symbol, "Checks if the value is a symbol."),
    (is_bigint, "Checks if the value is a BigInt."),
    (is_gcthing, "Checks if the value is a garbage collected thing."),
    (is_markable, "Checks if the value is markable by the garbage collector."),
}

define_to_methods! {
    (to_boolean, is_boolean, bool, "Converts to a boolean if the value is a boolean."),
    (to_int32, is_int32, i32, "Converts to an integer if the value is a 32-bit integer."),
    (to_double, is_double, f64, "Converts to a double if the value is a double."),
    (to_number, is_number, f64, "Converts to a number if the value is a number."),
}
