use std::collections::HashSet;

use serde_json::Value;

use crate::hdwallet::cryptocurrencies::icryptocurrency::CryptocurrencyDescriptor;
use crate::hdwallet::derivations::{DerivationStep, IDerivation};
use crate::hdwallet::exceptions::Error;
use crate::hdwallet::hdwallet::{build_derivation, HDWallet};

impl HDWallet {
    pub fn dumps(&mut self, exclude: &HashSet<String>) -> Result<Option<Value>, Error> {
        let derivation_name = self.derivation_name().to_string();
        let cryptocurrency = self.cryptocurrency_descriptor();
        let (template_path, template_indexes, steps) = {
            let derivation = match self.derivation() {
                Some(derivation) => derivation,
                None => return Ok(None),
            };
            (
                derivation.path().to_string(),
                derivation.indexes().to_vec(),
                derivation.derivations().to_vec(),
            )
        };

        let mut derivations = Vec::new();

        drive_derivation_dumps(
            self,
            &derivation_name,
            cryptocurrency,
            &template_path,
            &template_indexes,
            &steps,
            Vec::new(),
            exclude,
            &mut derivations,
        )?;

        if exclude.contains("root") {
            return Ok(Some(Value::Array(derivations)));
        }

        let mut root_exclude = exclude.clone();
        root_exclude.insert("derivation".to_string());
        let mut root = self.dump(&root_exclude)?;

        if !exclude.contains("derivations") {
            if let Some(object) = root.as_object_mut() {
                object.insert("derivations".to_string(), Value::Array(derivations));
            }
        }

        Ok(Some(root))
    }
}

pub fn drive_derivation_dumps(
    wallet: &mut HDWallet,
    derivation_name: &str,
    cryptocurrency: &CryptocurrencyDescriptor,
    template_path: &str,
    template_indexes: &[u32],
    steps: &[DerivationStep],
    current: Vec<(u32, bool)>,
    exclude: &HashSet<String>,
    derivations: &mut Vec<Value>,
) -> Result<(), Error> {
    if steps.is_empty() {
        let derivation = rebuild_derivation(
            derivation_name,
            cryptocurrency,
            template_path,
            template_indexes,
            &current,
        )?;
        wallet.update_derivation(derivation_name, derivation)?;

        let mut dump_exclude = exclude.clone();
        dump_exclude.insert("root".to_string());
        derivations.push(wallet.dump(&dump_exclude)?);
        return Ok(());
    }

    match steps[0] {
        DerivationStep::Range {
            from,
            to,
            hardened,
        } => {
            for value in from..=to {
                let mut next = current.clone();
                next.push((value, hardened));
                drive_derivation_dumps(
                    wallet,
                    derivation_name,
                    cryptocurrency,
                    template_path,
                    template_indexes,
                    &steps[1..],
                    next,
                    exclude,
                    derivations,
                )?;
            }
        }
        DerivationStep::Index { index, hardened } => {
            let mut next = current.clone();
            next.push((index, hardened));
            drive_derivation_dumps(
                wallet,
                derivation_name,
                cryptocurrency,
                template_path,
                template_indexes,
                &steps[1..],
                next,
                exclude,
                derivations,
            )?;
        }
    }

    Ok(())
}

pub fn rebuild_derivation(
    derivation_name: &str,
    cryptocurrency: &CryptocurrencyDescriptor,
    template_path: &str,
    template_indexes: &[u32],
    current: &[(u32, bool)],
) -> Result<Box<dyn IDerivation>, Error> {
    if current.is_empty() {
        return build_derivation(
            derivation_name,
            cryptocurrency,
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            Some(template_path),
            template_indexes,
        );
    }

    let index = |position: usize| current.get(position).map(|(value, _)| value.to_string());

    match derivation_name {
        "BIP44" | "BIP49" | "BIP84" | "BIP86" if current.len() >= 5 => build_derivation(
            derivation_name,
            cryptocurrency,
            &index(2).unwrap_or_else(|| "0".to_string()),
            &index(3).unwrap_or_else(|| "0".to_string()),
            &index(4).unwrap_or_else(|| "0".to_string()),
            "0",
            "0",
            "0",
            "0",
            None,
            &[],
        ),
        "CIP1852" if current.len() >= 5 => build_derivation(
            derivation_name,
            cryptocurrency,
            &index(2).unwrap_or_else(|| "0".to_string()),
            "0",
            &index(4).unwrap_or_else(|| "0".to_string()),
            &index(3).unwrap_or_else(|| "0".to_string()),
            "0",
            "0",
            "0",
            None,
            &[],
        ),
        "Electrum" if current.len() >= 2 => build_derivation(
            derivation_name,
            cryptocurrency,
            "0",
            &index(0).unwrap_or_else(|| "0".to_string()),
            &index(1).unwrap_or_else(|| "0".to_string()),
            "0",
            "0",
            "0",
            "0",
            None,
            &[],
        ),
        "Monero" if current.len() >= 2 => build_derivation(
            derivation_name,
            cryptocurrency,
            "0",
            "0",
            "0",
            "0",
            "0",
            &index(0).unwrap_or_else(|| "0".to_string()),
            &index(1).unwrap_or_else(|| "0".to_string()),
            None,
            &[],
        ),
        "HDW" if current.len() >= 3 => build_derivation(
            derivation_name,
            cryptocurrency,
            &index(0).unwrap_or_else(|| "0".to_string()),
            "0",
            &index(2).unwrap_or_else(|| "0".to_string()),
            "0",
            &index(1).unwrap_or_else(|| "0".to_string()),
            "0",
            "0",
            None,
            &[],
        ),
        _ => {
            let path = format!(
                "m/{}",
                current
                    .iter()
                    .map(|(index, hardened)| {
                        if *hardened {
                            format!("{index}'")
                        } else {
                            index.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("/")
            );
            build_derivation(
                "Custom",
                cryptocurrency,
                "0",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0",
                Some(&path),
                &[],
            )
        }
    }
}

pub fn dumps_json_output(dump: &Value) -> Result<String, Error> {
    if let Some(array) = dump.as_array() {
        let blocks = array
            .iter()
            .map(|value| {
                serde_json::to_string_pretty(value)
                    .map_err(|error| Error::new(&error.to_string(), None, None, None))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(blocks.join("\n"));
    }

    let mut blocks = Vec::new();
    if let Some(object) = dump.as_object() {
        let mut root = dump.clone();
        if let Some(root_object) = root.as_object_mut() {
            root_object.remove("derivations");
        }
        blocks.push(
            serde_json::to_string_pretty(&root)
                .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
        );

        if let Some(derivations) = object.get("derivations").and_then(Value::as_array) {
            for derivation in derivations {
                blocks.push(
                    serde_json::to_string_pretty(derivation).map_err(|error| {
                        Error::new(&error.to_string(), None, None, None)
                    })?,
                );
            }
        }
    }

    Ok(format!("{}\n", blocks.join("\n")))
}
