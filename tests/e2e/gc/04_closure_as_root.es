# Test that a closure keeps its captured variables alive.

let closure = nil;

{
    let a = "captured"; # Allocate the string.
    
    # This closure captures the environment where 'a' lives.
    closure = fun() {
        a 
    };
}

# Here, the original 'a' variable is out of scope. 
# If not for the closure, the string "captured" would be garbage.

let collected_count = gc_collect();

# Call the closure after the GC run.
let result = closure();

# Verify that the value returned is correct AND check the collected count.
print(result == "captured");
print(collected_count);

# The test runner can handle multiple stdout expectations.
# expect_stdout: true
# expect_stdout: 5
