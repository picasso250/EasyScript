let fact = fun (n) {
    if n == 0 {
        1;
    } else {
        n * fact(n - 1);
    };
};
fact(5); // 5 * 4 * 3 * 2 * 1 = 120
// expect: 120