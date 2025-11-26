// Test that an object that becomes unreachable is collected.

let a = { "heavy": "object" }; // 1 map object allocated
a = nil; // The map object is now unreachable.

// We expect the GC logs to show that at least 1 object was freed.
// (The exact number can vary based on compiler optimizations or temporary
// values created during expression evaluation, but at least the map is gone).
gc_collect();

// The script doesn't return anything meaningful, but it shouldn't crash.
// The real test is observing the GC logs during the test run.
// expect: nil
