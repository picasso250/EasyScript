# EasyScript E2E 测试指南

本文档介绍如何运行 EasyScript 的端到端 (E2E) 测试，以及如何使用 `TEST_SCOPE` 环境变量来控制测试的范围。

## 运行所有 E2E 测试

要运行所有 E2E 测试（包括 `core`、`builtin` 和 `functional` 目录下的测试），请使用以下命令：

```bash
cargo test
```

## 按范围运行 E2E 测试

您可以通过设置 `TEST_SCOPE` 环境变量来指定只运行特定类别的 E2E 测试。

### 只运行 `core` 测试

如果您只想运行位于 `tests/e2e/core` 目录下的测试文件，请使用以下命令：

```bash
TEST_SCOPE=core cargo test
```
*(在 Windows PowerShell 中，您可能需要使用 `$env:TEST_SCOPE="core"; cargo test`。在 Git Bash 或 WSL 中，直接使用 `TEST_SCOPE=core cargo test`。)*

### 只运行 `builtin` 测试

如果您只想运行位于 `tests/e2e/builtin` 目录下的测试文件，请使用以下命令：

```bash
TEST_SCOPE=builtin cargo test
```
*(在 Windows PowerShell 中，您可能需要使用 `$env:TEST_SCOPE="builtin"; cargo test`。在 Git Bash 或 WSL 中，直接使用 `TEST_SCOPE=builtin cargo test`。)*

### 其他作用域

如果 `TEST_SCOPE` 设置为除 `core` 或 `builtin` 之外的任何其他值，或者未设置，则 `cargo test` 将运行所有 E2E 测试。

## 如何添加新的 E2E 测试

1.  在 `tests/e2e/` 目录下创建相应的子目录 (例如 `tests/e2e/new_feature/`)。
2.  在该子目录中创建 `.es` 文件，编写 EasyScript 代码。
3.  在 `.es` 文件中，编写 EasyScript 代码时，可以使用 `#` 进行单行注释。此外，您需要使用 `# expect: <expected_value>` 和 `# expect_stdout: <expected_stdout>` 作为特殊注释来定义测试的预期结果。
    *   `# expect:` 用于检查脚本执行后的最终返回值。
    *   `# expect_stdout:` 用于检查脚本在执行过程中打印到标准输出的内容。
    *   一个测试文件必须至少包含一个 `# expect:` 或 `# expect_stdout:` 注释。

**示例 `tests/e2e/core/example.es`:**

```easyscript
let x = 10; # 这是一个注释
print("Hello from test!"); # 打印输出
x + 5 # 最后一个表达式的值是脚本的返回值
# expect: 15
# expect_stdout: Hello from test!
```
