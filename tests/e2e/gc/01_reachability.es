# Test that simple, reachable objects are not collected by the GC.

let a = 10;
let b = "hello";
let c = [1, 2, 3];
let d = {"key": "value"};

# At this point, 4 objects (number, string, list, map) are on the heap.
# We expect the GC logs to show a collection, but 0 objects freed.

gc_collect(); # Manually trigger GC

let result = a + 20; # 30

# We access `a` after collection to ensure it was not prematurely freed.
# The result of the script is the final expression.

# expect: 30
