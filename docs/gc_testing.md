# EasyScript GC 测试指南

本文档介绍如何对 EasyScript 的垃圾回收 (GC) 机制进行端到端 (E2E) 测试。

## GC 测试工具

为了进行可预测和确定性的 GC 测试，我们引入了一个新的全局内置函数：

### `gc_collect()`

- **签名**: `gc_collect()`
- **行为**: 手动触发一次完整的“标记-清除”(Mark-and-Sweep)垃圾回收周期。
- **返回值**: `nil`
- **用途**: 在脚本的特定位置强制执行垃圾回收，以便我们能验证对象是否被正确回收或保留。

在运行测试时，GC 会在标准错误输出 (stderr) 中打印调试信息，如下所示：

```
[GC] Manual collection triggered.
[GC] Starting collection phase. 10 objects on heap.
[GC] Swept and freed 2 objects. 8 remaining.
```

通过观察这些日志，我们可以确认 GC 是否按预期执行。

## 运行 GC 测试

要单独运行所有 GC 相关的 E2E 测试，请使用以下命令：

```bash
TEST_SCOPE=gc cargo test
```
*(在 Windows PowerShell 中, 使用 `$env:TEST_SCOPE="gc"; cargo test`)*

## GC 测试用例设计

GC 测试位于 `tests/e2e/gc/` 目录下，每个文件都专注于一个特定的场景：

- **`01_reachability.es`**:
  - **目的**: 验证简单存活的对象在 GC 后依然可以访问，GC 不会错误地回收它们。
  - **场景**: 创建几个变量，手动触发 GC，然后访问这些变量并验证其值。

- **`02_unreachability.es`**:
  - **目的**: 验证一个引用被覆盖（不可达）的对象会被 GC 回收。
  - **场景**: 创建一个对象，然后将其变量重新赋值为 `nil`，使其不可达。触发 GC 后，通过日志验证对象被回收。

- **`03_circular_reference.es`**:
  - **目的**: 验证两个相互引用的对象在与根节点断开连接后，能被双双回收。这是检验 Mark-and-Sweep GC 是否有效的关键测试。
  - **场景**: 创建两个 map `a` 和 `b`，让它们互相引用 (`a.b = b; b.a = a;`)，然后将 `a` 和 `b` 设为 `nil`。触发 GC，通过日志验证两个 map 对象都被回收。

- **`04_closure_as_root.es`**:
  - **目的**: 验证被闭包捕获的变量，在外部引用消失后，依然能因为闭包的存在而存活。
  - **场景**: 创建一个变量 `a`，定义一个捕获了 `a` 的函数 `f`。将 `a` 设为 `nil`，触发 GC。此时对象不应被回收。调用 `f` 并验证其返回值是否正确。

- **`05_bound_method_as_root.es`**:
  - **目的**: 验证一个对象的方法被赋值给变量后，即使对象本身的引用消失了，该对象依然会因为方法绑定而存活。
  - **场景**: 创建一个列表 `l`，将其 `push` 方法赋值给一个变量 `m`。将 `l` 设为 `nil`，触发 GC。此时列表不应被回收。调用 `m`，程序不应崩溃。
