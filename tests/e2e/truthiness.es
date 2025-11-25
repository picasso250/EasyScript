// Test bool() function
print("--- Testing bool() function ---");
print(bool(true));       // true
print(bool(1));          // true
print(bool("a"));        // true
print(bool([1]));        // true
print(bool({"a":1}));    // true
print(bool(fun(){}));    // true

print(bool(false));      // false
print(bool(nil));        // false
print(bool(0));          // false
print(bool(""));         // false
print(bool([]));         // false
print(bool({}));         // false

// Test if statement
print("--- Testing if statement ---");
if (true) { print("if true"); }
if (1) { print("if 1"); }
if ("a") { print("if 'a'"); }
if ([1]) { print("if [1]"); }
if ({"a":1}) { print("if {'a':1}"); }
if (fun(){}) { print("if fun(){}"); }

if (false) { print("if false - should not print"); } else { print("else for false"); }
if (nil) { print("if nil - should not print"); } else { print("else for nil"); }
if (0) { print("if 0 - should not print"); } else { print("else for 0"); }
if ("") { print("if '' - should not print"); } else { print("else for ''"); }
if ([]) { print("if [] - should not print"); } else { print("else for []"); }
if ({}) { print("if {} - should not print"); } else { print("else for {}"); }


// expect: nil
// expect_stdout: --- Testing bool() function ---
// expect_stdout: true
// expect_stdout: true
// expect_stdout: true
// expect_stdout: true
// expect_stdout: true
// expect_stdout: true
// expect_stdout: false
// expect_stdout: false
// expect_stdout: false
// expect_stdout: false
// expect_stdout: false
// expect_stdout: false
// expect_stdout: --- Testing if statement ---
// expect_stdout: if true
// expect_stdout: if 1
// expect_stdout: if 'a'
// expect_stdout: if [1]
// expect_stdout: if {'a':1}
// expect_stdout: if fun(){}
// expect_stdout: else for false
// expect_stdout: else for nil
// expect_stdout: else for 0
// expect_stdout: else for ''
// expect_stdout: else for []
// expect_stdout: else for {}
