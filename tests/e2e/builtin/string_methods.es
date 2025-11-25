// tests/e2e/string_methods.es

let s = "hello world";
print(s.find("hello")); // 0
// expect_stdout: 0
print(s.find("world")); // 6
// expect_stdout: 6
print(s.find("o"));     // 4 (first 'o')
// expect_stdout: 4
print(s.find("xyz"));   // nil
// expect_stdout: nil
print(s.find(""));      // 0 (empty string always found at start)
// expect_stdout: 0

print(s.starts_with("hello")); // true
// expect_stdout: true
print(s.starts_with("world")); // false
// expect_stdout: false
print(s.starts_with(""));      // true
// expect_stdout: true
print(s.starts_with("h"));     // true
// expect_stdout: true
print(s.starts_with("xyz"));   // false
// expect_stdout: false

print(s.contains("hello")); // true
// expect_stdout: true
print(s.contains("world")); // true
// expect_stdout: true
print(s.contains("o"));     // true
// expect_stdout: true
print(s.contains("xyz"));   // false
// expect_stdout: false
print(s.contains(""));      // true (empty string is always contained)
// expect_stdout: true

let empty_s = "";
print(empty_s.find("a"));         // nil
// expect_stdout: nil
print(empty_s.find(""));          // 0
// expect_stdout: 0
print(empty_s.starts_with("a"));  // false
// expect_stdout: false
print(empty_s.starts_with(""));   // true
// expect_stdout: true
print(empty_s.contains("a"));     // false
// expect_stdout: false
print(empty_s.contains(""));      // true
// expect_stdout: true

let s_unicode = "你好世界";
print(s_unicode.find("好"));   // 1
// expect_stdout: 1
print(s_unicode.starts_with("你好")); // true
// expect_stdout: true
print(s_unicode.contains("世界")); // true
// expect_stdout: true
print(s_unicode.find("a")); // nil
// expect_stdout: nil
