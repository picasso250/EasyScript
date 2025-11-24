// Test scoping rules

let a = "global";

{
  let a = "shadow"; // This explicitly shadows the outer 'a'
  a; // This should be the shadowed variable
}

// The value of the block should be "shadow"
a; // This should still be the original outer 'a'

// expect: global
