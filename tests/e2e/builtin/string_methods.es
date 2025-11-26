# tests/e2e/string_methods.es

let s = "hello world";
print(s.find("hello")); # 0
# expect_stdout: 0
print(s.find("world")); # 6
# expect_stdout: 6
print(s.find("o"));     # 4 (first 'o')
# expect_stdout: 4
print(s.find("xyz"));   # nil
# expect_stdout: nil
print(s.find(""));      # 0 (empty string always found at start)
# expect_stdout: 0

print(s.starts_with("hello")); # true
# expect_stdout: true
print(s.starts_with("world")); # false
# expect_stdout: false
print(s.starts_with(""));      # true
# expect_stdout: true
print(s.starts_with("h"));     # true
# expect_stdout: true
print(s.starts_with("xyz"));   # false
# expect_stdout: false

print(s.contains("hello")); # true
# expect_stdout: true
print(s.contains("world")); # true
# expect_stdout: true
print(s.contains("o"));     # true
# expect_stdout: true
print(s.contains("xyz"));   # false
# expect_stdout: false
print(s.contains(""));      # true (empty string is always contained)
# expect_stdout: true

let empty_s = "";
print(empty_s.find("a"));         # nil
# expect_stdout: nil
print(empty_s.find(""));          # 0
# expect_stdout: 0
print(empty_s.starts_with("a"));  # false
# expect_stdout: false
print(empty_s.starts_with(""));   # true
# expect_stdout: true
print(empty_s.contains("a"));     # false
# expect_stdout: false
print(empty_s.contains(""));      # true
# expect_stdout: true

let s_unicode = "你好世界";
print(s_unicode.find("好"));   # 1
# expect_stdout: 1
print(s_unicode.starts_with("你好")); # true
# expect_stdout: true
print(s_unicode.contains("世界")); # true
# expect_stdout: true

# Test str.ends_with()
let test_str_ew = "Hello World";
print(test_str_ew.ends_with("World"));   # Expected: true
# expect_stdout: true
print(test_str_ew.ends_with("world"));   # Expected: false (case-sensitive)
# expect_stdout: false
print(test_str_ew.ends_with("ld"));      # Expected: true
# expect_stdout: true
print(test_str_ew.ends_with("Hello"));   # Expected: false
# expect_stdout: false
print(test_str_ew.ends_with(""));        # Expected: true
# expect_stdout: true
print(test_str_ew.ends_with("Foo"));     # Expected: false
# expect_stdout: false
print("".ends_with(""));                 # Expected: true
# expect_stdout: true
print("".ends_with("a"));                # Expected: false
# expect_stdout: false


# Test str.substring()
let test_str_sub = "Hello World";
print(test_str_sub.substring(0, 5));  # Expected: Hello
# expect_stdout: Hello
print(test_str_sub.substring(6));     # Expected: World
# expect_stdout: World
print(test_str_sub.substring(0, 11)); # Expected: Hello World
# expect_stdout: Hello World
print(test_str_sub.substring(6, 11)); # Expected: World
# expect_stdout: World
print(test_str_sub.substring(0, 0));  # Expected:
# expect_stdout: 
print(test_str_sub.substring(7, 5));  # Expected:
# expect_stdout: 
print(test_str_sub.substring(100));   # Expected:
# expect_stdout: 
print(test_str_sub.substring(0, 100)); # Expected: Hello World
# expect_stdout: Hello World
print("Unicode 世界".substring(8, 10)); # Expected: 世界
# expect_stdout: 世界