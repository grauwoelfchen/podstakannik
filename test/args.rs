#[cfg(test)]
mod test {
    use std::process::Command;

    const BIN: &str = "podstakannik";

    #[test]
    fn test_run_with_unknown_args() {
        let output = Command::new(format!("./target/debug/{}", BIN))
            .arg("--foo")
            .output()
            .unwrap();
        assert!(String::from_utf8_lossy(&output.stderr).contains("error:"));
    }
}
