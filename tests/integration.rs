use std::{
    io::Write,
    process::{Command, Stdio},
};

fn process_input(input: &str) -> String {
    let mut command = Command::new("cargo");
    command
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = command.spawn().expect("Failed to start process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}

#[test]
fn test_main_integration_invalid_input() {
    let data = r#"[
        {"operation":"buy", "unitcost":12.00, "quantity": 100}, 
        {"operation":"sell", "unit-cost":15.00, "quantity": 50}, 
        {"operation":"sell", "unit-cost":15.00, "quantity": 50}
    ]"#;

    let data = data.replace('\n', "");

    let result = process_input(&data);
    assert!(result.contains("There was an error in the input"));
}

#[test]
fn test_main_integration_case_1() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10.00, "quantity": 100}, 
        {"operation":"sell", "unit-cost":15.00, "quantity": 50}, 
        {"operation":"sell", "unit-cost":15.00, "quantity": 50}
    ]"#;

    let data = data.replace('\n', "");
    let output = process_input(&data);

    let output_expected = r#"{"tax":0},{"tax":0},{"tax":0}"#;
    assert!(output.contains(output_expected));
}

#[test]
fn test_main_integration_case_2() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10, "quantity": 10000}, 
        {"operation":"sell", "unit-cost":20, "quantity": 5000}, 
        {"operation":"sell", "unit-cost":5, "quantity": 5000}
    ]"#;

    let data = data.replace('\n', "");
    let output_expected = r#"[{"tax":0},{"tax":10000},{"tax":0}]"#;

    let output = process_input(&data);
    assert!(output.contains(output_expected));
}

#[test]
fn test_main_integration_case_3() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10, "quantity": 10000}, 
        {"operation":"sell", "unit-cost":5, "quantity": 5000}, 
        {"operation":"sell", "unit-cost":20, "quantity": 5000}
    ]"#;

    let data = data.replace('\n', "");
    let output = process_input(&data);

    let output_expected = r#"[{"tax":0},{"tax":0},{"tax":5000}]"#;
    assert!(output.contains(output_expected));
}

#[test]
fn test_main_integration_case_4() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10, "quantity": 10000}, 
        {"operation":"buy", "unit-cost":25, "quantity": 5000}, 
        {"operation":"sell", "unit-cost":15, "quantity": 10000}
    ]"#;

    let data = data.replace('\n', "");
    let output = process_input(&data);

    let output_expected = r#"[{"tax":0},{"tax":0},{"tax":0}]"#;
    assert!(output.contains(output_expected));
}
#[test]
fn test_main_integration_case_5() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10, "quantity": 10000},
        {"operation":"buy", "unit-cost":25, "quantity": 5000},
        {"operation":"sell", "unit-cost":15, "quantity": 10000},
        {"operation":"sell", "unit-cost":25, "quantity": 5000}
    ]"#;

    let data = data.replace('\n', "");
    let output = process_input(&data);

    let output_expected = r#"[{"tax":0},{"tax":0},{"tax":0},{"tax":10000}]"#;
    assert!(output.contains(output_expected));
}
#[test]
fn test_main_integration_case_6() {
    let data = r#"[
        {"operation":"buy", "unit-cost":10, "quantity": 10000},
        {"operation":"sell", "unit-cost":2, "quantity": 5000},
        {"operation":"sell", "unit-cost":20, "quantity": 2000},
        {"operation":"sell", "unit-cost":20, "quantity": 2000},
        {"operation":"sell", "unit-cost":25, "quantity": 1000}
    ]"#;

    let data = data.replace('\n', "");
    let output = process_input(&data);

    let output_expected = r#"[{"tax":0},{"tax":0},{"tax":0},{"tax":0},{"tax":3000}]"#;
    assert!(output.contains(output_expected));
}
