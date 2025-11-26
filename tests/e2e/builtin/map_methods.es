let m1 = {"a": 1, "b": 2};
print(m1.len());        # Expected: 2

let m2 = {};
print(m2.len());        # Expected: 0

let m3 = {"key": "value", 10: true, false: nil}; # 键改回数字和布尔值
print(m3.len());        # Expected: 3

# Test calling len() as a global function (should still work)
print(len({"x": 1})); # Expected: 1

# expect: nil
# expect_stdout: 2
# expect_stdout: 0
# expect_stdout: 3
# expect_stdout: 1

# Test map.has_key()
let test_map_hk = {"name": "Alice", "age": 30, true: "yes", 1: "one"};
print(test_map_hk.has_key("name"));     # Expected: true
print(test_map_hk.has_key("city"));     # Expected: false
print(test_map_hk.has_key(true));       # Expected: true
print(test_map_hk.has_key(1));          # Expected: true
print(test_map_hk.has_key(false));      # Expected: false
print({}.has_key("empty"));             # Expected: false

# expect_stdout: true
# expect_stdout: false
# expect_stdout: true
# expect_stdout: true
# expect_stdout: false
# expect_stdout: false
