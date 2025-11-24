// Test that inner scopes can access outer scope variables

a = 100;
b = 50;
c = 0;

{
  c = a + b; // Access 'a' and 'b' from the outer scope
}

c; // The outer 'c' should now have been modified by the inner block

// expect: 150
