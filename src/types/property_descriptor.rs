use crate::runtime::with_js_cx;
use crate::traits::ExtendLifetime;
use crate::types::object::PyJSObject;
use crate::types::property_flags::PyPropertyFlags;
use crate::types::value::PyJSValue;
use ion::PropertyDescriptor;
use ion::format::Config;
use ion::format::descriptor::format_descriptor;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "PropertyDescriptor", module = "js_exec.js_exec")]
pub struct PyPropertyDescriptor(PropertyDescriptor<'static>);

impl From<PyPropertyDescriptor> for PropertyDescriptor<'static> {
    fn from(value: PyPropertyDescriptor) -> Self {
        value.0
    }
}

impl From<PropertyDescriptor<'static>> for PyPropertyDescriptor {
    fn from(value: PropertyDescriptor<'static>) -> Self {
        PyPropertyDescriptor(value)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPropertyDescriptor {
    #[new]
    pub fn new(value: &PyJSValue, attrs: &PyPropertyFlags) -> Self {
        with_js_cx(|cx| {
            let desc = PropertyDescriptor::new(cx, &value.0, attrs.0);
            PyPropertyDescriptor(desc.extend_lifetime())
        })
    }

    #[staticmethod]
    pub fn empty() -> Self {
        with_js_cx(|cx| {
            let desc = PropertyDescriptor::empty(cx);
            PyPropertyDescriptor(desc.extend_lifetime())
        })
    }

    #[staticmethod]
    pub fn from_object(obj: &PyJSObject) -> Option<Self> {
        with_js_cx(|cx| {
            let desc = PropertyDescriptor::from_object(cx, &obj.0);
            desc.map(|d| PyPropertyDescriptor(d.extend_lifetime()))
        })
    }

    pub fn to_object(&self) -> Option<PyJSObject> {
        with_js_cx(|cx| {
            let obj = self.0.to_object(cx);
            obj.map(|o| PyJSObject::from(o.extend_lifetime()))
        })
    }

    pub fn is_configurable(&self) -> bool {
        self.0.is_configurable()
    }

    pub fn is_enumerable(&self) -> bool {
        self.0.is_enumerable()
    }

    pub fn is_writable(&self) -> bool {
        self.0.is_writable()
    }

    pub fn is_resolving(&self) -> bool {
        self.0.is_resolving()
    }

    pub fn value(&self) -> Option<PyJSValue> {
        with_js_cx(|cx| {
            let value = self.0.value(cx);
            value.map(|v| PyJSValue::from(v.extend_lifetime()))
        })
    }

    pub fn __str__(&self) -> String {
        with_js_cx(|cx| format_descriptor(cx, Config::default(), &self.0, None).to_string())
    }
}
