// Test with a map that has number keys
let my_map1 = {1: "a", 2: "b", 3: "c"};
let keys1 = my_map1.keys();
print(len(keys1)); // Should be 3

let sum = 0;
for k in keys1 {
    sum = sum + k;
}
print(sum); // Should be 6 (1+2+3)

// Test with a map that has string keys
let my_map2 = {"a": 1, "b": 2};
let keys2 = my_map2.keys();
print(len(keys2)); // Should be 2

// Test with an empty map
let my_map3 = {};
let keys3 = my_map3.keys();
print(len(keys3)); // Should be 0
print(keys3);     // Should be []

// expect: nil
// expect_stdout: 3
// expect_stdout: 6
// expect_stdout: 2
// expect_stdout: 0
// expect_stdout: []
