// Success cases
print(num(123));          // Should be 123
print(num("45.6"));       // Should be 45.6
print(num(true));         // Should be 1
print(num(false));        // Should be 0
print(num(nil));          // Should be 0
print(num("  789  "));    // Should be 789
print(num("-10.5"));      // Should be -10.5

// New test cases for failure scenarios (should return nil)
print(num("abc"));       // Should be nil
print(num([1]));         // Should be nil
print(num({"a": 1}));    // Should be nil
let my_fun = fun(a) { return a; };
print(num(my_fun)); // Should be nil

// The final value of the script is the result of the last expression.
// The last expression is `print(...)`, which returns nil.
// expect: nil

// Expected output from the print statements:
// expect_stdout: 123
// expect_stdout: 45.6
// expect_stdout: 1
// expect_stdout: 0
// expect_stdout: 0
// expect_stdout: 789
// expect_stdout: -10.5
// expect_stdout: nil
// expect_stdout: nil
// expect_stdout: nil
// expect_stdout: nil