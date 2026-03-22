use std::process::Command;

fn tendermule() -> Command {
    Command::new(env!("CARGO_BIN_EXE_tendermule"))
}

#[test]
fn test_success_exits_zero() {
    let status = tendermule().args(["--count", "1"]).status().unwrap();
    assert!(status.success());
}

#[test]
fn test_error_exits_nonzero() {
    // Prefix longer than 5 chars triggers a generate_ids error
    let status = tendermule()
        .args(["--prefix", "toolong"])
        .status()
        .unwrap();
    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}
