import unittest

from js_exec import Runtime, JSValue, JSObject, PropertyKey


class JSObjectTests(unittest.TestCase):
    """Tests for JavaScript object manipulation."""

    @classmethod
    def setUpClass(cls):
        # Create a single JavaScript runtime for all tests
        cls.runtime = Runtime()

    def test_create_empty_object(self):
        """Test creating an empty JavaScript object."""
        # Create a new empty object
        obj = JSObject()
        self.assertFalse(obj.is_null())

        # Verify it has no properties
        self.assertFalse(obj.has(PropertyKey.with_string("test")))
        self.assertEqual(len(obj.keys()), 0)

    def test_object_properties(self):
        """Test setting and getting object properties."""
        obj = JSObject()

        # Set properties
        self.assertTrue(obj.set(PropertyKey.with_string("name"), JSValue.string("test")))
        self.assertTrue(obj.set(PropertyKey.with_string("value"), JSValue.i32(42)))

        # Get properties
        name = obj.get("name")
        value = obj.get("value")

        self.assertIsNotNone(name)
        self.assertIsNotNone(value)
        self.assertEqual(str(name), "test")
        self.assertEqual(value.to_number(), 42.0)

        # Check property exists
        self.assertTrue(obj.has(PropertyKey.with_string("name")))
        self.assertTrue(obj.has(PropertyKey.with_string("value")))
        self.assertFalse(obj.has(PropertyKey.with_string("nonexistent")))

    def test_property_deletion(self):
        """Test deleting object properties."""
        obj = JSObject()

        # Set a property
        obj.set(PropertyKey.with_string("test"), JSValue.string("value"))
        self.assertTrue(obj.has(PropertyKey.with_string("test")))

        # Delete the property
        self.assertTrue(obj.delete("test"))
        self.assertFalse(obj.has(PropertyKey.with_string("test")))


    def test_nested_objects(self):
        """Test working with nested JavaScript objects."""
        script = """
        ({
            person: {
                name: 'John',
                age: 30,
                address: {
                    city: 'New York',
                    country: 'USA'
                }
            }
        })
        """
        result = self.runtime.compile_and_evaluate_script(script)
        self.assertTrue(result.is_object())

        # Navigate through the nested objects
        root_obj = result.to_object()
        person = root_obj.get("person")
        self.assertIsNotNone(person)
        self.assertTrue(person.is_object())

        person_obj = person.to_object()
        name = person_obj.get("name")
        age = person_obj.get("age")

        self.assertEqual(str(name), "John")
        self.assertEqual(age.to_number(), 30.0)

        address = person_obj.get("address")
        self.assertTrue(address.is_object())

        address_obj = address.to_object()
        city = address_obj.get("city")
        country = address_obj.get("country")

        self.assertEqual(str(city), "New York")
        self.assertEqual(str(country), "USA")

    def test_object_keys(self):
        """Test getting object keys."""
        script = """({ a: 1, b: 2, c: 3 })"""
        result = self.runtime.compile_and_evaluate_script(script)
        obj = result.to_object()

        keys = obj.keys()
        # Convert keys to strings for easy comparison
        key_strings = [str(key) for key in keys]
        self.assertEqual(len(key_strings), 3)
        self.assertIn("a", key_strings)
        self.assertIn("b", key_strings)
        self.assertIn("c", key_strings)

    def test_object_to_hashmap(self):
        """Test converting a JavaScript object to a Python hashmap."""
        script = """({ a: 1, b: 'test', c: true })"""
        result = self.runtime.compile_and_evaluate_script(script)
        obj = result.to_object()

        hashmap = obj.to_hashmap()
        print(hashmap)
        self.assertEqual(len(hashmap), 3)

        # Check hashmap values
        for key, value in hashmap.items():
            key_str = str(key)
            if key_str == "a":
                self.assertEqual(value.to_number(), 1.0)
            elif key_str == "b":
                self.assertEqual(str(value), "test")
            elif key_str == "c":
                self.assertEqual(value.to_boolean(), True)
            else:
                self.fail(f"Unexpected key: {key_str}")


if __name__ == "__main__":
    unittest.main()
