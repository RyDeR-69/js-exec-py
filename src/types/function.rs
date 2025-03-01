use crate::errors::ErrorHandling;
use crate::runtime::{with_js_cx, with_js_runtime};
use crate::traits::ExtendLifetime;
use crate::types::object::PyJSObject;
use crate::types::value::PyJSValue;
use ion::Function as JSFunction;
use ion::format::Config;
use ion::format::function::format_function;
use pyo3::{PyErr, PyResult, pyclass, pymethods};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// Represents a [JSFunction] within the JavaScript Runtime.
/// Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions) for more details.
#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSFunction", module = "js_exec.js_exec")]
pub struct PyJSFunction(JSFunction<'static>);

impl From<PyJSFunction> for JSFunction<'static> {
    fn from(value: PyJSFunction) -> Self {
        value.0
    }
}

impl From<JSFunction<'static>> for PyJSFunction {
    fn from(value: JSFunction<'static>) -> Self {
        PyJSFunction(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyJSFunction {
    /// Creates a new [JSFunction] from an object.
    /// Returns [None] if the object is not a function.
    #[staticmethod]
    pub fn from_object(obj: &PyJSObject) -> Option<Self> {
        with_js_cx(|cx| JSFunction::from_object(cx, &obj.0).map(|f| f.extend_lifetime()))
            .map(|v| v.into())
    }

    /// Converts the [JSFunction] into an [JSObject].
    pub fn to_object(&self) -> PyJSObject {
        with_js_cx(|cx| self.0.to_object(cx).extend_lifetime()).into()
    }

    /// Converts the [JSFunction] into a [String] in the form of its definition/source.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        with_js_cx(|cx| self.0.to_string(cx))
    }

    /// Returns the name of the function.
    /// # Warning
    /// This can cause Access Violation errors if the function is anonymous function.
    pub fn name(&self) -> PyResult<String> {
        with_js_cx(|cx| self.0.name(cx).to_value_err("Failed to get function name"))
    }

    /// Returns the display name of the function.
    /// Function display names are a non-standard feature.
    /// Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/displayName) for more details.
    pub fn display_name(&self) -> PyResult<String> {
        with_js_cx(|cx| {
            self.0
                .display_name(cx)
                .to_value_err("Failed to get display name")
        })
    }
    /// Returns the number of arguments of the function.
    pub fn nargs(&self) -> u16 {
        self.0.nargs()
    }
    /// Returns the length of the source of the function.
    pub fn length(&self) -> Option<u16> {
        with_js_cx(|cx| self.0.length(cx))
    }

    /// Calls the [JSFunction] with the given `this` [JSObject] and arguments.
    /// Returns the result of the [JSFunction] as a [JSValue].
    /// Returns [Err] if the function call fails or an exception occurs.
    #[pyo3(signature = (args=None, this = None))]
    pub fn call(
        &self,
        args: Option<Vec<PyJSValue>>,
        this: Option<PyJSObject>,
    ) -> PyResult<PyJSValue> {
        with_js_runtime(|rt| {
            let cx = rt.cx();
            let args = args.map(|v| v.into_iter().map(|a| a.0).collect::<Vec<_>>());
            let result = if let Some(this_obj) = this {
                // Use provided 'this'
                self.0.call(cx, &this_obj.0, args.as_deref().unwrap_or(&[]))
            } else {
                // Use global object as 'this'
                self.0.call(cx, rt.global(), args.as_deref().unwrap_or(&[]))
            }
            .to_runtime_err("Failed to call function")?;
            Ok::<_, PyErr>(result.extend_lifetime().into())
        })
    }
    /// Checks if the [JSFunction] is the built-in eval function.
    pub fn is_eval(&self) -> bool {
        self.0.is_eval()
    }

    /// Checks if the [JSFunction] is a constructor.
    pub fn is_constructor(&self) -> bool {
        self.0.is_constructor()
    }

    /// Checks if the [JSFunction] is the built-in function constructor.
    pub fn is_function_constructor(&self) -> bool {
        self.0.is_function_constructor()
    }

    // handle methods
    pub fn is_null(&self) -> bool {
        self.0.handle().is_null()
    }

    pub fn is_aligned(&self) -> bool {
        self.0.handle().is_aligned()
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_function(cx, Config::default(), &self.0).to_string())
    }
}
