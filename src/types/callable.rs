use crate::types::value::PyJSValue;
use maplit::hashset;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use pyo3_stub_gen::{PyStubType, TypeInfo};

// Generic callable wrapper that encodes signature information in type parameters
#[allow(dead_code)] // for now
pub struct PyCallable<'py, Args, Ret> {
    pub inner: Bound<'py, PyAny>,
    _args: std::marker::PhantomData<Args>,
    _ret: std::marker::PhantomData<Ret>,
}

impl<'py, Args, Res> FromPyObject<'py> for PyCallable<'py, Args, Res> {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyCallable {
            inner: ob.clone(),
            _args: std::marker::PhantomData,
            _ret: std::marker::PhantomData,
        })
    }
}

impl PyStubType for PyCallable<'_, (), String> {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Callable[[], str]".to_string(),
            import: hashset! { "typing".into() },
        }
    }
}

impl PyStubType for PyCallable<'_, (), ()> {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Callable[[], []]".to_string(),
            import: hashset! { "typing".into() },
        }
    }
}

impl PyStubType for PyCallable<'_, PyJSValue, PyResult<PyJSValue>> {
    fn type_output() -> TypeInfo {
        TypeInfo {
            name: "typing.Callable[[JSValue], JSValue]".to_string(),
            import: hashset! { "typing".into() },
        }
    }
}
