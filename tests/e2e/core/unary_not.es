# Test unary logical NOT operator '!'

print(!true);
# expect_stdout: false
print(!false);
# expect_stdout: true
print(!0);
# expect_stdout: true
print(!1);
# expect_stdout: false
print(!-1);
# expect_stdout: false
print(!0.0);
# expect_stdout: true
print(!3.14);
# expect_stdout: false
print(!"");
# expect_stdout: true
print(!"hello");
# expect_stdout: false
print(!nil);
# expect_stdout: true
print(![]);
# expect_stdout: true
print(![1, 2]);
# expect_stdout: false
print(!{});
# expect_stdout: true
print(!{"k": "v"});
# expect_stdout: false

# Complex expressions
print(!true && false);
# expect_stdout: false
print(!(true && false));
# expect_stdout: true

print(!0 == true);
# expect_stdout: true
print(!(0 == true));
# expect_stdout: true

let a = 0;
let b = 1;
print(!a && b);
# expect_stdout: 1
print(a || !b);
# expect_stdout: false

# The final expression, whose value will be checked by '# expect:'
!(!false)
# expect: false