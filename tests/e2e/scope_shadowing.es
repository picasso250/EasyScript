// Test scoping rules

a = "global";

{
  a = "shadow"; // This shadows the global 'a'
  a; // This should be the shadowed variable
}

// The value of the block should be "shadow"
// expect: shadow
