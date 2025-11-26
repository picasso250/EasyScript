# Test that inner scopes can access outer scope variables

let a = 100;
let b = 50;
let c = 0;

{
  c = a + b; # Access 'a' and 'b' from the outer scope
}

c # The outer 'c' should now have been modified by the inner block

# expect: 150