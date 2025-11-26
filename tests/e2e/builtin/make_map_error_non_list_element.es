# Test make_map function - Error: list containing non-list elements

# Test with list containing non-list elements (should error)
let error_test_2_func = fun() {
    make_map([["a", 1], "not a list pair"]);
};
error_test_2_func(); # Call the function to trigger the runtime error
# expect_runtime_error: make_map() expects a list of lists, but found element of type 'string'.
