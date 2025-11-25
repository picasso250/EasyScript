// Allow dead code for now, as the runner is under development
#![allow(dead_code)]

use easyscript_rs::interpreter::Interpreter;
use easyscript_rs::lexer::Lexer;
use easyscript_rs::parser::Parser;
use gag::BufferRedirect;
use glob::glob;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

// Represents the expected outcome of a test case.
#[derive(Debug, Default)]
struct Expectation {
    value: Option<String>,
    stdout: Option<String>,
}

// A helper function to parse the test file.
// It separates the code from expectation comments.
// Expectation comments are expected to be at the end of the file.
fn parse_test_file(source: &str) -> (String, Expectation) {
    let mut code_lines = Vec::new();
    let mut value_expectation: Option<String> = None;
    let mut stdout_expectations: Vec<String> = Vec::new();

    for line in source.lines() {
        if let Some(val) = line.strip_prefix("// expect:") {
            value_expectation = Some(val.trim().to_string());
        } else if let Some(stdout_val) = line.strip_prefix("// expect_stdout:") {
            stdout_expectations.push(stdout_val.trim().to_string());
        } else {
            code_lines.push(line);
        }
    }

    let code = code_lines.join("\n");

    let final_stdout_exp = if !stdout_expectations.is_empty() {
        // Add a trailing newline to match the behavior of println!
        Some(stdout_expectations.join("\n") + "\n")
    } else {
        None
    };

    let expectation = Expectation {
        value: value_expectation,
        stdout: final_stdout_exp,
    };

    (code, expectation)
}

#[test]
fn run_e2e_tests() {
    let mut tests_run = 0;
    for entry in glob("tests/e2e/**/*.es").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                run_test_file(path);
                tests_run += 1;
            }
            Err(e) => println!("Glob error: {:?}", e),
        }
    }
    // Ensure that our glob pattern is actually finding tests.
    assert!(tests_run > 0, "No test files were found!");
}

fn run_test_file(path: PathBuf) {
    println!("-> Running test file: {:?}", path.display());

    let source = fs::read_to_string(&path).expect("Failed to read test file");
    let (code, expectation) = parse_test_file(&source);

    // An E2E test file must have at least one expectation.
    if expectation.value.is_none() && expectation.stdout.is_none() {
        panic!("Test file {:?} must have at least one '// expect: ...' or '// expect_stdout: ...' comment.", path);
    }

    // 1. Lexer
    let tokens = match Lexer::new(&code).scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            panic!("\nLexer errors in {:?}:\n{}", path, e);
        }
    };

    // 2. Parser
    let ast = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(e) => {
            panic!("\nParser errors in {:?}:\n{}", path, e);
        }
    };

    // 3. Interpreter
    // Capture stdout during interpretation
    let mut buf = BufferRedirect::stdout().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.run(&ast);

    // Read captured stdout
    let mut captured_stdout = String::new();
    buf.read_to_string(&mut captured_stdout).unwrap();
    // Drop the buffer to restore stdout
    drop(buf);

    // On Windows, captured stdout has \r\n, so normalize to \n
    let captured_stdout = captured_stdout.replace("\r\n", "\n");

    // Process result
    match result {
        Ok(value) => {
            let actual_value_str = format!("{}", value);

            if let Some(expected_value) = expectation.value {
                assert_eq!(
                    actual_value_str, expected_value,
                    "Value expectation mismatch for {:?}!", path
                );
            }

            if let Some(expected_stdout) = expectation.stdout {
                assert_eq!(
                    captured_stdout, expected_stdout,
                    "Stdout expectation mismatch for {:?}!", path
                );
            }
            println!("   PASS: {:?}", path.display());
        }
        Err(e) => {
            panic!(
                "\nExpected value '{:?}' and stdout '{:?}' but got runtime error in {:?}:\n{}",
                expectation.value, expectation.stdout, path, e
            );
        }
    }
}
