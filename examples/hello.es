# examples/hello.es
let message = "Hello, EasyScript!";
print(message);

let add = fun(a, b) {
    a + b;
};

let result = add(10, 20);
print("10 + 20 =", result);

let list = [1, 2, 3];
list.push(4);
print("List:", list);

let map = {"name": "Alice", "age": 30};
print("Map keys:", map.keys());
print("Map values:", map.values());

if result > 25 {
    print("Result is greater than 25.");
} else {
    print("Result is not greater than 25.");
};

for item in list {
    print("Item:", item);
};

let counter = 0;
for counter < 3 {
    print("Loop counter:", counter);
    counter = counter + 1;
};
