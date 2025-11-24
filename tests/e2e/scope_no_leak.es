// Test that inner scope assignments do not leak to outer scope

a = "outer";

{
  a = "inner"; // Create a new 'a' that shadows the outer one
}

a; // This should still be the original, outer 'a'

// expect: outer
