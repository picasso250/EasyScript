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
