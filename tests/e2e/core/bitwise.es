# tests/e2e/core/bitwise.es

let a = 5 & 3;
print(a);
# expect_stdout: 1

let b = 5 | 3;
print(b);
# expect_stdout: 7

let c = 5 ^ 3;
print(c);
# expect_stdout: 6

let d = 5 << 1;
print(d);
# expect_stdout: 10

let e = 10 << 2;
print(e);
# expect_stdout: 40

let f = 10 >> 1;
print(f);
# expect_stdout: 5

let g = 40 >> 2;
print(g);
# expect_stdout: 10

let h = 5.9 & 3.1; # (5 & 3)
print(h);
# expect_stdout: 1

let i = 10.9 << 1.1; # (10 << 1)
print(i);
# expect_stdout: 20

let j = -5 & 3;
print(j);
# expect_stdout: 3

let k = -5 | 3;
print(k);
# expect_stdout: -5

let l = -5 ^ 3;
print(l);
# expect_stdout: -8

let m = -5 << 1;
print(m);
# expect_stdout: -10

let n = -5 >> 1;
print(n);
# expect_stdout: -3

# The actual final expression that will be returned by the script.
# This value will be compared against the final #expect.
n # This will be the last expression, and its value is -3.0
# expect: -3