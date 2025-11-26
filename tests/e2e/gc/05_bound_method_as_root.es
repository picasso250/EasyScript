// Test that a bound method keeps its receiver object alive.

let push_method = nil;

{
    let my_list = [10]; // The list to be kept alive.
    
    // A bound method is created here. It captures 'my_list' as its receiver.
    push_method = my_list.push; 
}

// Here, the 'my_list' variable is out of scope. If the bound method did not
// correctly root its receiver, the list `[10]` would be collected.

gc_collect();

// Call the bound method. This will attempt to push '20' onto the list.
// This call will fail with a runtime error (or crash) if the list was collected.
push_method(20);

// We can't access the list directly anymore to check its contents,
// but the fact that the call above didn't crash is a strong indication
// that the receiver was kept alive. The push_method call returns nil.

// expect: nil
