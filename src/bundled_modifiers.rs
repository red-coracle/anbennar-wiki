use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;
use jomini::TextTape;
use crate::localisation::{Localisations, parse_all_localisations};
use crate::modifiers::{debug_modifiers, get_modifier};
use crate::religions::ReligiousGroup;
use crate::utils::gather;

#[derive(Debug, Default)]
pub struct BundledModifier {
    pub id: String,
    pub picture: Option<String>,
    pub name: Option<String>,
    pub modifiers: BTreeMap<String, Vec<u8>>,
    pub called_by: BTreeMap<String, String> // event, mission, on_action
}

fn parse_modifier_file(data: &[u8], localisations: Option<&HashMap<String, String>>) -> Vec<BundledModifier> {
    let mut bundled_modifiers = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();

    for (key, _op, value) in reader.fields() {
        let mut bundled_modifier = BundledModifier::default();
        bundled_modifier.id = key.read_string().to_string();
        if localisations.is_some() {
            let localisations = localisations.unwrap();
            let key = key.read_str();
            let key = key.as_ref();
            bundled_modifier.name = localisations.get(key).cloned();
        }
        if let Ok(value) = value.read_object() {
            for (key, _op, value) in value.fields() {
                match key.read_str().as_ref() {
                    "picture" => {
                        let value = value.read_string();
                        if value.is_ok() {
                            bundled_modifier.picture = Some(value.unwrap().to_lowercase());
                        }
                    },
                    "trigger" => {},
                    "potential" => {},
                    _ => {
                        // debug_modifiers(&key, &value);
                        let modifier = get_modifier(&key.read_string());
                        if modifier.is_some() {
                            let value = value.read_scalar().unwrap().as_bytes().to_vec();
                            bundled_modifier.modifiers.insert(key.read_string(), value);
                        }
                    }
                }
            }
        }
        bundled_modifiers.push(bundled_modifier);
    }

    bundled_modifiers
}

pub fn parse_bundled_modifiers(localisations:  Option<&HashMap<String, String>>) -> Vec<BundledModifier> {
    let mut bundled_modifiers = vec![];
    let mut files: Vec<PathBuf> = vec![];
    let paths = vec![
        "./anbennar/common/event_modifiers",
        "./anbennar/common/static_modifiers",
        // "./anbennar/common/triggered_modifiers" // Achievements have to be handled differently
    ];

    for path in paths {
        gather(path.to_string(), &mut files);
    }

    for file in files {
        if file.exists() {
            let data = fs::read(file.as_path()).expect("error reading file");
            let parsed = parse_modifier_file(data.as_slice(), localisations);
            bundled_modifiers.extend(parsed);
        }
    }

    bundled_modifiers
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    pub fn test_event_modifiers_parse() {
        let paths = fs::read_dir("./anbennar/common/event_modifiers").expect("Missing directory");
        let localisations = parse_all_localisations();
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let bundled_modifiers = parse_modifier_file(data.as_slice(), Some(&localisations));
                    for bundled_modifier in bundled_modifiers {
                        assert!(!bundled_modifier.id.is_empty());
                        //println!("{:?}", bundled_modifier); // is too much for stdout
                        //println!("{:?}", bundled_modifier.picture)
                        /*for (_, modifier) in bundled_modifier.modifiers {
                            // assert!(!modifier.id.is_empty());
                            // println!("{:?}", modifier);
                        }*/
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    pub fn test_static_modifiers_parse() {
        let paths = fs::read_dir("./anbennar/common/static_modifiers").expect("Missing directory");
        let localisations = parse_all_localisations();
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let bundled_modifiers = parse_modifier_file(data.as_slice(), Some(&localisations));
                    for bundled_modifier in bundled_modifiers {
                        assert!(!bundled_modifier.id.is_empty());
                        // println!("WTF");
                        println!("{:?}", bundled_modifier);
                        /*for (_, modifier) in bundled_modifier.modifiers {
                            // assert!(!modifier.id.is_empty());
                            // println!("{:?}", modifier);
                        }*/
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    pub fn test_triggered_modifiers_parse() {
        let paths = fs::read_dir("./anbennar/common/triggered_modifiers").expect("Missing directory");
        let localisations = parse_all_localisations();
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let bundled_modifiers = parse_modifier_file(data.as_slice(), Some(&localisations));
                    for bundled_modifier in bundled_modifiers {
                        assert!(!bundled_modifier.id.is_empty());
                        println!("{:?}", bundled_modifier);
                        /*for (_, modifier) in bundled_modifier.modifiers {
                            // assert!(!modifier.id.is_empty());
                            // println!("{:?}", modifier);
                        }*/
                    }
                }
                _ => {}
            }
        }
    }
}
