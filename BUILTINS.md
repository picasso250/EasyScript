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

### List 方法

#### `list.len()`
返回列表的元素个数。
- **签名**: `list.len()`
- **返回值**: `number`
- **示例**: `[1, 2, 3].len()` 返回 `3`。

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
