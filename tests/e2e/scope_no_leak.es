// Test that inner scope assignments do not leak to outer scope

let a = "outer";

{
  a = "inner"; // Create a new 'a' that shadows the outer one
}

a // This should now be the modified 'a' from the inner block

// expect: inner