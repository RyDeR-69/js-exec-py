use crate::errors::ErrorHandling;
use crate::traits::ExtendLifetime;
use crate::types::sourcemap::PySourceMap;
use crate::types::value::PyJSValue;
use ion::Context as JSContext;
use ion::script::Script;
use js_runtime::config::{CONFIG, Config, LogLevel};
use js_runtime::module::Loader;
use js_runtime::{Runtime as JSRuntime, RuntimeBuilder as JSRuntimeBuilder};
use modules::Modules;
use mozjs::rust::{JSEngine, Runtime as RustRuntime};
use ouroboros::self_referencing;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::cell::RefCell;

thread_local! {
    pub static JS_RUNTIME_CONTEXT: RefCell<Option<JSRuntimeContext>> = const { RefCell::new(None) };
}

/// Executes a function with access to the JavaScript runtime.
///
/// # Panics
///
/// Panics if the runtime has not been initialized for the current thread.
pub fn with_js_runtime<F, R>(f: F) -> R
where
    F: FnOnce(&mut JSRuntime) -> R,
{
    JS_RUNTIME_CONTEXT.with(|cell| {
        let mut maybe_runtime = cell.borrow_mut();
        if let Some(rt) = &mut *maybe_runtime {
            rt.with_js_runtime_mut(f)
        } else {
            panic!("JavaScript runtime not initialized for current thread");
        }
    })
}

/// Executes a function with access to the JavaScript context.
///
/// # Panics
///
/// Panics if the runtime has not been initialized for the current thread.
pub fn with_js_cx<F, R>(f: F) -> R
where
    F: FnOnce(&JSContext) -> R,
{
    with_js_runtime(|rt| {
        let cx = rt.cx();
        f(cx)
    })
}

/// Self-referential structure containing all components needed for JavaScript execution.
#[self_referencing]
pub struct JSRuntimeContext {
    engine: JSEngine,
    runtime: RustRuntime,
    pub(crate) js_context: JSContext,
    #[covariant]
    #[borrows(mut js_context)]
    pub js_runtime: JSRuntime<'this>,
}

/// Python wrapper for JavaScript runtime functionality.
///
/// # Warning
/// The runtime must be kept alive until the program ends.
/// If the runtime is dropped and an attempt is made to use it or any
/// JavaScript variables or objects created within it, an error will occur.
#[gen_stub_pyclass]
#[pyclass(name = "Runtime", module = "js_exec.js_exec")]
pub struct PythonJSRuntime;

#[gen_stub_pymethods]
#[pymethods]
impl PythonJSRuntime {
    /// Creates a new JavaScript runtime environment.
    ///
    /// This initializes a new JavaScript runtime for the current thread.
    /// Only one runtime can exist per thread.
    ///
    /// # Arguments
    /// * `microtask_queue` - Enable microtask queue for promises
    /// * `macrotask_queue` - Enable macrotask queue for setTimeout/setInterval
    /// * `script` - Enable support for JavaScript scripts
    /// * `typescript` - Enable support for TypeScript
    /// * `log_level` - Set the log level (0: None, 1: Info, 2: Warn, 3: Error, 4: Debug)
    ///
    /// # Returns
    /// A new PythonJSRuntime instance
    #[new]
    #[pyo3(signature = (microtask_queue = false, macrotask_queue = false, script = false, typescript = true, log_level = 0))]
    pub fn new(
        microtask_queue: bool,
        macrotask_queue: bool,
        script: bool,
        typescript: bool,
        log_level: u32,
    ) -> PyResult<Self> {
        JS_RUNTIME_CONTEXT.with(|cell| {
            let mut maybe_runtime = cell.borrow_mut();
            if let Some(_rt) = &*maybe_runtime {
                Err(PyRuntimeError::new_err(
                    "JavaScript runtime already initialized for this thread",
                ))
            } else {
                // First-time initialization for this thread.
                CONFIG.get_or_init(|| Config {
                    log_level: match log_level {
                        1 => LogLevel::Info,
                        2 => LogLevel::Warn,
                        3 => LogLevel::Error,
                        4 => LogLevel::Debug,
                        _ => LogLevel::None,
                    },
                    script,
                    typescript,
                });
                let engine = JSEngine::init().to_runtime_err("Failed to initialize JS engine")?;
                let engine_handle = engine.handle();
                let runtime = RustRuntime::new(engine_handle);
                if runtime.cx().is_null() {
                    return Err(PyRuntimeError::new_err("Failed to create JS runtime"));
                }
                let js_context = JSContext::from_runtime(&runtime);
                let mut rt_builder = JSRuntimeBuilder::<Loader, _>::new();
                if microtask_queue {
                    rt_builder = rt_builder.microtask_queue();
                }
                if macrotask_queue {
                    rt_builder = rt_builder.macrotask_queue();
                }

                let builder = JSRuntimeContextBuilder {
                    runtime,
                    engine,
                    js_context,
                    js_runtime_builder: |js_context: &mut JSContext| {
                        rt_builder
                            .modules(Loader::default())
                            .standard_modules(Modules)
                            .build(js_context)
                    },
                };
                let new_context = builder.build();
                *maybe_runtime = Some(new_context);
                Ok(())
            }
        })?;
        Ok(PythonJSRuntime)
    }

    /// Compiles and evaluates JavaScript code.
    ///
    /// This method compiles the provided JavaScript code and executes it in the
    /// JavaScript runtime, returning the result as a JSValue.
    ///
    /// # Safety
    /// This method uses unsafe code to transmute the JavaScript value's lifetime to 'static.
    /// This is safe because:
    /// 1. The JSRuntimeContext is stored in a thread_local and lives until the program ends
    /// 2. The value returned by this function cannot outlive its creating context
    /// 3. The runtime can only be dropped when the program ends
    /// 4. JavaScript values are only valid within their creating context
    ///
    /// # Arguments
    /// * `source` - JavaScript source code to execute
    /// * `filename` - Name to use in error messages (defaults to "inline.js")
    ///
    /// # Returns
    /// The result of evaluating the JavaScript code as a JSValue
    ///
    /// # Errors
    /// Returns an error if the compilation or evaluation fails
    #[pyo3(signature = (source, filename = "inline.js"))]
    pub fn compile_and_evaluate_script(&self, source: &str, filename: &str) -> PyResult<PyJSValue> {
        with_js_cx(|cx| {
            let result = Script::compile_and_evaluate(cx, filename.as_ref(), source)
                .map_err(|e| PyRuntimeError::new_err(e.format(cx)))?;

            Ok(PyJSValue::from(result.extend_lifetime()))
        })
    }

    /// TODO: Full support for modules
    #[pyo3(signature = (source, filename = "inline.js", path = Some("inline.js")))]
    pub fn compile_and_evaluate_module(
        &self,
        source: &str,
        filename: &str,
        path: Option<&str>,
    ) -> PyResult<(bool, Option<PyJSValue>)> {
        with_js_cx(|cx| {
            let (module, promise) = ion::module::Module::compile_and_evaluate(
                cx,
                filename,
                path.map(AsRef::as_ref),
                source,
            )
            .map_err(|e| PyRuntimeError::new_err(e.format(cx)))?;
            if let Some(promise) = promise {
                let result = promise.result(cx);
                return Ok((
                    module.is_linked(),
                    Some(PyJSValue::from(result.extend_lifetime())),
                ));
            }
            Ok((module.is_linked(), None))
        })
    }

    /// TODO: Full support for typescript
    #[pyo3(signature = (source, filename = "inline.js"))]
    pub fn compile_typescript(
        &self,
        source: &str,
        filename: &str,
    ) -> PyResult<(String, PySourceMap)> {
        let (compiled_js, sourcemap) = js_runtime::typescript::compile_typescript(filename, source)
            .to_runtime_err("Failed to compile TypeScript")?;
        Ok((compiled_js, PySourceMap::from(sourcemap)))
    }
}

impl Drop for PythonJSRuntime {
    fn drop(&mut self) {
        JS_RUNTIME_CONTEXT.with(|cell| {
            let mut context = cell.borrow_mut();
            *context = None;
        });
    }
}
