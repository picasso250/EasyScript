outer_var = 100;
{
    let outer_var = 200; // This should shadow the outer_var
    outer_var;
};
outer_var; // This should still be 100
// expect: 100