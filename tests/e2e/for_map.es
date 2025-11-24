let sum_values = 0;
let my_map = {"a": 10, "b": 20};
for k in my_map {
    if k == "a" {
        sum_values = sum_values + 10;
    };
    if k == "b" {
        sum_values = sum_values + 20;
    };
}
sum_values;
// expect: 30