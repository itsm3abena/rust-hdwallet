use serde_json::Value;

use super::conftest::{cli_tester, json_usize, TestData};

#[test]
fn test_cli_entropy() {
    let data = TestData::load();
    let entropies = data.entropies.as_object().expect("entropies object");

    for (client, strengths) in entropies {
        let strengths = strengths.as_object().expect("strength object");
        for (strength, entry) in strengths {
            let output = cli_tester(&[
                "generate",
                "entropy",
                "--client",
                client,
                "--strength",
                strength,
            ])
            .unwrap_or_else(|code| panic!("client={client} strength={strength} exit={code}"));

            let parsed: Value = serde_json::from_str(&output)
                .unwrap_or_else(|error| panic!("invalid json for {client}/{strength}: {error}"));
            assert_eq!(parsed["client"], *client);
            assert_eq!(json_usize(&parsed["strength"]), strength.parse::<usize>().unwrap());

            let entropy_field = parsed["entropy"].as_str().expect("entropy string");
            if client == "Electrum-V2" {
                let expected = entry["entropy-not-suitable"].as_str().expect("entropy-not-suitable");
                assert_eq!(entropy_field.len(), expected.len());
            } else {
                let expected = entry["entropy"].as_str().expect("entropy");
                assert_eq!(entropy_field.len(), expected.len());
            }
        }
    }
}
