// Simple while-style for loop
let i = 0;
for i < 3 {
    print(i);
    i = i + 1;
};
// expect_stdout: 0
// expect_stdout: 1
// expect_stdout: 2
// expect: [nil, nil, nil]

// While-style for loop with an expression as condition
let x = 1;
let y = 5;
for x < y {
    print("Loop iteration");
    x = x + 1;
};
// expect_stdout: Loop iteration
// expect_stdout: Loop iteration
// expect_stdout: Loop iteration
// expect_stdout: Loop iteration
// expect: [nil, nil, nil, nil]

// Loop that returns a collected list
let count = 0;
let results = for count < 4 {
    count = count + 1;
    count * 10;
};
print(results);
// expect_stdout: [10, 20, 30, 40]
// expect: [nil]

// Loop with no iterations (condition immediately false)
let a = 10;
for a < 5 {
    print("Should not print");
};
// expect: []

// Loop with complex condition and scope
let outer = 0;
for outer < 2 {
    let inner = 0;
    for inner < 2 {
        print(str(outer) + " " + str(inner));
        inner = inner + 1;
    };
    outer = outer + 1;
};
// expect_stdout: 0 0
// expect_stdout: 0 1
// expect_stdout: 1 0
// expect_stdout: 1 1
// expect: [nil, nil]
