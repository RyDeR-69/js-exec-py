use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;


/// Creates a PyValueError with a formatted message
///
/// # Arguments
/// * `msg` - The error message prefix
/// * `error` - The error to include in the message
pub fn value_error<E: std::fmt::Debug>(msg: &str, error: E) -> PyErr {
    PyValueError::new_err(format!("{}: {:?}", msg, error))
}

/// Creates a PyRuntimeError with a formatted message
///
/// # Arguments
/// * `msg` - The error message prefix
/// * `error` - The error to include in the message
pub fn runtime_error<E: std::fmt::Debug>(msg: &str, error: E) -> PyErr {
    PyRuntimeError::new_err(format!("{}: {:?}", msg, error))
}

// Extension trait for Results to make error handling more fluent
pub trait ErrorHandling<T, E> {
    /// Converts the error to a PyValueError with the specified message
    fn to_value_err(self, msg: &str) -> PyResult<T>;

    /// Converts the error to a PyRuntimeError with the specified message
    fn to_runtime_err(self, msg: &str) -> PyResult<T>;
}

impl<T, E: std::fmt::Debug> ErrorHandling<T, E> for Result<T, E> {
    fn to_value_err(self, msg: &str) -> PyResult<T> {
        self.map_err(|e| value_error(msg, e))
    }

    fn to_runtime_err(self, msg: &str) -> PyResult<T> {
        self.map_err(|e| runtime_error(msg, e))
    }
}
