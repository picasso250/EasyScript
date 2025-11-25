// Test for expression with list iteration
let list_transform = for x in [1, 2, 3] { x * 2 };
print(list_transform); // Expected: [2, 4, 6]

// Test for expression with map iteration (iterates over keys)
let map_keys_transform = for k in {"a": 1, "b": 2} { k + k };
// print(map_keys_transform); // Expected: [aa, bb] (string values without quotes)

// Test for expression with mixed types
let mixed_transform = for x in [1, "hello", true] { str(x) + "!" };
print(mixed_transform); // Expected: [1!, hello!, true!]

// Test for expression with side effects (body returns nil)
let counter = 0;
let side_effect_list = for _ in [1, 2, 3] { counter = counter + 1; };
print(side_effect_list); // Expected: [nil, nil, nil]
print(counter);          // Expected: 3

// Test for expression with empty iterable
let empty_transform = for x in [] { x * 10 };
print(empty_transform); // Expected: []

// expect: nil
// expect_stdout: [2, 4, 6]
// expect_stdout: [1!, hello!, true!]
// expect_stdout: [nil, nil, nil]
// expect_stdout: 3
// expect_stdout: []