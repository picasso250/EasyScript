# EasyScript 内置函数与方法参考

本文档详细介绍了 EasyScript 中所有可用的内置函数和方法。

根据 EasyScript 的设计哲学，标准库分为两类：
- **全局内置函数**: 用于普适性、多态性强的操作。
- **内置方法**: 用于特定类型的操作，通过 `.` 运算符调用。

---

## 全局内置函数

### `print(...)`
打印一个或多个值到控制台。
- **签名**: `print(value1, value2, ...)`
- **返回值**: `nil`

### `len(value)`
返回一个对象的“长度”。这是一个多态函数。
- **签名**: `len(value)`
- **返回值**: `number`
- **行为**:
  - `len(string)`: 返回字符串的**字符**数量。
  - `len(list)`: 返回列表的元素个数。
  - `len(map)`: 返回映射的键值对数量。
  - 对其他类型调用会抛出运行时错误。

### `type(value)`
返回一个值的类型的字符串表示。
- **签名**: `type(value)`
- **返回值**: `string` (`"nil"`, `"boolean"`, `"number"`, `"string"`, `"list"`, `"map"`, `"function"`)

### `bool(value)`
将一个值转换为布尔值 (`true` 或 `false`)。
- **签名**: `bool(value)`
- **返回值**: `boolean`
- **假值 (Falsy) 规则**: `nil`, `false`, `0`, `""`, `[]`, `{}` 会被转换为 `false`。其他一切为 `true`。

### `str(value)`
将一个值转换为其字符串表示。
- **签名**: `str(value)`
- **返回值**: `string`

### `num(value)`
将一个值转换为数字。
- **签名**: `num(value)`
- **返回值**: `number` 或 `nil`
- **行为**: 尝试将输入转换为数字。对于无法转换的字符串或不支持的类型，返回 `nil`。

### `input([prompt])`
从标准输入读取一行文本。
- **签名**: `input()` 或 `input(prompt)`
- **返回值**: `string`

### `repr(value)`
返回一个值的“正式”字符串表示 (developer-friendly representation)。
- **签名**: `repr(value)`
- **返回值**: `string`
- **行为**: 返回一个值的字符串表示，通常是带引号的字符串，或能清晰展示值内部结构的格式。对于字符串，这意味着输出将包含引号。对于复合类型（列表、映射），其内部元素也将以其 `repr` 形式显示。
- **示例**:
  ```easyscript
  repr(1)        // "1"
  repr("hello")  // "\"hello\""
  repr([1, "a"]) // "[1, \"a\"]"
  repr({"k": 1}) // "{\"k\": 1}"
  ```

### `gc_collect()`
手动触发一次垃圾回收。
- **签名**: `gc_collect()`
- **返回值**: `number`
- **行为**: 强制执行一次完整的“标记-清除”(Mark-and-Sweep)垃圾回收周期。返回被回收对象的数量。

### `make_map(list_of_pairs)`
将一个包含键值对列表的列表转换为一个映射。
- **签名**: `make_map(list_of_pairs)`
- **返回值**: `map`
- **行为**:
  - `list_of_pairs` 必须是一个列表，其中每个元素本身也是一个包含两个元素的列表 `[key, value]`。
  - `key` 必须是原始类型（字符串、数字或布尔值）。
  - 如果输入不符合预期，将抛出运行时错误。
- **示例**:
  ```easyscript
  let pairs = [["name", "Alice"], ["age", 30], [true, "active"]];
  let my_map = make_map(pairs);
  print(my_map); // {"name": "Alice", "age": 30, true: "active"}

  let data = {"a": 1, "b": 2};
  let new_map = make_map(for k in data if data[k] > 1 {[k + "_new", data[k] * 10]});
  print(new_map); // {"b_new": 20}
  ```


---

## 内置方法

### String 方法

#### `string.len()`
返回字符串的**字符**数量。
- **签名**: `string.len()`
- **返回值**: `number`
- **示例**: `"你好".len()` 返回 `2`。

#### `string.contains(substring)`
检查字符串是否包含给定的子字符串。
- **签名**: `string.contains(substring)`
- **返回值**: `boolean`
- **行为**: 如果字符串包含 `substring`，则返回 `true`，否则返回 `false`。
- **示例**: `"hello world".contains("world")` 返回 `true`。`"hello".contains("xyz")` 返回 `false`。

#### `string.starts_with(prefix)`
检查字符串是否以给定的前缀字符串开头。
- **签名**: `string.starts_with(prefix)`
- **返回值**: `boolean`
- **行为**: 如果字符串以 `prefix` 开头，则返回 `true`，否则返回 `false`。
- **示例**: `"hello world".starts_with("hello")` 返回 `true`。`"hello world".starts_with("world")` 返回 `false`。

#### `string.find(substring)`
在字符串中查找子字符串的第一个出现位置。
- **签名**: `string.find(substring)`
- **返回值**: `number` (索引) 或 `nil`
- **行为**: 返回 `substring` 在字符串中第一次出现的起始索引 (0-based)。如果未找到，则返回 `nil`。
- **示例**: `"hello world".find("world")` 返回 `6`。`"hello".find("xyz")` 返回 `nil`。

#### `string.replace(old, new)`
替换字符串中所有匹配的子字符串。
- **签名**: `string.replace(old, new)`
- **返回值**: `string`
- **行为**: 返回一个新的字符串，其中所有 `old` 子字符串的出现都替换为 `new`。
- **示例**: `"hello world".replace("o", "x")` 返回 `"hellx wxrld"`。

#### `string.split(delimiter)`
将字符串分割成列表。
- **签名**: `string.split(delimiter)`
- **返回值**: `list`
- **行为**: 返回一个列表，其中包含按 `delimiter` 分割后的字符串子串。
- **示例**: `"a,b,c".split(",")` 返回 `["a", "b", "c"]`。

#### `string.trim()`
移除字符串两端的空白字符。
- **签名**: `string.trim()`
- **返回值**: `string`
- **行为**: 返回一个新的字符串，其中移除了原始字符串开头和结尾的所有空白字符。
- **示例**: `"  hello world  ".trim()` 返回 `"hello world"`。

#### `string.to_upper()`
将字符串转换为大写。
- **签名**: `string.to_upper()`
- **返回值**: `string`
- **行为**: 返回一个新的字符串，其中所有字符都转换为大写。
- **示例**: `"Hello".to_upper()` 返回 `"HELLO"`。

#### `string.to_lower()`
将字符串转换为小写。
- **签名**: `string.to_lower()`
- **返回值**: `string`
- **行为**: 返回一个新的字符串，其中所有字符都转换为小写。
- **示例**: `"Hello".to_lower()` 返回 `"hello"`。

### List 方法

#### `list.len()`
返回列表的元素个数。
- **签名**: `list.len()`
- **返回值**: `number`
- **示例**: `[1, 2, 3].len()` 返回 `3`。

#### `list.push(element)`
在列表末尾添加一个元素。
- **签名**: `list.push(element)`
- **返回值**: `nil`
- **行为**: 修改列表本身，将指定元素添加到列表的末尾。
- **示例**:
  ```easyscript
  let my_list = [1, 2];
  my_list.push(3);
  print(my_list); // [1, 2, 3]
  my_list.push("hello");
  print(my_list); // [1, 2, 3, "hello"]
  ```

### Map 方法

#### `map.len()`
返回映射的键值对数量。
- **签名**: `map.len()`
- **返回值**: `number`
- **示例**: `{"a": 1}.len()` 返回 `1`。

#### `map.keys()`
返回一个包含该映射所有键的新列表。
- **签名**: `map.keys()`
- **返回值**: `list`
- **注意**: 返回的列表中，键的顺序是不保证的。
- **示例**: `{"a": 1, "b": 2}.keys()` 返回 `["a", "b"]` (或 `["b", "a"]`)。

#### `map.values()`
返回一个包含该映射所有值的新列表。
- **签名**: `map.values()`
- **返回值**: `list`
- **注意**: 返回的列表中，值的顺序是不保证的。
- **示例**: `{"a": 1, "b": 2}.values()` 返回 `[1, 2]` (或 `[2, 1]`)。