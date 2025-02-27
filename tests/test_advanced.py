import unittest

from js_exec import Runtime, JSValue, JSObject, Symbol, PropertyKey, OwnedKey, IteratorFlags


class AdvancedFeaturesTests(unittest.TestCase):
    """Tests for more advanced features of js_exec."""

    @classmethod
    def setUpClass(cls):
        # Create a single JavaScript runtime for all tests
        cls.runtime = Runtime()

    def test_symbol_properties(self):
        """Test working with Symbol properties on objects."""
        # Create a symbol
        symbol = Symbol("testSymbol")
        assert OwnedKey.string("a") == OwnedKey.string("a")
        # Create an object and set a property with the symbol as key
        obj = JSObject()
        prop_key = PropertyKey.with_symbol(symbol)
        obj.set(prop_key, JSValue.string("Symbol value"))

        # Create a property key from the symbol
        prop_key = PropertyKey.with_symbol(symbol)

        # Convert to an owned key for hashmap lookup
        owned_key = prop_key.to_owned_key()
        assert owned_key == prop_key.to_owned_key()
        # Convert object to hashmap with symbol keys enabled
        flags = IteratorFlags.symbols()
        hashmap = obj.to_hashmap(flags)

        # Find our symbol value in the hashmap
        found_value = None
        for key, value in hashmap.items():
            if isinstance(key, OwnedKey) and key == owned_key:
                found_value = value
                break

        self.assertIsNotNone(found_value)
        self.assertEqual(str(found_value), "Symbol value")

    def test_well_known_symbols(self):
        """Test using well-known JavaScript symbols."""
        from js_exec import WellKnownSymbolCode

        # Create a well-known symbol (Symbol.iterator)
        iterator_symbol = Symbol.well_known(WellKnownSymbolCode.Iterator)

        # Execute a script that returns a value with an iterator
        result = self.runtime.compile_and_evaluate_script("[1, 2, 3]")
        array_obj = result.to_object()

        # Get the Symbol.iterator method
        symbol_value = JSValue.symbol(iterator_symbol)
        symbol_str = str(symbol_value)

        # Execute script to use the Symbol.iterator
        script = f"""
        const arr = [1, 2, 3];
        const iterator = arr[Symbol.iterator]();
        const results = [];
        
        let result = iterator.next();
        while (!result.done) {{
            results.push(result.value);
            result = iterator.next();
        }}
        
        results;
        """

        iterator_result = self.runtime.compile_and_evaluate_script(script)
        self.assertTrue(iterator_result.is_object())

        # Verify iterator results
        iter_obj = iterator_result.to_object()
        for i in range(3):
            value = iter_obj.get(str(i))
            self.assertEqual(value.to_number(), i + 1)

    def test_json_stringify_parse(self):
        """Test JSON.stringify and JSON.parse through JavaScript."""
        # Create a complex object
        complex_obj_script = """
        const obj = {
            string: "Hello",
            number: 42,
            boolean: true,
            null: null,
            array: [1, 2, 3],
            nested: {
                a: 1,
                b: 2
            }
        };
        
        // Stringify and then parse back
        let json = JSON.stringify(obj);
        let parsed = JSON.parse(json);
        
        // Expose objects
        globalThis.obj = obj;
        globalThis.json = json;
        globalThis.parsed = parsed;
        """

        self.runtime.compile_and_evaluate_script(complex_obj_script)
        obj = JSObject.global_object()

        # Get the JSON string and parsed object
        json_value = obj.get("json")
        # print(json_value)
        parsed_value = obj.get("parsed")

        self.assertTrue(json_value.is_string())
        self.assertTrue(parsed_value.is_object())

        # Verify the JSON string contains expected elements
        json_str = str(json_value)
        self.assertIn("Hello", json_str)
        self.assertIn("42", json_str)
        self.assertIn("true", json_str)
        self.assertIn("null", json_str)
        self.assertIn("[1,2,3]", json_str.replace(" ", ""))

        # Verify the parsed object has the same structure
        parsed_obj = parsed_value.to_object()
        self.assertEqual(str(parsed_obj.get("string")), "Hello")
        self.assertEqual(parsed_obj.get("number").to_number(), 42.0)
        self.assertEqual(parsed_obj.get("boolean").to_boolean(), True)
        self.assertTrue(parsed_obj.get("null").is_null())

        array = parsed_obj.get("array")
        self.assertTrue(array.is_object())
        array_obj = array.to_object()
        self.assertEqual(array_obj.get("0").to_number(), 1.0)
        self.assertEqual(array_obj.get("1").to_number(), 2.0)
        self.assertEqual(array_obj.get("2").to_number(), 3.0)

        nested = parsed_obj.get("nested")
        self.assertTrue(nested.is_object())
        nested_obj = nested.to_object()
        self.assertEqual(nested_obj.get("a").to_number(), 1.0)
        self.assertEqual(nested_obj.get("b").to_number(), 2.0)

    def test_regex(self):
        """Test working with JavaScript regular expressions."""
        regex_script = """
        const regex = /hello (\\w+)/i;
        const str = 'Hello World';
        const match = regex.exec(str);
        
        // Return the match result
        match
        """

        result = self.runtime.compile_and_evaluate_script(regex_script)
        self.assertTrue(result.is_object())

        match_obj = result.to_object()

        # Check match properties
        full_match = match_obj.get("0")
        group1 = match_obj.get("1")

        self.assertEqual(str(full_match), "Hello World")
        self.assertEqual(str(group1), "World")

        # Check match object properties
        index = match_obj.get("index")
        input_str = match_obj.get("input")

        self.assertEqual(index.to_number(), 0.0)
        self.assertEqual(str(input_str), "Hello World")


if __name__ == "__main__":
    unittest.main()
