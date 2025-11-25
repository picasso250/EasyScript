// Y 组合子实现阶乘函数

// G 是一个函数，它接受一个递归函数 "rec" 并返回阶乘的逻辑
let G = fun (rec) {
    fun (n) {
        if n == 0 {
            1
        } else {
            n * rec(n - 1)
        }
    }
};

// Y 组合子本身
// Y = fun (f) { (fun (x) { f(fun (arg) { (x(x))(arg); }); }) (fun (x) { f(fun (arg) { (x(x))(arg); }); }) };
// 简化写法，为了避免在 EasyScript 中出现可能的问题
let Y = fun (f) {
    let x_builder = fun (g) {
        // 这是 f(y(y)) 中的 y 匿名函数
        f(fun (arg) {
            (g(g))(arg)
        })
    };
    x_builder(x_builder)
};

let factorial = Y(G);
factorial(5) // 5 * 4 * 3 * 2 * 1 = 120
// expect: 120
