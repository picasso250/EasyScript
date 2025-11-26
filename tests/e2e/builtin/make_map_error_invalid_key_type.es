# Test make_map function - Error: invalid key type

# Test with invalid key type (should error)
let error_test_4_func = fun() {
    make_map([["a", 1], [["nested"], 2]]);
};
error_test_4_func(); # Call the function to trigger the runtime error
# expect_runtime_error: make_map() map keys must be primitive types (String, Number, Boolean), but got 'list'.
