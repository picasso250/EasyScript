// Test chained if-else if-else statement

a = 10;
result = if a == 1 {
  "one";
} else if a > 5 {
  "greater than five";
} else {
  "less than or equal to five";
};
result;

// expect: greater than five
