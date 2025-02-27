import unittest
from js_exec import Runtime
from contextlib import contextmanager

class ErrorHandlingTests(unittest.TestCase):
    """Tests for error handling and exceptions in js_exec."""

    @classmethod
    def setUpClass(cls):
        # Create a single JavaScript runtime for all tests
        cls.runtime = Runtime()

    @contextmanager
    def assert_raises_js_error(self, expected_substring=None):
        """Context manager to assert that a JavaScript error is raised."""
        try:
            yield
            self.fail("Expected JavaScript error was not raised")
        except (RuntimeError, ValueError) as e:
            error_str = str(e)
            if expected_substring and expected_substring not in error_str:
                self.fail(f"Expected error containing '{expected_substring}', got: {error_str}")

    def test_syntax_error(self):
        """Test handling of JavaScript syntax errors."""
        with self.assert_raises_js_error("SyntaxError"):
            self.runtime.compile_and_evaluate_script("function() { return 1; }")

    def test_reference_error(self):
        """Test handling of JavaScript reference errors."""
        with self.assert_raises_js_error("ReferenceError"):
            self.runtime.compile_and_evaluate_script("undefinedVariable")

    def test_type_error(self):
        """Test handling of JavaScript type errors."""
        with self.assert_raises_js_error("TypeError"):
            self.runtime.compile_and_evaluate_script("let x = null; x.property")

    def test_range_error(self):
        """Test handling of JavaScript range errors."""
        with self.assert_raises_js_error("RangeError"):
            self.runtime.compile_and_evaluate_script("new Array(-1)")

    def test_error_in_function_call(self):
        """Test error handling when calling a JavaScript function that throws."""
        # Define a function that throws
        script = """
        function throwingFunction() {
            throw new Error('Deliberate error for testing');
        }
        throwingFunction;
        """
        result = self.runtime.compile_and_evaluate_script(script)
        func = result.to_object().to_function()

        # Call the function and expect an error
        with self.assert_raises_js_error("Deliberate error for testing"):
            func.call([])

    def test_bigint_creation_errors(self):
        """Test error handling for invalid BigInt creation."""
        from js_exec import JSBigInt

        # Try to create a BigInt from an invalid string
        with self.assert_raises_js_error():
            JSBigInt.string("not a number")

        # Try to create a BigInt from a non-integer float
        with self.assert_raises_js_error():
            JSBigInt.f64(3.14)

    def test_handling_try_catch(self):
        """Test that JavaScript try/catch blocks work as expected."""
        script = """
        function testTryCatch() {
            try {
                throw new Error('Test error');
                return 'This should not be reached';
            } catch (e) {
                return 'Caught: ' + e.message;
            }
        }
        testTryCatch();
        """
        result = self.runtime.compile_and_evaluate_script(script)
        self.assertEqual(str(result), "Caught: Test error")

    def test_error_stack_trace(self):
        """Test that JavaScript errors include stack traces."""
        script = """
        function level3() {
            throw new Error('Test error with stack');
        }
        function level2() {
            return level3();
        }
        function level1() {
            return level2();
        }
        level1();
        """

        with self.assert_raises_js_error("Test error with stack"):
            result = self.runtime.compile_and_evaluate_script(script)
            # Check that the error message contains the function names
            self.assertIn("level3", str(result))
            self.assertIn("level2", str(result))
            self.assertIn("level1", str(result))
            self.assertIn("inline.js", str(result))
            self.assertIn("Test error with stack", str(result))
            self.assertIn("Uncaught Error", str(result))

if __name__ == "__main__":
    unittest.main()