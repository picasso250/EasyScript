# EasyScript 语言核心

本文档详细介绍了 EasyScript 语言的核心特性、语法结构和基本概念。

## 1. 基本语法元素

### 注释 (Comments)

EasyScript 支持单行注释：

```easyscript
# 这是一个单行注释
let x = 10; # 这是行尾注释
```

### 标识符 (Identifiers)

用于命名变量、函数等。
- 必须以字母或下划线 `_` 开头。
- 后续可以是字母、数字或下划线。
- 区分大小写。

```easyscript
let myVariable = 10;
let _another_one = "hello";
let calculateSum = fun(a, b) { a + b };
```

### 关键字 (Keywords)

EasyScript 的关键字包括：
`let`, `fun`, `if`, `else`, `for`, `true`, `false`, `nil`。

## 2. 数据类型 (Data Types)

EasyScript 是一种动态类型语言，支持以下基本数据类型：

### 数字 (Number)

所有数字都以 64 位浮点数 (`f64`) 存储。

```easyscript
let integer = 10;
let float = 3.14;
let negative = -5;
```

### 布尔值 (Boolean)

表示真或假，`true` 或 `false`。

```easyscript
let isActive = true;
let hasPermission = false;
```

### 字符串 (String)

由双引号 `"` 包裹的文本序列。

```easyscript
let greeting = "Hello, EasyScript!";
let emptyString = "";
```

### 列表 (List)

有序的异构值集合，用方括号 `[]` 包裹。

```easyscript
let myList = [1, "hello", true, 3.14];
let emptyList = [];
```

### 映射 (Map)

无序的键值对集合，键必须是字符串、数字或布尔值，用花括号 `{}` 包裹。

```easyscript
let myMap = {"name": "Alice", "age": 30};
let anotherMap = {1: "one", true: "yes"};
let emptyMap = {};
```

### 函数 (Function)

一等公民，可以作为值传递和返回。使用 `fun` 关键字定义。

```easyscript
let add = fun(a, b) { a + b };
let greet = fun(name) { print("Hello, " + name); };
```

### 空值 (Nil)

表示缺失或无效的状态，类似其他语言的 `null` 或 `None`。

```easyscript
let noValue = nil;
```

## 3. 变量与作用域 (Variables and Scope)

### 变量声明 (Variable Declaration)

使用 `let` 关键字声明变量。变量声明后可重新赋值。

```easyscript
let message = "initial";
message = "new value"; // 重新赋值
```

### 作用域 (Scope)

EasyScript 使用词法作用域 (Lexical Scoping)。变量在定义它们的代码块 `{}` 中可见。

```easyscript
let globalVar = 10;

let myFunction = fun() {
    let localVar = 20;
    print(globalVar); // 访问外部作用域变量
};
```

## 4. 运算符 (Operators)

EasyScript 支持常见的算术、比较和逻辑运算符。

### 算术运算符 (Arithmetic Operators)

- `+` (加法，字符串和列表的拼接)
- `-` (减法)
- `*` (乘法)
- `/` (除法)

```easyscript
let result = 10 + 5;    // 15
let combined = "hello" + " world"; // "hello world"
let mergedList = [1, 2] + [3, 4]; // [1, 2, 3, 4]
```

### 比较运算符 (Comparison Operators)

- `==` (等于)
- `!=` (不等于)
- `<` (小于)
- `<=` (小于等于)
- `>` (大于)
- `>=` (大于等于)

```easyscript
print(10 == 10); // true
print("hello" != "world"); // true
print(5 < 10); // true
```

### 逻辑运算符 (Logical Operators)

- `!` (逻辑非)
- `&&` (逻辑与)
- `||` (逻辑或)

```easyscript
print(true && false); // false
print(true || false);  // true
```

### 运算符优先级 (Operator Precedence)

EasyScript 的运算符优先级遵循以下规则。数字越小表示优先级越高。所有二元运算符的结合性均为左结合（从左到右计算），但赋值运算符 `=` 和一元运算符 `-` 除外，它们是右结合。

| 优先级 | 运算符                               | 描述                                     | 结合性   |
| :----- | :----------------------------------- | :--------------------------------------- | :------- |
| 1      | `()`, `[]`, `.`                      | 函数调用, 索引访问, 属性访问             | 左结合   |
| 2      | `-` (一元), `!`                   | 一元负号, 逻辑非             | 右结合   |
| 3      | `*`, `/`, `%`                        | 乘法, 除法, 取模                         | 左结合   |
| 4      | `+`, `-` (二元)                      | 加法, 减法                               | 左结合   |
| 5      | `\|`, `^`, `&`, `<<`, `>>`            | 位或, 位异或, 位与, 位移 (统一优先级)    | 左结合   |
| 6      | `==`, `!=`, `<`, `<=`, `>`, `>=`   | 等性, 比较 (统一优先级)                  | 左结合   |
| 7      | `\|\|`, `&&`                           | 逻辑或, 逻辑与 (统一优先级)              | 左结合   |
| 8      | `=`                                  | 赋值                                     | 右结合   |

**注意:**

-   为了简化语言模型并遵循人类直觉，EasyScript 将一些在传统 C 风格语言中具有不同优先级的运算符合并到同一个优先级级别。例如：
    -   逻辑运算符 `||` 和 `&&` 具有相同的优先级。这意味着 `a || b && c` 将被解析为 `(a || b) && c`。在需要不同行为时，请使用括号明确意图，例如 `a || (b && c)`。
    -   等性运算符 (`==`, `!=`) 和比较运算符 (`<`, `<=`, `>`, `>=`) 具有相同的优先级。
    -   所有位运算符 (`|`, `^`, `&`, `<<`, `>>`) 具有相同的优先级。
-   这种设计旨在减少记忆负担，但这意味着在编写涉及这些混合运算符的复杂表达式时，强烈建议使用括号 `()` 来明确意图，以确保代码行为符合预期，并提高可读性。
-   逻辑非 `!` 运算符的行为遵循 EasyScript 的真值判断规则。例如，`!0` 为 `true`，`!"hello"` 为 `false`，`!nil` 为 `true`。

## 5. 控制流 (Control Flow)

EasyScript 的控制流结构都是表达式。

### If 表达式 (If Expression)

```easyscript
let result = if (10 > 5) {
    "10 is greater than 5"
} else {
    "10 is not greater than 5"
};
print(result);

// 没有 else 分支时，如果条件为假，整个 if 表达式返回 nil
let status = if (false) { "active" };
print(status); // nil
```

### For 表达式 (For Expression)

EasyScript 支持两种 `for` 循环。

#### `while` 风格循环
这是一种条件循环，其行为类似其他语言中的 `while` 循环。只要条件表达式为真 (`truthy`)，循环就会持续执行。

```easyscript
let i = 0;
for i < 5 {
    print(i);
    i = i + 1;
}
```

#### `for-in` 迭代循环
用于遍历列表或映射的键，并支持可选的 `if` 过滤条件。

```easyscript
# 基本的 for-in 循环
let myNumbers = [1, 2, 3];
for num in myNumbers { # 注意：for num in myNumbers, 没有小括号
    print(num * 2);
};

# 带 if 条件的 for-in 循环
let filteredNumbers = for num in myNumbers if num > 1 {
    num * 10
};
print(filteredNumbers); // 示例输出: [20, 30]

let mySettings = {"theme": "dark", "fontSize": 14, "darkMode": true};
for key in mySettings if mySettings[key] == true { // 注意：for-in 遍历 map 时，得到的是 key
    print("Setting " + key + " is true.");
};
```



## 6. 函数 (Functions)

### 函数定义 (Function Definition)

使用 `fun` 关键字定义函数。函数体是一个表达式块。最后一个表达式的值是函数的返回值。

```easyscript
let multiply = fun(a, b) {
    a * b // 隐式返回 a * b
};

print(multiply(3, 4)); // 12
```

### 函数调用 (Function Call)

使用圆括号 `()` 调用函数。

```easyscript
let myFunc = fun() { print("Called!"); };
myFunc(); // 调用函数

let sum = add(10, 20); // 30
```

## 7. 表达式块 (Block Expressions)

EasyScript 中的 `{}` 不仅仅用于控制流，它们本身就是表达式，会返回块中最后一个表达式的值。这允许创建局部作用域和封装逻辑。

```easyscript
let x = {
    let a = 1;
    let b = 2;
    a + b // 块表达式返回 3
};
print(x); // 3

let y = {
    print("side effect");
    10 // 块表达式返回 10
};
print(y); // 10
```
