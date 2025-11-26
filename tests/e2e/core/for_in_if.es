# Test for_in_if.es
# Basic for-in loop without if (should still work)
let list1 = [1, 2, 3];
let result1 = for x in list1 { x * 2 };
# expect: [2, 4, 6]

# For-in loop with if condition for a list
let list2 = [1, 2, 3, 4, 5];
let result2 = for x in list2 if x % 2 == 0 { x * 10 };
# expect: [20, 40]

# For-in loop with if condition for a map (iterating keys)
let map1 = {"a": 1, "b": 2, "c": 3};
let result3 = for k in map1 if k == "a" || k == "c" {
    k + "_" + str(map1[k])
};
# expect: ["a_1", "c_3"]

# For-in loop with if condition and block expression (complex projection)
let list3 = [10, 20, 30];
let result4 = for x in list3 if x > 15 {
    let y = x + 5;
    y * 2
};
# expect: [50, 70]

# Empty list
let list4 = [];
let result5 = for x in list4 if x > 0 { x };
# expect: []

# No matching condition
let list5 = [1, 2, 3];
let result6 = for x in list5 if x > 10 { x };
# expect: []
