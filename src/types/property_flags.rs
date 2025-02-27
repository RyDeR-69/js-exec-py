use ion::flags::PropertyFlags;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "PropertyFlags", module = "js_exec.js_exec")]
#[derive(Clone, Debug)]
pub struct PyPropertyFlags(pub PropertyFlags);

impl From<PyPropertyFlags> for PropertyFlags {
    fn from(value: PyPropertyFlags) -> Self {
        value.0
    }
}

impl From<PropertyFlags> for PyPropertyFlags {
    fn from(value: PropertyFlags) -> Self {
        PyPropertyFlags(value)
    }
}

/// Represents the flags of properties on an [JSObject]
#[gen_stub_pymethods]
#[pymethods]
impl PyPropertyFlags {
    /// Allows enumeration through `Object.keys()`, `for...in` and other functions.
    /// See [Enumerability of Properties](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Enumerability_and_ownership_of_properties#traversing_object_properties).
    #[staticmethod]
    pub fn enumerate() -> Self {
        Self(PropertyFlags::ENUMERATE)
    }

    /// Prevents reassignment of the property.
    #[staticmethod]
    pub fn read_only() -> Self {
        Self(PropertyFlags::READ_ONLY)
    }

    /// Prevents deletion and attribute modification of the property.
    #[staticmethod]
    pub fn permanent() -> Self {
        Self(PropertyFlags::PERMANENT)
    }

    #[staticmethod]
    pub fn resolving() -> Self {
        Self(PropertyFlags::RESOLVING)
    }

    #[staticmethod]
    pub fn constant() -> Self {
        Self(PropertyFlags::CONSTANT)
    }

    #[staticmethod]
    pub fn constant_enumerated() -> Self {
        Self(PropertyFlags::CONSTANT_ENUMERATED)
    }

    // bitflags methods
    #[inline]
    #[staticmethod]
    pub const fn empty() -> Self {
        Self(PropertyFlags::empty())
    }

    #[inline]
    #[staticmethod]
    pub const fn all() -> Self {
        Self(PropertyFlags::all())
    }

    #[inline]
    pub const fn bits(&self) -> u16 {
        self.0.bits()
    }

    #[inline]
    #[staticmethod]
    pub fn from_bits(bits: u16) -> Option<Self> {
        PropertyFlags::from_bits(bits).map(Self)
    }

    #[inline]
    #[staticmethod]
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self(PropertyFlags::from_bits_truncate(bits))
    }

    #[inline]
    #[staticmethod]
    pub const fn from_bits_retain(bits: u16) -> Self {
        Self(PropertyFlags::from_bits_retain(bits))
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0.is_all()
    }

    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        self.0.intersects(other.0)
    }

    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        self.0.contains(other.0)
    }

    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0.insert(other.0);
    }

    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0.remove(other.0);
    }

    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0.toggle(other.0);
    }

    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        self.0.set(other.0, value);
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
