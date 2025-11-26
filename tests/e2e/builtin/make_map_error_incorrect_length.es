# Test make_map function - Error: list containing inner lists of incorrect length

# Test with list containing inner lists of incorrect length (should error)
let error_test_3_func = fun() {
    make_map([["a", 1], ["b"]]);
};
error_test_3_func(); # Call the function to trigger the runtime error
# expect_runtime_error: make_map() expects inner lists to have 2 elements (key, value), but found 1 elements.
