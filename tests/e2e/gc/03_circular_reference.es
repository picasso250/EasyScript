// Test that a cycle of objects is collected once it becomes unreachable.

let a = {};
let b = {};

// Create a cycle.
a.b = b;
b.a = a;

// a and b are now unreachable from the root (global scope).
a = nil;
b = nil;

// We expect the GC logs to show that the 2 map objects in the cycle,
// plus the 2 string keys "a" and "b", have been successfully freed.
print(gc_collect());

// expect_stdout: 13
