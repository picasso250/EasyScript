// Test with a map that has number values
let my_map1 = {"a": 10, "b": 20, "c": 30};
let values1 = my_map1.values();
print(len(values1)); // Should be 3

let sum = 0;
for v in values1 {
    sum = sum + v;
}
print(sum); // Should be 60 (10+20+30)

// Test with a map that has string values
let my_map2 = {1: "hello", 2: "world"};
let values2 = my_map2.values();
print(len(values2)); // Should be 2

// Test with an empty map
let my_map3 = {};
let values3 = my_map3.values();
print(len(values3)); // Should be 0
print(values3);     // Should be []

// expect: nil
// expect_stdout: 3
// expect_stdout: 60
// expect_stdout: 2
// expect_stdout: 0
// expect_stdout: []
