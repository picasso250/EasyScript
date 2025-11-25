let l1 = [1, 2, 3];
print(l1.len());        // Expected: 3

let l2 = [];
print(l2.len());        // Expected: 0

let l3 = [1, "hello", true];
print(l3.len());        // Expected: 3

// Test calling len() as a global function (should still work)
print(len([10, 20])); // Expected: 2

// expect: nil
// expect_stdout: 3
// expect_stdout: 0
// expect_stdout: 3
// expect_stdout: 2
