print(str(nil));
print(str(true));
print(str(123));
print(str("hello"));
print(str([1, 2]));
print(str({"a": 1}));
print(str(print)); // Test a native function

let fun_test = fun(a, b) { a + b }; // 修改这里
print(str(fun_test));

// expect: nil
// expect_stdout: nil
// expect_stdout: true
// expect_stdout: 123
// expect_stdout: hello
// expect_stdout: [1, 2]
// expect_stdout: {"a": 1}
// expect_stdout: <function>
// expect_stdout: <function>