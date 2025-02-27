# js-exec

A Python library that provides JavaScript execution capabilities from Python, built on top of the spiderfire JavaScript Runtime.

## Installation

Currently only building from source
```bash
pip install git+https://github.com/RyDeR-69/js-exec-py.git
```

## Quick Start

```python
from js_exec import Runtime

# Create a JavaScript runtime
runtime = Runtime()

# Execute JavaScript code
result = runtime.compile_and_evaluate_script("1 + 2")
assert result.to_number() == 3.0

# Create and manipulate JavaScript objects
result = runtime.compile_and_evaluate_script("({x: 10, y: 20})")
obj = result.to_object()
x_value = obj.get("x")
assert x_value is not None and x_value.to_number() == 10.0
```

## ⚠️ Important Runtime Considerations

Please note these critical aspects of the JavaScript runtime:

1. **Single Runtime Per Thread**: Only one runtime can exist per thread. Attempting to create multiple runtimes in the same thread will result in an error.

2. **Runtime Lifetime**: The runtime instance **must be kept alive** until the program ends. If the runtime is dropped and an attempt is made to use it or any JavaScript variables/objects created within it, undefined behavior will occur.

   ```python
   from js_exec import Runtime
   # Keep the runtime alive for the entire program execution
   _runtime = Runtime()
   
   # Store this in a global or long-lived variable to ensure it's not garbage collected
   ```

3. **JavaScript Values**: All JavaScript values (JSValue, JSObject, etc.) are only valid for use with the runtime that created them.

## Working with JavaScript Types

The module provides Python classes that map to JavaScript types:

- `JSValue`: Represents any JavaScript value (primitives, objects, functions)
- `JSObject`: Represents JavaScript objects
- `JSFunction`: Represents JavaScript functions
- `JSBigInt`: Represents JavaScript BigInt values
- `Symbol`: Represents JavaScript Symbols

## TypeScript Support

```python
# Compile TypeScript to JavaScript
from js_exec import Runtime

runtime = Runtime()

js_code, sourcemap = runtime.compile_typescript("let x: number = 10; x + 5;")

# Execute the compiled JavaScript
result = runtime.compile_and_evaluate_script(js_code)
```
