// Success cases
print(number(123));          // Should be 123
print(number("45.6"));       // Should be 45.6
print(number(true));         // Should be 1
print(number(false));        // Should be 0
print(number(nil));          // Should be 0
print(number("  789  "));    // Should be 789
print(number("-10.5"));      // Should be -10.5

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
