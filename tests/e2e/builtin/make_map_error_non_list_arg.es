# Test make_map function - Error: non-list argument

# Test with non-list argument (should error)
let error_test_1_func = fun() {
    make_map("not a list");
};
error_test_1_func(); # Call the function to trigger the runtime error
# expect_runtime_error: make_map() expected a list, but got type 'string'.
