print(type(nil));
print(type(true));
print(type(123));
print(type("hello"));
print(type([1, 2]));
print(type({"a": 1}));
print(type(print)); # Test a native function

# Test a user-defined function
let my_func_test = fun(a, b) { a + b }; # 修改这里
print(type(my_func_test));


# expect: nil
# expect_stdout: nil
# expect_stdout: boolean
# expect_stdout: number
# expect_stdout: string
# expect_stdout: list
# expect_stdout: map
# expect_stdout: function
# expect_stdout: function