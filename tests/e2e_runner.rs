// Allow dead code for now, as the runner is under development
#![allow(dead_code)]

use easyscript_rs::interpreter::Interpreter;
use easyscript_rs::lexer::Lexer;
use easyscript_rs::parser::Parser;
use easyscript_rs::value::Value;

use glob::glob;
use std::fs;
use std::path::PathBuf;

// Represents the expected outcome of a test case.
// We'll only support Value expectations for now.
#[derive(Debug)]
struct Expectation {
    value: String,
}

// A helper function to parse the test file.
// It separates the code from the expectation comment.
fn parse_test_file(source: &str) -> (String, Option<Expectation>) {
    let mut code = String::new();
    let mut expectation = None;

    for line in source.lines() {
        if let Some(stripped) = line.strip_prefix("// expect:") {
            expectation = Some(Expectation {
                value: stripped.trim().to_string(),
            });
            // Stop at the expectation line, don't add it to the code
            break; 
        }
        code.push_str(line);
        code.push('\n');
    }

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

    // An E2E test file must have an expectation.
    let expectation = expectation.expect("Test file must have an '// expect: ...' comment.");

    // 1. Lexer
    let lexer = Lexer::new(&code);
    let (tokens, lexer_errors) = lexer.scan_tokens();
    if !lexer_errors.is_empty() {
        panic!("\nLexer errors in {:?}:\n{:?}", path, lexer_errors);
    }

    // 2. Parser
    let parser = Parser::new(tokens);
    let (ast, parser_errors) = parser.parse();
    if !parser_errors.is_empty() {
        panic!("\nParser errors in {:?}:\n{:?}", path, parser_errors);
    }

    // 3. Interpreter
    let mut interpreter = Interpreter::new();
    match interpreter.run(&ast) {
        Ok(value) => {
            // Compare the Display format of the value with the expectation.
            let actual_value_str = format!("{}", value);
            println!("   Success: resulted in value: {}", actual_value_str);
            assert_eq!(actual_value_str, expectation.value);
        }
        Err(e) => {
            // For now, runtime errors cause a panic.
            // Later we can add '// expect runtime error:'
            panic!("\nExpected value '{}' but got runtime error in {:?}:\n{}", expectation.value, path, e);
        }
    }
}
