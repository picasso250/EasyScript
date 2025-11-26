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

# expect_stdout: 3
# expect_stdout: 0
# expect_stdout: 3
# expect_stdout: 2
# expect_stdout: [10]
# expect_stdout: [10, 20]
# expect_stdout: [10, 20, "hello"]
# expect_stdout: [10, 20, "hello", true]

# Test list.pop()
let pop_list = [1, 2, 3];
print(pop_list.pop());   # Expected: 3
print(pop_list);         # Expected: [1, 2]
print(pop_list.pop());   # Expected: 2
print(pop_list);         # Expected: [1]
print(pop_list.pop());   # Expected: 1
print(pop_list);         # Expected: []
print(pop_list.pop());   # Expected: nil
print(pop_list);         # Expected: []

# expect_stdout: 3
# expect_stdout: [1, 2]
# expect_stdout: 2
# expect_stdout: [1]
# expect_stdout: 1
# expect_stdout: []
# expect_stdout: nil
# expect_stdout: []

# Test list.remove()
let remove_list = [10, 20, 30, 40];
print(remove_list.remove(1));    # Expected: 20
print(remove_list);              # Expected: [10, 30, 40]
print(remove_list.remove(2));    # Expected: 40
print(remove_list);              # Expected: [10, 30]
print(remove_list.remove(0));    # Expected: 10
print(remove_list);              # Expected: [30]
# remove_list.remove(5); # This should cause a runtime error "List index out of bounds"

# expect_stdout: 20
# expect_stdout: [10, 30, 40]
# expect_stdout: 40
# expect_stdout: [10, 30]
# expect_stdout: 10
# expect_stdout: [30]

# Test list.insert()
let insert_list = [10, 30];
print(insert_list);          # Expected: [10, 30]
insert_list.insert(1, 20);
print(insert_list);          # Expected: [10, 20, 30]
insert_list.insert(0, 5);
print(insert_list);          # Expected: [5, 10, 20, 30]
insert_list.insert(4, 40);
print(insert_list);          # Expected: [5, 10, 20, 30, 40]
insert_list.insert(2, "middle");
print(insert_list);          # Expected: [5, 10, "middle", 20, 30, 40]

# expect_stdout: [10, 30]
# expect_stdout: [10, 20, 30]
# expect_stdout: [5, 10, 20, 30]
# expect_stdout: [5, 10, 20, 30, 40]
# expect_stdout: [5, 10, "middle", 20, 30, 40]

# Test list.join()
let join_list = [1, "hello", 3.14, true];
print(join_list.join(", "));    # Expected: 1, hello, 3.14, true
print([].join("-"));            # Expected: 
let another_join_list = ["a", "b", "c"];
print(another_join_list.join("")); # Expected: abc
print(another_join_list.join("|")); # Expected: a|b|c

# expect_stdout: 1, hello, 3.14, true
# expect_stdout: 
# expect_stdout: abc
# expect_stdout: a|b|c
