# Test that a bound method keeps its receiver object alive.

let push_method = nil;

{
    let my_list = [10]; # The list to be kept alive.
    
    # A bound method is created here. It captures 'my_list' as its receiver.
    push_method = my_list.push; 
}

# Here, the 'my_list' variable is out of scope. If the bound method did not
# correctly root its receiver, the list `[10]` would be collected.

let collected_count = gc_collect();

# Call the bound method. This will attempt to push '20' onto the list.
# This call will fail with a runtime error (or crash) if the list was collected.
push_method(20);

# Print the collected count. We expect nothing important was collected.
print(collected_count);

# The push method returns nil, so the script's final value is from print, which also returns nil.
# expect_stdout: 5
