// Test that an object that becomes unreachable is collected.

let a = { "heavy": "object" }; // 1 map object, plus its key and value strings allocated.
a = nil; // The map object is now unreachable.

// We expect the GC to free the map, its key, and its value.
print(gc_collect());

// The real test is observing the stdout.
// expect_stdout: 6
