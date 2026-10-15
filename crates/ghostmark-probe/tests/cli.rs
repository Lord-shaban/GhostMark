use std::{fs::File, process::Command};

#[test]
fn output_errors_are_reported_without_panicking() {
    for argument in ["inspect", "--help"] {
        let result = Command::new(env!("CARGO_BIN_EXE_ghostmark-probe"))
            .arg(argument)
            // A read-only handle rejects writes without modifying the executable.
            .stdout(File::open(env!("CARGO_BIN_EXE_ghostmark-probe")).unwrap())
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1), "{argument}: {result:?}");
        assert!(!result.stderr.is_empty());
        assert!(!String::from_utf8_lossy(&result.stderr).contains("panicked"));
    }
}

#[test]
fn inspection_is_valid_json_and_cannot_claim_a_verified_build() {
    let result = Command::new(env!("CARGO_BIN_EXE_ghostmark-probe"))
        .arg("inspect")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["mutation_capable"], false);
    assert_ne!(report["compatibility"]["status"], "supported");
    let environment = report["environment"].as_object().unwrap();
    for disallowed in [
        "product_key",
        "machine_name",
        "user_name",
        "hardware_id",
        "process_list",
    ] {
        assert!(!environment.contains_key(disallowed));
    }
}

#[test]
fn mutation_commands_and_extra_arguments_are_rejected() {
    for arguments in [
        vec!["hide"],
        vec!["restore"],
        vec!["inspect", "--apply"],
        vec![],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_ghostmark-probe"))
            .args(arguments)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2));
        assert!(result.stdout.is_empty());
    }
}
