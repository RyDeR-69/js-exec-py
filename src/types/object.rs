use crate::errors::ErrorHandling;
use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::es_class::PyESClass;
use crate::types::function::PyJSFunction;
use crate::types::iterator_flags::PyIteratorFlags;
use crate::types::owned_key::PyOwnedKey;
use crate::types::property_descriptor::PyPropertyDescriptor;
use crate::types::property_flags::PyPropertyFlags;
use crate::types::property_key::{PropertyKeyTypes, PyPropertyKey};
use crate::types::value::PyJSValue;
use ion::conversions::{FromValue, ToValue};
use ion::format::Config;
use ion::format::object::format_raw_object;
use ion::{Function as JSFunction, Object as JSObject};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::collections::HashMap;

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "JSObject", module = "js_exec.js_exec")]
#[derive(Debug)]
pub struct PyJSObject(pub JSObject<'static>);

impl From<PyJSObject> for JSObject<'static> {
    fn from(value: PyJSObject) -> Self {
        value.0
    }
}

impl From<JSObject<'static>> for PyJSObject {
    fn from(value: JSObject<'static>) -> Self {
        PyJSObject(value)
    }
}

impl<'py> FromPyObject<'py> for PyJSObject {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(js_val) = ob.downcast::<PyJSObject>() {
            with_js_cx(|cx| {
                let borrowed = js_val.borrow();
                let obj = JSObject::from_value(cx, &borrowed.0.as_value(cx), true, ())
                    .to_value_err("Failed to convert to JSObject")?;
                Ok(obj.extend_lifetime().into())
            })
        } else {
            Err(PyValueError::new_err("Failed to convert to JSObject"))
        }
    }
}

/// Represents an [JSObject] in the JS Runtime.
///
/// Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object) for more details.
#[allow(clippy::new_without_default)]
#[gen_stub_pymethods]
#[pymethods]
impl PyJSObject {
    /// Creates a plain empty [JSObject].
    #[new]
    pub fn new() -> Self {
        with_js_cx(|cx| JSObject::new(cx).extend_lifetime()).into()
    }

    /// Creates a `null` "JSObject".
    ///
    /// Most operations on this will result in an error, so be wary of where it is used.
    #[staticmethod]
    pub fn null() -> Self {
        with_js_cx(|cx| JSObject::null(cx).extend_lifetime()).into()
    }

    /// Returns the current global object or `null` if one has not been initialised yet.
    #[staticmethod]
    pub fn global_object() -> Self {
        with_js_cx(|cx| JSObject::global(cx).extend_lifetime()).into()
    }

    /// Checks if the [JSObject] has a value at the given key.
    pub fn has(&self, key: PropertyKeyTypes) -> bool {
        with_js_cx(|cx| self.0.has(cx, &key))
    }
    /// Checks if the [JSObject] has its own value at the given key.
    ///
    /// An object owns its properties if they are not inherited from a prototype.
    pub fn has_own(&self, key: PropertyKeyTypes) -> bool {
        with_js_cx(|cx| self.0.has_own(cx, &key))
    }

    /// Gets the [JSValue] at the given key of the [JSObject].
    ///
    /// Returns [None] if there is no value at the given key.
    pub fn get(&self, key: PropertyKeyTypes) -> PyResult<Option<PyJSValue>> {
        with_js_cx(|cx| {
            if self.is_null() {
                return Ok(None);
            }
            let value = self
                .0
                .get(cx, &key)
                .map(|opt_value| opt_value.map(|v| PyJSValue::from(v.extend_lifetime())))
                .to_value_err("Failed to get value")?;
            Ok(value)
        })
    }

    /// Gets the [JSFunction] at the given key of the [JSObject].
    /// Returns [None] if there is no value at the given key.
    pub fn get_function(&self, key: &str) -> PyResult<Option<PyJSFunction>> {
        with_js_cx(|cx| {
            let value = self.0.get(cx, key).to_value_err("Failed to get value")?;

            if let Some(value) = value {
                if !value.handle().is_object() {
                    return Ok(None);
                }
                let object = value.to_object(cx);
                let function = JSFunction::from_object(cx, &object);
                return Ok(function.map(|f| PyJSFunction::from(f.extend_lifetime())));
            }
            Ok(None)
        })
    }

    /// Gets the descriptor at the given key of the [JSObject].
    /// Returns [None] if the object does not contain the key.
    pub fn get_descriptor(&self, key: PropertyKeyTypes) -> PyResult<Option<PyPropertyDescriptor>> {
        with_js_cx(|cx| {
            self.0
                .get_descriptor(cx, &key)
                .map(|opt_desc| opt_desc.map(|d| PyPropertyDescriptor::from(d.extend_lifetime())))
                .to_value_err("Failed to get descriptor")
        })
    }

    /// Sets the [JSValue] at the given key of the [JSObject].
    ///
    /// Returns `false` if the property cannot be set.
    ///
    /// Perform the assignment `obj[id] = v`.
    ///
    /// This function performs non-strict assignment, so if the property is
    /// read-only, nothing happens and no error is thrown.
    pub fn set(&self, key: PropertyKeyTypes, value: &PyJSValue) -> bool {
        with_js_cx(|cx| self.0.set(cx, &key, &value.0))
    }

    /// Defines the [JSValue] at the given key of the [JSObject] with the given attributes.
    ///
    /// Returns `false` if the property cannot be defined.
    pub fn define(&self, key: PropertyKeyTypes, value: &PyJSValue, attrs: PyPropertyFlags) -> bool {
        with_js_cx(|cx| self.0.define(cx, &key, &value.0, attrs.0))
    }

    /// Deletes the [JSValue] at the given index.
    ///
    /// Returns `false` if the element cannot be deleted.
    pub fn delete(&self, key: PropertyKeyTypes) -> bool {
        with_js_cx(|cx| self.0.delete(cx, &key))
    }

    /// Gets the builtin class of the object as described in the ECMAScript specification.
    ///
    /// Returns [ESClass::Other] for other projects or proxies that cannot be unwrapped.
    pub fn get_builtin_class(&self) -> PyESClass {
        with_js_cx(|cx| self.0.get_builtin_class(cx).into())
    }

    /// Returns the builtin class of the object if it a wrapper around a primitive.
    ///
    /// The boxed types are `Boolean`, `Number`, `String` and `BigInt`
    pub fn is_boxed_primitive(&self) -> Option<PyESClass> {
        with_js_cx(|cx| self.0.is_boxed_primitive(cx).map(Into::into))
    }

    /// Unboxes primitive wrappers. See [Self::is_boxed_primitive] for details.
    pub fn unbox_primitive(&self) -> Option<PyJSValue> {
        with_js_cx(|cx| {
            self.0
                .unbox_primitive(cx)
                .map(|v| PyJSValue::from(v.extend_lifetime()))
        })
    }

    /// Returns a vector of [PropertyKey] in the [JSObject].
    /// Each key can be a [String], [Symbol] or integer.
    #[pyo3(signature = (flags=None))]
    pub fn keys(&self, flags: Option<PyIteratorFlags>) -> Vec<PyPropertyKey> {
        with_js_cx(|cx| {
            self.0
                .keys(cx, flags.map(Into::into))
                .map(|key| PyPropertyKey::from(key.extend_lifetime()))
                .collect()
        })
    }

    /// Returns a vector of [OwnedKey] in the [JSObject].
    #[pyo3(signature = (flags=None))]
    pub fn keys_owned(&self, flags: Option<PyIteratorFlags>) -> PyResult<Vec<PyOwnedKey>> {
        with_js_cx(|cx| {
            let keys = self
                .0
                .keys(cx, flags.map(Into::into))
                .map(|key| key.to_owned_key(cx).map(ExtendLifetime::extend_lifetime))
                .collect::<Result<Vec<_>, _>>()
                .to_value_err("Failed to get owned keys")?;
            Ok(keys.into_iter().map(Into::into).collect())
        })
    }

    #[pyo3(signature = (flags=None))]
    pub fn to_hashmap(
        &self,
        flags: Option<PyIteratorFlags>,
    ) -> PyResult<HashMap<PyOwnedKey, PyJSValue>> {
        with_js_cx(|cx| {
            self.0
                .to_hashmap(cx, flags.map(Into::into))
                .map(|hashmap| {
                    hashmap
                        .into_iter()
                        .map(|(key, value)| {
                            let owned_key = key.extend_lifetime();
                            let js_value = value.extend_lifetime();
                            (owned_key.into(), js_value.into())
                        })
                        .collect()
                })
                .to_value_err("HashMap conversion failed")
        })
    }

    /// Converts the [JSObject] to a [JSFunction] if possible.
    /// Returns [None] if the object is not a function.
    pub fn to_function(&self) -> Option<PyJSFunction> {
        with_js_cx(|cx| {
            if self.0.get_builtin_class(cx) == mozjs::jsapi::ESClass::Function {
                let function = JSFunction::from_object(cx, &self.0);
                return function.map(|f| PyJSFunction::from(f.extend_lifetime()));
            }
            None
        })
    }

    // handle methods
    pub fn is_null(&self) -> bool {
        self.0.handle().is_null()
    }

    pub fn is_aligned(&self) -> bool {
        self.0.handle().is_aligned()
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_raw_object(cx, Config::default(), &self.0).to_string())
    }

    /*pub fn __repr__(&self) -> String {
        format!("JSObject({:?})", self.__str__())
    }*/
}
