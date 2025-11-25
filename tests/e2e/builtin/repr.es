// Test the global built-in repr() function

print(repr(nil));
print(repr(true));
print(repr(123));
print(repr("hello"));
print(repr([1, 2]));
print(repr({"a": 1}));
print(repr(print)); // repr of a native function

let fun_test = fun(a, b) { a + b };
print(repr(fun_test)); // repr of a user-defined function

// expect: nil
// expect_stdout: nil
// expect_stdout: true
// expect_stdout: 123
// expect_stdout: "hello"
// expect_stdout: [1, 2]
// expect_stdout: {"a": 1}
// expect_stdout: <function>
// expect_stdout: <function>
