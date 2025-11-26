# EasyScript 垃圾回收 (GC) 设计文档 (第一版)

**1. 目标**
*   实现 EasyScript 对象（列表、映射、函数闭包等）的自动内存管理。
*   提供真正的“原地修改”语义，确保对共享对象的修改在所有引用处可见，消除 `Rc::make_mut` 带来的“写时复制”行为可能导致的混淆。
*   简化程序员对内存生命周期的管理负担。

**2. GC 方案选择：Mark-and-Sweep Stop-the-World**
*   **选择原因**：对于第一版实现，该算法相对简单直观，易于理解和实现，能有效处理循环引用问题。
*   **工作原理**：
    *   **标记 (Mark)**：从一组已知“根”对象开始，递归遍历所有可达对象，并对其进行标记。
    *   **清除 (Sweep)**：遍历整个 GC 堆，回收所有未被标记的对象所占用的内存。
    *   **Stop-the-World (STW)**：在 GC 运行时暂停 EasyScript 代码的执行，确保对象图的静态性，简化实现难度。

**3. 核心组件**

*   **3.1 GC 堆 (GC Heap)**
    *   **目的**：统一管理所有 GC 对象分配的内存。
    *   **实现**：可能是一个 `Vec<Option<GcObjectHeader>>` 或自定义的内存池。`GcObjectHeader` 包含对象类型信息、标记位等。
    *   **对象存储**：GC 堆将存储 `Object` 枚举的实例，因为所有 EasyScript 运行时数据都将是 `Object` 的一个变体。

*   **3.1.1 开发者调试选项**
    *   **`DEBUG_GC` 环境变量**：
        *   当 `DEBUG_GC` 被设置为 `1` 时，GC 相关的内存分配和回收操作将在标准错误输出 (`eprintln!`) 中打印详细日志。这对于跟踪对象的生命周期和调试 GC 行为非常有用。


*   **3.2 `Gc<T>` 句柄 (Gc Handle)**
    *   **目的**：统一作为指向 GC 堆上 `Object` 实例的安全引用。
    *   **实现**：`Gc<T>` 将主要用作 `Gc<Object>`。它将是一个包装了 GC 堆索引或裸指针的智能指针。它需要实现 `Copy`, `Clone` 等，以便在 EasyScript 代码中方便传递。
    *   **生命周期**：`Gc<T>` 的生命周期由 GC 机制而非 Rust 编译器管理。

*   **3.3 `Value` 类型定义**
    *   **目的**：简化 `Value` 类型，使其统一表示为 GC 管理的堆对象。
    *   **修改**：`Value` 不再是枚举，而是一个包装 `Gc<Object>` 的结构体。这意味着 EasyScript 中的每一个 `Value` 都是一个指向 GC 堆上 `Object` 实例的句柄。
        ```rust
        // src/value.rs
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] // Gc<Object> 应实现这些 Trait
        pub struct Value(pub Gc<Object>);
        ```
    *   **内部结构**：引入 `Object` 枚举，它将包含 EasyScript 运行时所有可能的数据类型（Number, Boolean, Nil, String, List, Map, Function, BoundMethod 等）。`Object` 的实例将直接存储在 GC 堆上。

*   **3.4 `Object` 枚举**
    *   **目的**：定义所有 EasyScript 运行时数据的实际存储结构，它们将统一由 GC 管理。
    *   **实现**：
        ```rust
        // src/gc.rs 或 src/object.rs (此处暂时定义在 src/gc.rs)
        pub enum Object {
            Number(f64),
            Boolean(bool),
            String(String),
            Nil,
            List(Vec<Value>), // 列表中包含 Value 句柄
            Map(HashMap<Value, Value>), // Map 的键和值都是 Value 句柄
            Function(FunctionObject), // FunctionObject 封装用户或原生函数
            BoundMethod {
                receiver: Value, // receiver 也是 Value 句柄
                method_name: String, // 方法名，用于在 Interpreter 中查找原生函数
                // method: NativeFunction, // NativeFunction 将在调用时查找
            },
            // ... 其他可能的类型
        }
        ```
    *   **注意**：`Object` 枚举中的 `List` 和 `Map` 将存储 `Value` 句柄，而 `Value` 又包裹了 `Gc<Object>`，这形成了 GC 对象图中的引用链。

*   **3.5 `GcTrace` Trait**
    *   **目的**：定义 GC 如何遍历对象图。
    *   **实现**：**`Object` 枚举**将实现 `GcTrace` trait。
    *   `GcTrace::trace(&self)` 方法将负责标记其自身以及其内部引用的所有 `Gc<Object>` 句柄。

*   **3.6 GC 根 (GC Roots)**
    *   **目的**：GC 标记阶段的起始点。
    *   **识别**：
        *   `Environment` 中的所有变量 (`HashMap<String, Value>`)。
        *   解释器调用栈上，**所有作为局部变量或参数的 `Value` 句柄**。
        *   全局内置函数/常量等。

**4. GC 算法流程 (Mark-and-Sweep)**

*   **4.1 触发 (Trigger)**
    *   **第一版**：可能通过手动调用 `gc.collect()` 触发，或在每次解释器执行一定数量的表达式后（简单计数器）。
    *   **未来**：基于 GC 堆的内存使用量或对象分配数量自动触发。

*   **4.2 暂停世界 (Stop-the-World)**
    *   在 GC 运行期间，所有 EasyScript 代码的执行必须暂停。这通常通过一个标志位和解释器的检查点来实现。

*   **4.3 标记阶段 (Mark Phase)**
    *   将所有 GC 堆上的对象标记位重置为“未标记”。
    *   遍历所有 GC 根，对每个根持有的 `Value` 调用 `GcTrace::trace()`。
    *   `GcTrace::trace()` 会递归地标记所有可达的 GC 对象。

*   **4.4 清除阶段 (Sweep Phase)**
    *   遍历 GC 堆。
    *   对于每个“未标记”的对象，释放其占用的内存。
    *   对于“已标记”的对象，将其标记位重置为“未标记”，为下一次 GC 做准备。

*   **4.5 恢复世界 (Resume-the-World)**
    *   GC 完成后，允许 EasyScript 代码继续执行。

**5. 对现有代码库的影响**
*   `src/value.rs`：`Value` 枚举的定义，`List` 和 `Map` 的内部类型，`BoundMethod` 的 `receiver` 类型，`PartialEq` 和 `Hash` 实现。
*   `src/interpreter.rs`：`Interpreter` 结构（增加 GC 堆），`evaluate` 方法中所有 `List` 和 `Map` 的创建、访问和修改逻辑。`BoundMethod` 的创建和调用逻辑。
*   `src/native.rs`：所有操作 `List` 和 `Map` 的内置函数，它们将不再使用 `Rc::make_mut`，而是直接通过 `Gc<T>` 句柄的可变引用修改底层数据。
*   `src/environment.rs`：`Environment` 存储 `Value`，需要确保 `Value` 正确处理 `Gc` 句柄的追踪。
*   `src/parser.rs`：字面量创建部分需要知道如何分配 `List` 和 `Map` 到 GC 堆。

**6. `unsafe` 代码的使用**
*   `Gc<T>` 的实现将不可避免地涉及 `unsafe` Rust 代码，用于直接操作内存和裸指针。
*   将 `unsafe` 隔离在 `Gc<T>` 类型和 GC 堆管理的代码中，最小化其影响范围，并确保其正确性。

**7. 第一版实现 (不求效率，只求功能正确)**
*   GC 堆可以从简单的 `Vec<Option<Box<dyn GcObject>>>` 开始。
*   `Gc<T>` 可以是一个简单的 `usize` 索引到 GC 堆的 `Vec`。
*   Mark-and-Sweep 可以是单次完整遍历。

**8. 未来优化方向**
*   更高效的 GC 堆分配器（例如分代分配）。
*   增量 GC 或并发 GC 以减少 STW 时间。
*   内存碎片整理。
*   弱引用支持。
