import unittest

from js_exec import Runtime, JSValue


class BasicJSExecutionTests(unittest.TestCase):
    """Tests for basic JavaScript execution functionality."""

    @classmethod
    def setUpClass(cls):
        cls.runtime = Runtime()

    def test_simple_expression(self):
        """Test evaluating a simple JavaScript expression."""
        result = self.runtime.compile_and_evaluate_script("2 + 3")
        self.assertTrue(result.is_number())
        self.assertEqual(result.to_number(), 5.0)

    def test_string_value(self):
        """Test evaluating a JavaScript string."""
        result = self.runtime.compile_and_evaluate_script("'Hello, World!'")
        self.assertTrue(result.is_string())
        self.assertEqual(str(result), 'Hello, World!')

    def test_boolean_value(self):
        """Test evaluating JavaScript boolean values."""
        result_true = self.runtime.compile_and_evaluate_script("true")
        result_false = self.runtime.compile_and_evaluate_script("false")

        self.assertTrue(result_true.is_boolean())
        self.assertTrue(result_false.is_boolean())
        self.assertEqual(result_true.to_boolean(), True)
        self.assertEqual(result_false.to_boolean(), False)

    def test_undefined_and_null(self):
        """Test evaluating undefined and null values."""
        result_undefined = self.runtime.compile_and_evaluate_script("undefined")
        result_null = self.runtime.compile_and_evaluate_script("null")

        self.assertTrue(result_undefined.is_undefined())
        self.assertTrue(result_null.is_null())

    def test_array_creation(self):
        """Test creating a JavaScript array."""
        result = self.runtime.compile_and_evaluate_script("[1, 2, 3, 4, 5]")
        self.assertTrue(result.is_object())

        obj = result.to_object()
        for i in range(5):
            value = obj.get(str(i))
            self.assertIsNotNone(value)
            self.assertEqual(value.to_number(), i + 1)

    def test_multiple_statements(self):
        """Test executing multiple JavaScript statements."""
        script = """
        let x = 10;
        let y = 20;
        x + y;
        """
        result = self.runtime.compile_and_evaluate_script(script)
        self.assertTrue(result.is_number())
        self.assertEqual(result.to_number(), 30.0)

    def test_js_value_creation(self):
        """Test creating JavaScript values from Python."""
        # Create values
        string_val = JSValue.string("Hello")
        number_val = JSValue.i32(42)
        bool_val = JSValue.bool(True)

        # Verify type and value
        self.assertTrue(string_val.is_string())
        self.assertTrue(number_val.is_number())
        self.assertTrue(bool_val.is_boolean())

        self.assertEqual(str(string_val), "Hello")
        self.assertEqual(number_val.to_number(), 42.0)
        self.assertEqual(bool_val.to_boolean(), True)


if __name__ == "__main__":
    unittest.main()
