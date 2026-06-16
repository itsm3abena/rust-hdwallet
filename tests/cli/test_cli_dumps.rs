use serde_json::{Map, Value};

use super::conftest::{cli_tester, TestData};
use super::dump_rules::{rules, ArgSpec, HdRules, MethodRule};

struct DumpCase {
    dumps: Value,
    derivation: Value,
    hd: String,
    name: String,
}

fn unpack_dumps() -> Vec<DumpCase> {
    let data = TestData::load();
    let hdw_data = &data.hdwallet;
    let rule_map = rules();
    let mut all_data = Vec::new();

    for hd in rule_map.keys() {
        let hd_object = hdw_data[hd].as_object().expect("hd object");
        for (key, dumps) in hd_object {
            if key == "derivation" {
                continue;
            }
            if hd == &"Monero" && key == "from-private-key" {
                continue;
            }
            all_data.push(DumpCase {
                dumps: dumps.clone(),
                derivation: hd_object["derivation"].clone(),
                hd: (*hd).to_string(),
                name: key.clone(),
            });
        }
    }
    all_data
}

fn get_derivation_args(derivation: &Value) -> Vec<String> {
    let mut args = vec![
        "--derivation".to_string(),
        derivation["name"].as_str().expect("derivation name").to_string(),
    ];
    if let Some(derivation_args) = derivation["args"].as_object() {
        for (key, value) in derivation_args {
            args.push(format!("--{key}"));
            args.push(value.as_str().expect("derivation arg").to_string());
        }
    }
    args
}

fn get_hd_args(dumps: &Value, args: &[(&str, ArgSpec)]) -> Vec<String> {
    let mut args_ls = Vec::new();
    for (key, spec) in args {
        let arg = match spec {
            ArgSpec::Literal(value) => (*value).to_string(),
            ArgSpec::Field(field) => {
                let mut value = dumps[*field].as_str().expect("dump field").to_string();
                if *key == "language" {
                    value = value.to_lowercase();
                }
                value
            }
        };
        args_ls.push(format!("--{key}"));
        args_ls.push(arg);
    }
    args_ls
}

fn apply_null_fields(object: &mut Map<String, Value>, fields: &[&str]) {
    for field in fields {
        object.insert((*field).to_string(), Value::Null);
    }
}

fn get_dumps_output(ddict: &Value) -> String {
    let object = ddict.as_object().expect("dump object");
    let mut root_keys = Map::new();
    for (key, value) in object {
        if key != "derivations" {
            root_keys.insert(key.clone(), value.clone());
        }
    }
    let mut ret = format!("{}\n", serde_json::to_string_pretty(&Value::Object(root_keys)).unwrap());
    if let Some(derivations) = object.get("derivations").and_then(Value::as_array) {
        for derivation in derivations {
            ret.push_str(&format!("{}\n", serde_json::to_string_pretty(derivation).unwrap()));
        }
    }
    ret
}

fn build_final_dumps(dumps: &Value, rule: &MethodRule) -> Value {
    let mut final_dumps = dumps.clone();
    if let Some(object) = final_dumps.as_object_mut() {
        apply_null_fields(object, rule.root_changes);
    }
    if !rule.derivation_changes.is_empty() {
        if let Some(derivations) = final_dumps.get_mut("derivations").and_then(Value::as_array_mut) {
            for derivation in derivations {
                if let Some(object) = derivation.as_object_mut() {
                    apply_null_fields(object, rule.derivation_changes);
                }
            }
        }
    }
    final_dumps
}

fn build_dumps_args(
    dumps: &Value,
    derivation: &Value,
    hd: &str,
    name: &str,
    param: &str,
    rule: &MethodRule,
    hd_rules: &HdRules,
) -> Vec<String> {
    let mut args = vec![
        "dumps".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--symbol".to_string(),
        dumps["symbol"].as_str().expect("symbol").to_string(),
        "--hd".to_string(),
        hd.to_string(),
    ];
    args.extend(get_hd_args(dumps, hd_rules.args));

    if ["byron-icarus", "byron-ledger", "byron-legacy"].contains(&name) {
        args.extend(["--address-type".to_string(), "public-key".to_string()]);
    } else if ["shelley-icarus", "shelley-ledger"].contains(&name) {
        args.extend(["--address-type".to_string(), "staking".to_string()]);
    }

    if rule.derivable {
        args.extend(get_derivation_args(derivation));
        args.push(format!("--{param}"));
        args.push(dumps[rule.method].as_str().expect("derivable method").to_string());
    } else {
        let last = dumps["derivations"].as_array().expect("derivations").last().expect("last");
        args.push(format!("--{param}"));
        args.push(last[rule.method].as_str().expect("non-derivable method").to_string());
    }
    args
}

#[test]
fn test_cli_dumps() {
    let rule_map = rules();
    for case in unpack_dumps() {
        let hd_rules = rule_map.get(case.hd.as_str()).expect("hd rules");
        for (param, rule) in hd_rules.available_methods {
            let final_dumps = build_final_dumps(&case.dumps, rule);
            let args = build_dumps_args(
                &case.dumps,
                &case.derivation,
                &case.hd,
                &case.name,
                param,
                rule,
                hd_rules,
            );
            let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
            let output = cli_tester(&arg_refs).unwrap_or_else(|code| {
                panic!("dumps failed {} {} {} exit={code}", case.hd, case.name, param)
            });

            if rule.derivable {
                assert_eq!(
                    output,
                    get_dumps_output(&final_dumps),
                    "dumps mismatch {} {} {}",
                    case.hd,
                    case.name,
                    param
                );
            } else {
                assert_eq!(output.trim(), "", "non-derivable {} {} {}", case.hd, case.name, param);
            }

            let mut dump_args = args;
            dump_args[0] = "dump".to_string();
            dump_args.retain(|arg| arg != "--format" && arg != "json");

            let mut dump_expected = final_dumps.clone();
            if let Some(object) = dump_expected.as_object_mut() {
                let derivations = object.remove("derivations").and_then(|v| v.as_array().cloned()).expect("derivations");
                let mut last_derivation = derivations.last().cloned().expect("last derivation");
                if !rule.derivable {
                    if let Some(derivation_object) = last_derivation.as_object_mut() {
                        derivation_object.remove("at");
                    }
                }
                object.insert("derivation".to_string(), last_derivation);
            }

            let dump_refs: Vec<&str> = dump_args.iter().map(String::as_str).collect();
            let dump_output = cli_tester(&dump_refs).unwrap_or_else(|code| {
                panic!("dump failed {} {} {} exit={code}", case.hd, case.name, param)
            });
            let parsed: Value = serde_json::from_str(&dump_output).expect("dump json");
            assert_eq!(parsed, dump_expected, "dump mismatch {} {} {}", case.hd, case.name, param);
        }
    }
}
