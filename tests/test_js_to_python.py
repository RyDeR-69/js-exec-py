import unittest

from js_exec import Runtime, JSValue, JSBigInt, Symbol, ESClass, WellKnownSymbolCode


class TypeConversionTests(unittest.TestCase):
    """Tests for conversions between JavaScript and Python types."""

    @classmethod
    def setUpClass(cls):
        # Create a single JavaScript runtime for all tests
        if Runtime.is_initialized():
            cls.runtime = Runtime.empty()
        else:
            cls.runtime = Runtime()

    def test_boolean_conversion(self):
        """Test conversion between JavaScript and Python booleans."""
        # Python to JavaScript
        js_true = JSValue.bool(True)
        js_false = JSValue.bool(False)

        self.assertTrue(js_true.is_boolean())
        self.assertTrue(js_false.is_boolean())

        # JavaScript to Python
        self.assertEqual(js_true.to_boolean(), True)
        self.assertEqual(js_false.to_boolean(), False)

        # Roundtrip via JavaScript execution
        result = self.runtime.compile_and_evaluate_script("Boolean(true)")
        self.assertEqual(result.to_boolean(), True)

    def test_number_conversion(self):
        """Test conversion between JavaScript and Python numbers."""
        # Integer
        js_int = JSValue.i32(42)
        self.assertTrue(js_int.is_int32())
        self.assertEqual(js_int.to_int32(), 42)

        # Float
        js_float = JSValue.f64(3.14159)
        self.assertTrue(js_float.is_double())
        self.assertAlmostEqual(js_float.to_double(), 3.14159)

        # Roundtrip via JavaScript execution
        result_int = self.runtime.compile_and_evaluate_script("42")
        result_float = self.runtime.compile_and_evaluate_script("3.14159")

        self.assertEqual(result_int.to_number(), 42.0)
        self.assertAlmostEqual(result_float.to_number(), 3.14159)

    def test_string_conversion(self):
        """Test conversion between JavaScript and Python strings."""
        # Python to JavaScript
        js_str = JSValue.string("Hello, World!")
        self.assertTrue(js_str.is_string())

        # JavaScript to Python (via string representation)
        self.assertEqual(str(js_str), "Hello, World!")

        # Roundtrip via JavaScript execution
        result = self.runtime.compile_and_evaluate_script("'Hello, World!'")
        self.assertEqual(str(result), "Hello, World!")

    def test_bigint_conversion(self):
        """Test conversion between JavaScript and Python BigInt values."""
        # Create a BigInt from various sources
        from_i64 = JSBigInt.i64(9223372036854775807)  # Max int64
        from_u64 = JSBigInt.u64(18446744073709551615)  # Max uint64
        from_string = JSBigInt.string("12345678901234567890")

        self.assertIsNotNone(from_i64)
        self.assertIsNotNone(from_u64)
        self.assertIsNotNone(from_string)

        # Convert back to Python types
        self.assertEqual(from_i64.to_i64(), 9223372036854775807)
        self.assertEqual(from_u64.to_u64(), 18446744073709551615)

        # Check string conversion
        decimal_str = from_string.to_string(10)
        self.assertEqual(decimal_str, "12345678901234567890")

        # Create a JSValue from BigInt and verify
        js_bigint_value = JSValue.bigint(from_i64)
        self.assertTrue(js_bigint_value.is_bigint())

        # Roundtrip via JavaScript execution
        result = self.runtime.compile_and_evaluate_script("BigInt('12345678901234567890')")
        self.assertTrue(result.is_bigint())

    def test_symbol_conversion(self):
        """Test JavaScript Symbol type."""
        # Create a Symbol
        symbol = Symbol("test_symbol")

        # Create a value from the Symbol
        js_symbol_value = JSValue.symbol(symbol)
        self.assertTrue(js_symbol_value.is_symbol())

        # Test well-known symbols
        iterator_symbol = Symbol.well_known(WellKnownSymbolCode.Iterator)

        # Check symbol properties
        self.assertEqual(symbol.description(), "test_symbol")
        self.assertIsNotNone(iterator_symbol.description())

    def test_es_class_detection(self):
        """Test detecting the ES class of objects."""
        # undefined
        value = self.runtime.compile_and_evaluate_script("({})")
        obj = value.to_object()
        self.assertEqual(obj.get_builtin_class(), ESClass.object())

        # Array
        array_result = self.runtime.compile_and_evaluate_script("[]")
        array = array_result.to_object()
        self.assertEqual(array.get_builtin_class(), ESClass.array())

        # Function
        func_result = self.runtime.compile_and_evaluate_script("(function func() {})")
        func = func_result.to_object()
        self.assertEqual(func.get_builtin_class(), ESClass.function())

        # Date
        date_result = self.runtime.compile_and_evaluate_script("new Date()")
        date = date_result.to_object()
        self.assertEqual(date.get_builtin_class(), ESClass.date())

    def test_value_boxing_unboxing(self):
        """Test boxing and unboxing of primitive values."""
        # Create boxed primitives
        boxed_string = self.runtime.compile_and_evaluate_script("new String('test')")
        boxed_number = self.runtime.compile_and_evaluate_script("new Number(42)")
        boxed_boolean = self.runtime.compile_and_evaluate_script("new Boolean(true)")

        # Verify they're objects with the right ES class
        self.assertTrue(boxed_string.is_object())
        self.assertTrue(boxed_number.is_object())
        self.assertTrue(boxed_boolean.is_object())

        string_obj = boxed_string.to_object()
        number_obj = boxed_number.to_object()
        boolean_obj = boxed_boolean.to_object()

        # Check ES class
        self.assertEqual(string_obj.get_builtin_class(), ESClass.string())
        self.assertEqual(number_obj.get_builtin_class(), ESClass.number())
        self.assertEqual(boolean_obj.get_builtin_class(), ESClass.boolean())

        # Check is_boxed_primitive
        self.assertEqual(string_obj.is_boxed_primitive(), ESClass.string())
        self.assertEqual(number_obj.is_boxed_primitive(), ESClass.number())
        self.assertEqual(boolean_obj.is_boxed_primitive(), ESClass.boolean())

        # Unbox primitives
        unboxed_string = string_obj.unbox_primitive()
        unboxed_number = number_obj.unbox_primitive()
        unboxed_boolean = boolean_obj.unbox_primitive()

        self.assertEqual(str(unboxed_string), "test")
        self.assertEqual(unboxed_number.to_number(), 42.0)
        self.assertEqual(unboxed_boolean.to_boolean(), True)

if __name__ == "__main__":
    unittest.main()