use ion::flags::IteratorFlags;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[gen_stub_pyclass]
#[pyclass(unsendable, name = "IteratorFlags", module = "js_exec.js_exec")]
#[derive(Clone, Debug)]
pub struct PyIteratorFlags(pub IteratorFlags);

impl From<PyIteratorFlags> for IteratorFlags {
    fn from(value: PyIteratorFlags) -> Self {
        value.0
    }
}

impl From<IteratorFlags> for PyIteratorFlags {
    fn from(value: IteratorFlags) -> Self {
        PyIteratorFlags(value)
    }
}

/// Represents the flags when iterating over an [JSObject]
#[gen_stub_pymethods]
#[pymethods]
impl PyIteratorFlags {
    /// Allows iterating over private properties.
    #[staticmethod]
    pub fn private() -> Self {
        Self(IteratorFlags::PRIVATE)
    }

    /// Disallows iterating over inherited properties.
    #[staticmethod]
    pub fn own_only() -> Self {
        Self(IteratorFlags::OWN_ONLY)
    }

    /// Allows iteration over non-enumerable properties.
    #[staticmethod]
    pub fn hidden() -> Self {
        Self(IteratorFlags::HIDDEN)
    }

    /// Allows iteration over symbol keys.
    #[staticmethod]
    pub fn symbols() -> Self {
        Self(IteratorFlags::SYMBOLS)
    }

    /// Disallows iteration over string keys.
    #[staticmethod]
    pub fn symbols_only() -> Self {
        Self(IteratorFlags::SYMBOLS_ONLY)
    }

    /// Iteration over async iterable objects and async generators.
    #[staticmethod]
    pub fn for_await_of() -> Self {
        Self(IteratorFlags::FOR_AWAIT_OF)
    }

    // bitflags methods
    #[inline]
    #[staticmethod]
    pub const fn empty() -> Self {
        Self(IteratorFlags::empty())
    }

    #[inline]
    #[staticmethod]
    pub const fn all() -> Self {
        Self(IteratorFlags::all())
    }

    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0.bits()
    }

    #[inline]
    #[staticmethod]
    pub fn from_bits(bits: u32) -> Option<Self> {
        IteratorFlags::from_bits(bits).map(Self)
    }

    #[inline]
    #[staticmethod]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(IteratorFlags::from_bits_truncate(bits))
    }

    #[inline]
    #[staticmethod]
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(IteratorFlags::from_bits_retain(bits))
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
        format!("IteratorFlags({:?})", self.0)
    }
}
