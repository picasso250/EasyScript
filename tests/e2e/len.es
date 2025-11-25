print(len("hello"));
print(len("你好"));
print(len([1, 2, 3, 4]));
print(len({"a": 1, "b": 2}));

// The final value of the script is the result of the last expression, which is the print() call.
// print() returns nil.
// expect: nil 

// Expected output from the four print statements:
// expect_stdout: 5
// expect_stdout: 2
// expect_stdout: 4
// expect_stdout: 2