let s1 = "  hello world  ";
print(s1.trim());       // Expected: "hello world"
print(s1.len());        // Expected: 15 (original length)
print(s1.trim().len()); // Expected: 11 (trimmed length)

let s2 = "你好";
print(s2.len());        // Expected: 2

let s3 = "";
print(s3.len());        // Expected: 0
print(s3.trim().len()); // Expected: 0

// Test with numbers in string
let s4 = " 123 ";
print(s4.trim());       // Expected: "123"

// Test calling len() as a global function (should still work)
print(len("test global len")); // Expected: 15

// expect: nil
// expect_stdout: hello world
// expect_stdout: 15
// expect_stdout: 11
// expect_stdout: 2
// expect_stdout: 0
// expect_stdout: 0
// expect_stdout: 123
// expect_stdout: 15
