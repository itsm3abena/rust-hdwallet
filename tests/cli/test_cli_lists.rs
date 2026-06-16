use super::conftest::{cli_tester, TestData};

fn normalize_cli_output(output: &str) -> String {
    output.replace("\r\n", "\n").trim().to_string()
}

#[test]
fn test_cryptocurrencies_list() {
    let output = cli_tester(&["list", "c"]).expect("list cryptocurrencies");
    assert_eq!(
        normalize_cli_output(&output),
        normalize_cli_output(&TestData::raw("cryptocurrencies"))
    );
}

#[test]
fn test_languages_list() {
    let output = cli_tester(&["list", "l"]).expect("list languages");
    assert_eq!(
        normalize_cli_output(&output),
        normalize_cli_output(&TestData::raw("languages"))
    );
}

#[test]
fn test_strengths_list() {
    let output = cli_tester(&["list", "s"]).expect("list strengths");
    assert_eq!(
        normalize_cli_output(&output),
        normalize_cli_output(&TestData::raw("strengths"))
    );
}
