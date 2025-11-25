let list1 = [1, 2];
let list2 = [3, 4];
let combined_list = list1 + list2;
print(combined_list); // Should be [1, 2, 3, 4]

let list3 = [];
let list4 = [5];
let combined_list2 = list3 + list4;
print(combined_list2); // Should be [5]

// let mixed_types = [1] + "hello"; // Should cause runtime error

// expect: nil
// expect_stdout: [1, 2, 3, 4]
// expect_stdout: [5]