// Test that a closure keeps its captured variables alive.

let closure = nil;

{
    let a = "captured"; // Allocate the string.
    
    // This closure captures the environment where 'a' lives.
    closure = fun() {
        a 
    };
}

// Here, the original 'a' variable is out of scope. 
// If not for the closure, the string "captured" would be garbage.

// Trigger a GC cycle.
gc_collect();

// Call the closure after the GC run.
// If the GC incorrectly collected the string "captured", this call would
// likely fail or return an incorrect value.
let result = closure();

// Verify that the value returned from the closure is the one that was captured.
result == "captured"

// expect: true
