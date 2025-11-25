print(string(nil));
print(string(true));
print(string(123));
print(string("hello"));
print(string([1, 2]));
print(string({"a": 1}));
print(string(print)); // Test a native function

let fun_test = fun(a, b) { return a + b; };
print(string(fun_test));

// expect: nil
// expect_stdout: nil
// expect_stdout: true
// expect_stdout: 123
// expect_stdout: hello
// expect_stdout: [1, 2]
// expect_stdout: {a: 1}
// expect_stdout: <function>
// expect_stdout: <function>
