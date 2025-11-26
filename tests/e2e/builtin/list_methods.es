let l1 = [1, 2, 3];
print(l1.len());        # Expected: 3

let l2 = [];
print(l2.len());        # Expected: 0

let l3 = [1, "hello", true];
print(l3.len());        # Expected: 3

# Test calling len() as a global function (should still work)
print(len([10, 20])); # Expected: 2

# Test list.push()
let my_list = [10];
print(my_list);         # Expected: [10]
my_list.push(20);
print(my_list);         # Expected: [10, 20]
my_list.push("hello");
print(my_list);         # Expected: [10, 20, "hello"]
my_list.push(true);
print(my_list);         # Expected: [10, 20, "hello", true]

# expect: nil
# expect_stdout: 3
# expect_stdout: 0
# expect_stdout: 3
# expect_stdout: 2
# expect_stdout: [10]
# expect_stdout: [10, 20]
# expect_stdout: [10, 20, "hello"]
# expect_stdout: [10, 20, "hello", true]
