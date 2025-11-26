# Test make_map function - Success cases

# Test basic creation
let m1 = make_map([["a", 1], ["b", 2]]);
print(m1.len());
# expect_stdout: 2
m1 # Final expression for value assertion
# expect: {"a": 1, "b": 2}

# Test with different key types
let m2 = make_map([[1, "one"], [true, "yes"]]);
print(m2.len());
# expect_stdout: 2
m2 # Final expression for value assertion
# expect: {1: "one", true: "yes"}

# Test with empty list
let m3 = make_map([]);
print(m3.len());
# expect_stdout: 0
m3 # Final expression for value assertion
# expect: {}

# Test combined with for-in expression (map comprehension style)
let source_map = {"k1": 10, "k2": 20, "k3": 30};
let new_map_from_for = make_map(for k in source_map if source_map[k] > 15 {[k + "_suffix", source_map[k] * 2]});
print(new_map_from_for.len());
# expect_stdout: 2
new_map_from_for # Final expression for value assertion
# expect: {"k3_suffix": 60, "k2_suffix": 40}
