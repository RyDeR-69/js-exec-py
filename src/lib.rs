mod errors;
pub mod runtime;
mod traits;
mod types;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use pyo3_stub_gen::{define_stub_info_gatherer, module_variable};
use crate::runtime::with_js_cx;

/// # js-exec
///
/// A Python library that provides JavaScript execution capabilities from Python,
/// built on top of the spiderfire JavaScript Runtime.
///
/// This module allows you to execute JavaScript code from Python
/// and interact with JavaScript objects, functions, and values using a clean
/// and type-safe API.
///
/// ## Features
///
/// - Execute JavaScript code directly from Python
/// - TypeScript support out of the box (not fully implemented yet)
/// - Full access to JavaScript's type system including primitives and objects
/// - Support for JavaScript modules
/// - Safe handling of JavaScript values in Python
///
/// ## Basic Usage
///
/// ```python
/// from js_exec import Runtime, JSValue
///
/// # Create a JavaScript runtime
/// runtime = Runtime()
///
/// # Execute JavaScript code
/// result = runtime.compile_and_evaluate_script("1 + 2")
/// assert result.to_number() == 3.0
///
/// # Create and manipulate JavaScript objects
/// result = runtime.compile_and_evaluate_script("({x: 10, y: 20})")
/// obj = result.to_object()
/// x_value = obj.get("x")
/// assert x_value is not None and x_value.to_number() == 10.0
/// ```
///
/// ## Working with JavaScript Types
///
/// The module provides Python classes that map to JavaScript types:
///
/// - `JSValue`: Represents any JavaScript value (primitives, objects, functions)
/// - `JSObject`: Represents JavaScript objects
/// - `JSFunction`: Represents JavaScript functions
/// - `JSBigInt`: Represents JavaScript BigInt values
/// - `Symbol`: Represents JavaScript Symbols
///
/// ## Thread Safety
///
/// The JavaScript runtime is thread-local. Each thread must create its own Runtime
/// instance, and only one Runtime can exist per thread.
#[pymodule]
fn js_exec(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register functions
    m.add_function(wrap_pyfunction!(r#typeof, m)?)?;
    
    // Register classes
    m.add_class::<runtime::PythonJSRuntime>()?;
    m.add_class::<types::value::PyJSValue>()?;
    m.add_class::<types::object::PyJSObject>()?;
    m.add_class::<types::context::PyJSContext>()?;
    m.add_class::<types::bigint::PyJSBigInt>()?;
    m.add_class::<types::symbol::PySymbol>()?;
    m.add_class::<types::symbol::PyWellKnownSymbolCode>()?;
    m.add_class::<types::symbol::PySymbolCode>()?;
    m.add_class::<types::property_descriptor::PyPropertyDescriptor>()?;
    m.add_class::<types::property_flags::PyPropertyFlags>()?;
    m.add_class::<types::es_class::PyESClass>()?;
    m.add_class::<types::iterator_flags::PyIteratorFlags>()?;
    m.add_class::<types::owned_key::PyOwnedKey>()?;
    m.add_class::<types::function::PyJSFunction>()?;
    m.add_class::<types::property_key::PyPropertyKey>()?;
    m.add_class::<types::sourcemap::PySourceMap>()?;

    // Register custom error types
    // m.add("JSRuntimeError", py.get_type::<errors::JSRuntimeError>())?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#description
#[gen_stub_pyfunction(module = "js_exec.js_exec")]
#[pyfunction(name = "typeof")]
pub fn r#typeof(value: &types::value::PyJSValue) -> String {
    with_js_cx(|cx| {
        if value.is_undefined() {
            "undefined".to_string()
        } else if value.is_null() {
            "object".to_string()
        } else if value.is_boolean() {
            "boolean".to_string()
        } else if value.is_number() {
            "number".to_string()
        } else if value.is_bigint() {
            "bigint".to_string()
        } else if value.is_string() {
            "string".to_string()
        } else if value.is_symbol() {
            "symbol".to_string()
        } else if value.is_object() {
            let object = value.0.to_object(cx);
            let class = object.get_builtin_class(cx);
            match class {
                mozjs::jsapi::ESClass::Function => "function".to_string(),
                _ => "object".to_string(),
            }
        } else {
            "object".to_string()
        }
    })
}

module_variable!("js_exec.js_exec", "__version__", String);
define_stub_info_gatherer!(stub_info);
