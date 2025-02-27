# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401

import builtins
import typing
from enum import Enum, auto

__version__: builtins.str

class ESClass:
    @staticmethod
    def object() -> ESClass: ...
    @staticmethod
    def array() -> ESClass: ...
    @staticmethod
    def number() -> ESClass: ...
    @staticmethod
    def string() -> ESClass: ...
    @staticmethod
    def boolean() -> ESClass: ...
    @staticmethod
    def regexp() -> ESClass: ...
    @staticmethod
    def array_buffer() -> ESClass: ...
    @staticmethod
    def shared_array_buffer() -> ESClass: ...
    @staticmethod
    def date() -> ESClass: ...
    @staticmethod
    def set() -> ESClass: ...
    @staticmethod
    def map() -> ESClass: ...
    @staticmethod
    def promise() -> ESClass: ...
    @staticmethod
    def map_iterator() -> ESClass: ...
    @staticmethod
    def set_iterator() -> ESClass: ...
    @staticmethod
    def arguments() -> ESClass: ...
    @staticmethod
    def error() -> ESClass: ...
    @staticmethod
    def bigint() -> ESClass: ...
    @staticmethod
    def function() -> ESClass: ...
    @staticmethod
    def other() -> ESClass: ...
    def __repr__(self) -> builtins.str: ...

class IteratorFlags:
    @staticmethod
    def private() -> IteratorFlags:
        r"""
        Allows iterating over private properties.
        """
        ...

    @staticmethod
    def own_only() -> IteratorFlags:
        r"""
        Disallows iterating over inherited properties.
        """
        ...

    @staticmethod
    def hidden() -> IteratorFlags:
        r"""
        Allows iteration over non-enumerable properties.
        """
        ...

    @staticmethod
    def symbols() -> IteratorFlags:
        r"""
        Allows iteration over symbol keys.
        """
        ...

    @staticmethod
    def symbols_only() -> IteratorFlags:
        r"""
        Disallows iteration over string keys.
        """
        ...

    @staticmethod
    def for_await_of() -> IteratorFlags:
        r"""
        Iteration over async iterable objects and async generators.
        """
        ...

    @staticmethod
    def empty() -> IteratorFlags: ...
    @staticmethod
    def all() -> IteratorFlags: ...
    def bits(self) -> builtins.int: ...
    @staticmethod
    def from_bits(bits: builtins.int) -> typing.Optional[IteratorFlags]: ...
    @staticmethod
    def from_bits_truncate(bits: builtins.int) -> IteratorFlags: ...
    @staticmethod
    def from_bits_retain(bits: builtins.int) -> IteratorFlags: ...
    def is_empty(self) -> builtins.bool: ...
    def is_all(self) -> builtins.bool: ...
    def intersects(self, other: IteratorFlags) -> builtins.bool: ...
    def contains(self, other: IteratorFlags) -> builtins.bool: ...
    def insert(self, other: IteratorFlags) -> None: ...
    def remove(self, other: IteratorFlags) -> None: ...
    def toggle(self, other: IteratorFlags) -> None: ...
    def set(self, other: IteratorFlags, value: builtins.bool) -> None: ...
    def __repr__(self) -> builtins.str: ...

class JSBigInt:
    @staticmethod
    def bool(value: builtins.bool) -> JSBigInt:
        r"""
        Creates a [JSBigInt] from a boolean.
        """
        ...

    @staticmethod
    def i64(value: builtins.int) -> JSBigInt:
        r"""
        Creates a [JSBigInt] from a 64-bit signed integer.
        """
        ...

    @staticmethod
    def u64(value: builtins.int) -> JSBigInt:
        r"""
        Creates a [JSBigInt] from a 64-bit unsigned integer.
        """
        ...

    @staticmethod
    def f64(value: builtins.float) -> JSBigInt:
        r"""
        Creates a [JSBigInt] from a double.
        Returns an error if `number` is `NaN`, `Infinity`, `-Infinity` or contains a fractional component.
        """
        ...

    @staticmethod
    def string(value: builtins.str) -> JSBigInt:
        r"""
        Creates a [JSBigInt] from a string.
        """
        ...

    def to_i64(self) -> typing.Optional[builtins.int]:
        r"""
        Converts a [JSBigInt] to a 64-bit signed integer if possible.
        """
        ...

    def to_u64(self) -> typing.Optional[builtins.int]:
        r"""
        Converts a [JSBigInt] to a 64-bit unsigned integer if possible.
        """
        ...

    def to_f64(self) -> builtins.float:
        r"""
        Converts a [JSBigInt] to a double.
        Returns `Infinity` or `-Infinity` if it does not fit in a double.
        """
        ...

    def fits_f64(self) -> typing.Optional[builtins.float]:
        r"""
        Converts a [JSBigInt] to a double if it fits in a double.
        """
        ...

    def to_string(self, radix: builtins.int) -> typing.Optional[builtins.str]:
        r"""
        Converts a [JSBigInt] to a string.
        Returns `None` if the radix is not within the range (2..=36).
        """
        ...

    def is_negative(self) -> builtins.bool:
        r"""
        Checks if the [JSBigInt] is negative.
        """
        ...

    def is_null(self) -> builtins.bool: ...
    def is_aligned(self) -> builtins.bool: ...
    def __str__(self) -> builtins.str: ...
    def __repr__(self) -> builtins.str: ...

class JSContext: ...

class JSFunction:
    r"""
    Represents a [JSFunction] within the JavaScript Runtime.
    Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions) for more details.
    """
    @staticmethod
    def from_object(obj: JSObject) -> typing.Optional[JSFunction]:
        r"""
        Creates a new [JSFunction] from an object.
        Returns [None] if the object is not a function.
        """
        ...

    def to_object(self) -> JSObject:
        r"""
        Converts the [JSFunction] into an [JSObject].
        """
        ...

    def to_string(self) -> builtins.str:
        r"""
        Converts the [JSFunction] into a [String] in the form of its definition/source.
        """
        ...

    def name(self) -> builtins.str:
        r"""
        Returns the name of the function.
        # Warning
        This can cause Access Violation errors if the function is anonymous function.
        """
        ...

    def display_name(self) -> builtins.str:
        r"""
        Returns the display name of the function.
        Function display names are a non-standard feature.
        Refer to [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/displayName) for more details.
        """
        ...

    def nargs(self) -> builtins.int:
        r"""
        Returns the number of arguments of the function.
        """
        ...

    def length(self) -> typing.Optional[builtins.int]:
        r"""
        Returns the length of the source of the function.
        """
        ...

    def call(self, args: typing.Sequence[JSValue], this: typing.Optional[JSObject] = None) -> JSValue:
        r"""
        Calls the [JSFunction] with the given `this` [JSObject] and arguments.
        Returns the result of the [JSFunction] as a [JSValue].
        Returns [Err] if the function call fails or an exception occurs.
        """
        ...

    def is_eval(self) -> builtins.bool:
        r"""
        Checks if the [JSFunction] is the built-in eval function.
        """
        ...

    def is_constructor(self) -> builtins.bool:
        r"""
        Checks if the [JSFunction] is a constructor.
        """
        ...

    def is_function_constructor(self) -> builtins.bool:
        r"""
        Checks if the [JSFunction] is the built-in function constructor.
        """
        ...

    def is_null(self) -> builtins.bool: ...
    def is_aligned(self) -> builtins.bool: ...
    def __str__(self) -> builtins.str: ...

class JSObject:
    def __init__(self) -> JSObject: ...
    @staticmethod
    def null() -> JSObject:
        r"""
        Creates a `null` "JSObject".

        Most operations on this will result in an error, so be wary of where it is used.
        """
        ...

    @staticmethod
    def global_object() -> JSObject:
        r"""
        Returns the current global object or `null` if one has not been initialised yet.
        """
        ...

    def has(self, key: typing.Union[builtins.str, PropertyKey]) -> builtins.bool:
        r"""
        Checks if the [JSObject] has a value at the given key.
        """
        ...

    def has_own(self, key: typing.Union[builtins.str, PropertyKey]) -> builtins.bool:
        r"""
        Checks if the [JSObject] has its own value at the given key.

        An object owns its properties if they are not inherited from a prototype.
        """
        ...

    def get(self, key: typing.Union[builtins.str, PropertyKey]) -> typing.Optional[JSValue]:
        r"""
        Gets the [JSValue] at the given key of the [JSObject].

        Returns [None] if there is no value at the given key.
        """
        ...

    def get_function(self, key: builtins.str) -> typing.Optional[JSFunction]:
        r"""
        Gets the [JSFunction] at the given key of the [JSObject].
        Returns [None] if there is no value at the given key.
        """
        ...

    def get_descriptor(self, key: typing.Union[builtins.str, PropertyKey]) -> typing.Optional[PropertyDescriptor]:
        r"""
        Gets the descriptor at the given key of the [JSObject].
        Returns [None] if the object does not contain the key.
        """
        ...

    def set(self, key: typing.Union[builtins.str, PropertyKey], value: JSValue) -> builtins.bool:
        r"""
        Sets the [JSValue] at the given key of the [JSObject].

        Returns `false` if the property cannot be set.

        Perform the assignment `obj[id] = v`.

        This function performs non-strict assignment, so if the property is
        read-only, nothing happens and no error is thrown.
        """
        ...

    def define(
        self, key: typing.Union[builtins.str, PropertyKey], value: JSValue, attrs: PropertyFlags
    ) -> builtins.bool:
        r"""
        Defines the [JSValue] at the given key of the [JSObject] with the given attributes.

        Returns `false` if the property cannot be defined.
        """
        ...

    def delete(self, key: typing.Union[builtins.str, PropertyKey]) -> builtins.bool:
        r"""
        Deletes the [JSValue] at the given index.

        Returns `false` if the element cannot be deleted.
        """
        ...

    def get_builtin_class(self) -> ESClass:
        r"""
        Gets the builtin class of the object as described in the ECMAScript specification.

        Returns [ESClass::Other] for other projects or proxies that cannot be unwrapped.
        """
        ...

    def is_boxed_primitive(self) -> typing.Optional[ESClass]:
        r"""
        Returns the builtin class of the object if it a wrapper around a primitive.

        The boxed types are `Boolean`, `Number`, `String` and `BigInt`
        """
        ...

    def unbox_primitive(self) -> typing.Optional[JSValue]:
        r"""
        Unboxes primitive wrappers. See [Self::is_boxed_primitive] for details.
        """
        ...

    def keys(self, flags: typing.Optional[IteratorFlags] = None) -> builtins.list[PropertyKey]:
        r"""
        Returns a vector of [PropertyKey] in the [JSObject].
        Each key can be a [String], [Symbol] or integer.
        """
        ...

    def keys_owned(self, flags: typing.Optional[IteratorFlags] = None) -> builtins.list[OwnedKey]:
        r"""
        Returns a vector of [OwnedKey] in the [JSObject].
        """
        ...

    def to_hashmap(self, flags: typing.Optional[IteratorFlags] = None) -> builtins.dict[OwnedKey, JSValue]: ...
    def to_function(self) -> typing.Optional[JSFunction]:
        r"""
        Converts the [JSObject] to a [JSFunction] if possible.
        Returns [None] if the object is not a function.
        """
        ...

    def is_null(self) -> builtins.bool: ...
    def is_aligned(self) -> builtins.bool: ...
    def __str__(self) -> builtins.str: ...

class JSValue:
    r"""
    Represents a JavaScript value in the Python environment.

    This class wraps the underlying JavaScript value and provides methods to
    interact with it from Python. JavaScript values can be any of the following types:

    - Undefined
    - Null
    - Boolean
    - Number (integer or floating-point)
    - String
    - Symbol
    - Object (including Arrays and Functions)
    - BigInt

    # Examples

    Creating values:
    ```python
    # Create a string value
    string_val = JSValue.string("Hello, World")

    # Create a numeric value
    num_val = JSValue.i32(42)

    # Create a boolean value
    bool_val = JSValue.bool(True)

    # Create undefined and null
    undefined = JSValue.undefined()
    null_val = JSValue.null()
    ```

    Type checking:
    ```python
    value = JSValue.string("Hello")
    if value.is_string():
        print("Value is a string")
    elif value.is_number():
        print("Value is a number")
    elif value.is_object():
        print("Value is an object")
    ```

    Converting to Python types:
    ```python
    # Get a boolean value
    if value.is_boolean():
        python_bool = value.to_boolean()

    # Get a numeric value
    if value.is_int32():
        python_int = value.to_int32()
    elif value.is_double():
        python_float = value.to_double()
    ```
    """
    def to_number(self) -> typing.Optional[builtins.float]:
        r"""
        Converts to a number if the value is a number.
        """
        ...

    def to_double(self) -> typing.Optional[builtins.float]:
        r"""
        Converts to a double if the value is a double.
        """
        ...

    def to_int32(self) -> typing.Optional[builtins.int]:
        r"""
        Converts to an integer if the value is a 32-bit integer.
        """
        ...

    def to_boolean(self) -> typing.Optional[builtins.bool]:
        r"""
        Converts to a boolean if the value is a boolean.
        """
        ...

    def is_markable(self) -> builtins.bool:
        r"""
        Checks if the value is markable by the garbage collector.
        """
        ...

    def is_gcthing(self) -> builtins.bool:
        r"""
        Checks if the value is a garbage collected thing.
        """
        ...

    def is_bigint(self) -> builtins.bool:
        r"""
        Checks if the value is a BigInt.
        """
        ...

    def is_symbol(self) -> builtins.bool:
        r"""
        Checks if the value is a symbol.
        """
        ...

    def is_magic(self) -> builtins.bool:
        r"""
        Checks if the value is a magic value.
        """
        ...

    def is_object_or_null(self) -> builtins.bool:
        r"""
        Checks if the value is an object or null.
        """
        ...

    def is_object(self) -> builtins.bool:
        r"""
        Checks if the value is an object.
        """
        ...

    def is_string(self) -> builtins.bool:
        r"""
        Checks if the value is a string.
        """
        ...

    def is_primitive(self) -> builtins.bool:
        r"""
        Checks if the value is a primitive.
        """
        ...

    def is_number(self) -> builtins.bool:
        r"""
        Checks if the value is a number.
        """
        ...

    def is_double(self) -> builtins.bool:
        r"""
        Checks if the value is a double.
        """
        ...

    def is_int32(self) -> builtins.bool:
        r"""
        Checks if the value is a 32-bit integer.
        """
        ...

    def is_boolean(self) -> builtins.bool:
        r"""
        Checks if the value is a boolean.
        """
        ...

    def is_null_or_undefined(self) -> builtins.bool:
        r"""
        Checks if the value is null or undefined.
        """
        ...

    def is_null(self) -> builtins.bool:
        r"""
        Checks if the value is null.
        """
        ...

    def is_undefined(self) -> builtins.bool:
        r"""
        Checks if the value is undefined.
        """
        ...

    @staticmethod
    def bool(value: builtins.bool) -> JSValue:
        r"""
        Creates a [JSValue] from a boolean.
        """
        ...

    @staticmethod
    def i32(value: builtins.int) -> JSValue:
        r"""
        Creates a [JSValue] from a 32-bit signed integer.
        """
        ...

    @staticmethod
    def u32(value: builtins.int) -> JSValue:
        r"""
        Creates a [JSValue] from a 32-bit unsigned integer.
        """
        ...

    @staticmethod
    def f64(value: builtins.float) -> JSValue:
        r"""
        Creates a [JSValue] from a 64-bit float.
        """
        ...

    @staticmethod
    def string(value: builtins.str) -> JSValue:
        r"""
        Creates a [JSValue] from a string.
        """
        ...

    @staticmethod
    def bigint(value: JSBigInt) -> JSValue:
        r"""
        Creates a [JSValue] from a [JSBigInt].
        """
        ...

    @staticmethod
    def symbol(value: Symbol) -> JSValue:
        r"""
        Creates a [JSValue] from a [Symbol].
        """
        ...

    def to_object(self) -> JSObject:
        r"""
        Converts a [JSValue] to an [JSObject].
        """
        ...

    def is_same(self, other: JSValue) -> builtins.bool:
        r"""
        Compares two values for equality using the [SameValue algorithm](https://tc39.es/ecma262/multipage/abstract-operations.html#sec-samevalue).
        This is identical to strict equality (===), except that NaN's are equal and 0 !== -0.
        """
        ...

    def to_source(self) -> builtins.str:
        r"""
        Converts a [JSValue] to a string.
        """
        ...

    @staticmethod
    def undefined() -> JSValue:
        r"""
        Creates an `undefined` [JSValue].
        """
        ...

    @staticmethod
    def null() -> JSValue:
        r"""
        Creates a `null` [JSValue].
        """
        ...

    def debug_info(self) -> builtins.str:
        r"""
        basically "typeof"
        """
        ...

    def __eq__(self, other: JSValue) -> builtins.bool: ...
    def __ne__(self, other: JSValue) -> builtins.bool: ...
    def __str__(self) -> builtins.str: ...
    def __repr__(self) -> builtins.str: ...

class OwnedKey:
    @staticmethod
    def int(value: builtins.int) -> OwnedKey:
        r"""
        Creates a new OwnedKey representing an integer value
        """
        ...

    @staticmethod
    def string(value: builtins.str) -> OwnedKey:
        r"""
        Creates a new OwnedKey representing a string value
        """
        ...

    @staticmethod
    def symbol(symbol: Symbol) -> OwnedKey:
        r"""
        Creates a new OwnedKey representing a symbol
        """
        ...

    @staticmethod
    def void() -> OwnedKey:
        r"""
        Creates a new void OwnedKey
        """
        ...

    def __str__(self) -> builtins.str: ...
    def __repr__(self) -> builtins.str: ...

class PropertyDescriptor:
    def __init__(self, value: JSValue, attrs: PropertyFlags) -> PropertyDescriptor: ...
    @staticmethod
    def empty() -> PropertyDescriptor: ...
    @staticmethod
    def from_object(obj: JSObject) -> typing.Optional[PropertyDescriptor]: ...
    def to_object(self) -> typing.Optional[JSObject]: ...
    def is_configurable(self) -> builtins.bool: ...
    def is_enumerable(self) -> builtins.bool: ...
    def is_writable(self) -> builtins.bool: ...
    def is_resolving(self) -> builtins.bool: ...
    def value(self) -> typing.Optional[JSValue]: ...
    def __str__(self) -> builtins.str: ...

class PropertyFlags:
    @staticmethod
    def enumerate() -> PropertyFlags:
        r"""
        Allows enumeration through `Object.keys()`, `for...in` and other functions.
        See [Enumerability of Properties](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Enumerability_and_ownership_of_properties#traversing_object_properties).
        """
        ...

    @staticmethod
    def read_only() -> PropertyFlags:
        r"""
        Prevents reassignment of the property.
        """
        ...

    @staticmethod
    def permanent() -> PropertyFlags:
        r"""
        Prevents deletion and attribute modification of the property.
        """
        ...

    @staticmethod
    def resolving() -> PropertyFlags: ...
    @staticmethod
    def constant() -> PropertyFlags: ...
    @staticmethod
    def constant_enumerated() -> PropertyFlags: ...
    @staticmethod
    def empty() -> PropertyFlags: ...
    @staticmethod
    def all() -> PropertyFlags: ...
    def bits(self) -> builtins.int: ...
    @staticmethod
    def from_bits(bits: builtins.int) -> typing.Optional[PropertyFlags]: ...
    @staticmethod
    def from_bits_truncate(bits: builtins.int) -> PropertyFlags: ...
    @staticmethod
    def from_bits_retain(bits: builtins.int) -> PropertyFlags: ...
    def is_empty(self) -> builtins.bool: ...
    def is_all(self) -> builtins.bool: ...
    def intersects(self, other: PropertyFlags) -> builtins.bool: ...
    def contains(self, other: PropertyFlags) -> builtins.bool: ...
    def insert(self, other: PropertyFlags) -> None: ...
    def remove(self, other: PropertyFlags) -> None: ...
    def toggle(self, other: PropertyFlags) -> None: ...
    def set(self, other: PropertyFlags, value: builtins.bool) -> None: ...
    def __repr__(self) -> builtins.str: ...

class PropertyKey:
    @staticmethod
    def with_int(value: builtins.int) -> PropertyKey:
        r"""
        Creates a [PropertyKey] from an integer.
        """
        ...

    @staticmethod
    def with_string(value: builtins.str) -> typing.Optional[PropertyKey]:
        r"""
        Creates a [PropertyKey] from a string.
        """
        ...

    @staticmethod
    def with_symbol(symbol: Symbol) -> PropertyKey: ...
    @staticmethod
    def from_value(value: JSValue) -> typing.Optional[PropertyKey]: ...
    def to_owned_key(self) -> OwnedKey: ...
    def is_void(self) -> builtins.bool: ...
    def is_int(self) -> builtins.bool: ...
    def is_string(self) -> builtins.bool: ...
    def is_symbol(self) -> builtins.bool: ...
    def is_gcthing(self) -> builtins.bool:
        r"""
        Garbage Collected Thing
        """
        ...

    def __str__(self) -> builtins.str: ...
    def __repr__(self) -> builtins.str: ...

class Runtime:
    r"""
    Python wrapper for JavaScript runtime functionality.

    # Warning
    The runtime must be kept alive until the program ends.
    If the runtime is dropped and an attempt is made to use it or any
    JavaScript variables or objects created within it, an error will occur.
    """
    def __init__(
        self,
        microtask_queue: builtins.bool = False,
        macrotask_queue: builtins.bool = False,
        script: builtins.bool = False,
        typescript: builtins.bool = True,
        log_level: builtins.int = 0,
    ) -> Runtime: ...
    def compile_and_evaluate_script(self, source: builtins.str, filename: builtins.str = "inline.js") -> JSValue:
        r"""
        Compiles and evaluates JavaScript code.

        This method compiles the provided JavaScript code and executes it in the
        JavaScript runtime, returning the result as a JSValue.

        # Safety
        This method uses unsafe code to transmute the JavaScript value's lifetime to 'static.
        This is safe because:
        1. The JSRuntimeContext is stored in a thread_local and lives until the program ends
        2. The value returned by this function cannot outlive its creating context
        3. The runtime can only be dropped when the program ends
        4. JavaScript values are only valid within their creating context

        # Arguments
        * `source` - JavaScript source code to execute
        * `filename` - Name to use in error messages (defaults to "inline.js")

        # Returns
        The result of evaluating the JavaScript code as a JSValue

        # Errors
        Returns an error if the compilation or evaluation fails
        """
        ...

    def compile_and_evaluate_module(
        self,
        source: builtins.str,
        filename: builtins.str = "inline.js",
        path: typing.Optional[builtins.str] = "inline.js",
    ) -> tuple[builtins.bool, typing.Optional[JSValue]]:
        r"""
        TODO: Full support for modules
        """
        ...

    def compile_typescript(
        self, source: builtins.str, filename: builtins.str = "inline.js"
    ) -> tuple[builtins.str, SourceMap]:
        r"""
        TODO: Full support for typescript
        """
        ...

class SourceMap:
    def __repr__(self) -> builtins.str: ...

class Symbol:
    def __init__(self, description: builtins.str) -> Symbol: ...
    @staticmethod
    def for_key(key: builtins.str) -> Symbol:
        r"""
        Gets a [Symbol] from the symbol registry with the given key.
        """
        ...

    @staticmethod
    def well_known(code: WellKnownSymbolCode) -> Symbol:
        r"""
        Creates a well-known symbol with its corresponding code.
        """
        ...

    def description(self) -> typing.Optional[builtins.str]:
        r"""
        Returns the description of a [Symbol].
        Returns [None] for well-known symbols.
        """
        ...

    def code(self) -> SymbolCode:
        r"""
        Returns the identifying code of a [Symbol].
        """
        ...

    def is_null(self) -> builtins.bool: ...
    def is_aligned(self) -> builtins.bool: ...
    def __str__(self) -> builtins.str: ...
    def __repr__(self) -> builtins.str: ...

class SymbolCode:
    @staticmethod
    def well_known(code: WellKnownSymbolCode) -> SymbolCode:
        r"""
        Creates a [SymbolCode] from a [WellKnownSymbolCode].
        """
        ...

    @staticmethod
    def private_name() -> SymbolCode:
        r"""
        Creates a [SymbolCode] from a [PrivateNameSymbol].
        """
        ...

    @staticmethod
    def in_symbol_registry() -> SymbolCode:
        r"""
        Creates a [SymbolCode] from a [InSymbolRegistry].
        """
        ...

    @staticmethod
    def unique_symbol() -> SymbolCode:
        r"""
        Creates a [SymbolCode] from a [UniqueSymbol].
        """
        ...

    def get_well_known(self) -> typing.Optional[WellKnownSymbolCode]:
        r"""
        Returns the [WellKnownSymbolCode] of a [SymbolCode].
        """
        ...

    def __repr__(self) -> builtins.str: ...

class WellKnownSymbolCode(Enum):
    IsConcatSpreadable = auto()
    Iterator = auto()
    Match = auto()
    Replace = auto()
    Search = auto()
    Species = auto()
    HasInstance = auto()
    Split = auto()
    ToPrimitive = auto()
    ToStringTag = auto()
    Unscopables = auto()
    AsyncIterator = auto()
    MatchAll = auto()

def typeof(value: JSValue) -> builtins.str:
    r"""
    https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#description
    """
    ...
