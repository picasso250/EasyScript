// Test that a cycle of objects is collected once it becomes unreachable.

let a = {};
let b = {};

// Create a cycle.
a.b = b;
b.a = a;

// a and b are now unreachable from the root (global scope).
a = nil;
b = nil;

// We expect the GC logs to show that the 2 map objects in the cycle
// have been successfully freed. Without a proper mark-sweep GC,
// a simple reference counter would leak these objects.
gc_collect();

// expect: nil
