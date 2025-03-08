import unittest
from js_exec import Runtime, JSValue, JSObject, ESClass, JSFunction


class JSFunctionTests(unittest.TestCase):
    """Tests for JavaScript function handling."""

    @classmethod
    def setUpClass(cls):
        # Create a single JavaScript runtime for all tests
        if Runtime.is_initialized():
            cls.runtime = Runtime.empty()
        else:
            cls.runtime = Runtime()

    def test_function_creation_and_call(self):
        """Test creating and calling a JavaScript function."""
        # Define a simple function in JavaScript
        script = """
        function add(a, b) {
            return a + b;
        }
        add; // Return the function itself
        """
        result = self.runtime.compile_and_evaluate_script(script)
        self.assertTrue(result.is_object())
        self.assertEqual(result.to_object().get_builtin_class(), ESClass.function())

        # Convert to object and then get as function
        obj = result.to_object()
        func = JSFunction.from_object(obj)
        self.assertIsNotNone(func)

        # Call the function with arguments
        this_obj = JSObject()  # "this" context
        args = [JSValue.i32(5), JSValue.i32(7)]
        call_result = func.call(args, this_obj)
        self.assertTrue(call_result.is_number())
        
        call_result = func.call(args) # Default of "this" is global object
        self.assertTrue(call_result.is_number())
        self.assertEqual(call_result.to_number(), 12.0)

    def test_function_properties(self):
        """Test function properties like name, length, etc."""
        script = """
        function testFunction(a, b, c) {
            return a + b + c;
        }
        testFunction;
        """
        result = self.runtime.compile_and_evaluate_script(script)
        obj = result.to_object()
        func = obj.to_function()

        # Test function properties
        self.assertEqual(func.name(), "testFunction")
        self.assertEqual(func.nargs(), 3)  # Number of formal parameters
        self.assertIsNotNone(func.length())

    def test_method_calls(self):
        """Test calling methods on JavaScript objects."""
        script = """
        ({
            value: 10,
            double: function() {
                return this.value * 2;
            },
            add: function(x) {
                return this.value + x;
            }
        })
        """
        result = self.runtime.compile_and_evaluate_script(script)
        obj = result.to_object()

        # Get the methods
        double_func = obj.get_function("double")
        add_func = obj.get_function("add")

        self.assertIsNotNone(double_func)
        self.assertIsNotNone(add_func)

        # Call the methods
        double_result = double_func.call([], obj)
        add_result = add_func.call([JSValue.i32(5)], obj)

        self.assertEqual(double_result.to_number(), 20.0)
        self.assertEqual(add_result.to_number(), 15.0)

    def test_arrow_function(self):
        """Test arrow functions which have lexical 'this'."""
        script = """
        ({
            value: 10,
            regularFn: function() {
                return this.value;
            },
            arrowFn: () => this.value, // This 'this' refers to global object
            getArrowFn: function() {
                // Arrow function capturing 'this' from surrounding scope
                return () => this.value;
            }
        })
        """
        result = self.runtime.compile_and_evaluate_script(script)
        obj = result.to_object()

        # Regular function
        regular_fn = obj.get_function("regularFn")
        regular_result = regular_fn.call([], obj)
        self.assertEqual(regular_result.to_number(), 10.0)

        # Arrow function with this from enclosing scope
        get_arrow_fn = obj.get_function("getArrowFn")
        arrow_fn_value = get_arrow_fn.call([], obj)
        arrow_fn = arrow_fn_value.to_object()
        arrow_fn_func = arrow_fn.to_function()

        # Call the arrow function - it should remember 'this' as 'obj'
        arrow_result = arrow_fn_func.call([])
        self.assertEqual(arrow_result.to_number(), 10.0)

    def test_function_constructor_property(self):
        """Test function constructor property detection."""
        script = """
        function ConstructorFn() {
            this.value = 42;
        }
        ConstructorFn;
        """
        result = self.runtime.compile_and_evaluate_script(script)
        func = result.to_object().to_function()

        # Check if it's a constructor
        self.assertTrue(func.is_constructor())

        # Call as a constructor (simulate `new ConstructorFn()`)
        # Note: We'd typically use a factory method in JavaScript to do this properly
        script2 = """
        function ConstructorFn() {
            this.value = 42;
        }
        new ConstructorFn();
        """
        instance = self.runtime.compile_and_evaluate_script(script2)
        self.assertTrue(instance.is_object())

        instance_obj = instance.to_object()
        value = instance_obj.get("value")
        self.assertEqual(value.to_number(), 42.0)

    def test_function_source(self):
        """Test getting the source of a JavaScript function."""
        script = """
        function testFunction(a, b) {
            return a + b;
        }
        testFunction;
        """
        result = self.runtime.compile_and_evaluate_script(script)
        func = result.to_object().to_function()

        # Get the function source
        source = func.to_string()
        self.assertIn("function testFunction", source)
        self.assertIn("return a + b", source)


if __name__ == "__main__":
    unittest.main()
