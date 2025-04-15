use std::collections::{BTreeMap, HashMap};
use std::fs;

use jomini::{Scalar, TextTape};

use crate::localisation::parse_all_localisations;
use crate::modifiers::get_modifier;
use crate::utils::jsonify;

#[derive(Debug, Default)]
pub struct Government {
    pub id: String,
    pub basic_reform: String,
    pub color: Vec<u8>,
    pub reform_levels: BTreeMap<u8, ReformLevel>
}

#[derive(Debug, Default)]
pub struct ReformLevel {
    pub id: String,
    pub reforms: Vec<String>,
}

#[derive(Debug, Default)]
pub struct GovernmentReform {
    pub id: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub icon: Option<String>,
    pub potential: Option<String>,
    // pub trigger
    // pub conditional
    pub modifiers: BTreeMap<String, Vec<u8>>, // lifetime problems on scalar
    // pub effect
    // pub removed_effect
    // pub custom_attributes

    pub basic_reform: Option<bool>,
    pub monarchy: Option<bool>
    // etc.
}

pub fn parse_government(data: &[u8]) -> Vec<Government> {
    let mut governments = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();

    for (key, _op, value) in reader.fields() {
        let mut government = Government::default();
        government.id = key.read_str().to_string();
        if let Ok(value) = value.read_object() {
            for (key, _op, value) in value.fields() {
                let key = key.read_str();
                let key = key.as_ref();
                match key {
                    "reform_levels" => {
                        if let Ok(value) = value.read_object() {
                            let mut tier = 1;
                            for (key, _op, value) in value.fields() {
                                let mut reform_level = ReformLevel::default();
                                reform_level.id = key.read_string();
                                if let Ok(value) = value.read_object() {
                                    for (key, _op, value) in value.fields() {
                                        let key = key.read_str();
                                        let key = key.as_ref();
                                        match key {
                                            "reforms" => {
                                                if let Ok(v) = value.read_array() {
                                                    for value in v.values() {
                                                        reform_level.reforms.push(value.read_string().unwrap());
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                government.reform_levels.insert(tier, reform_level);
                                tier += 1;
                            }
                        }
                    }
                    "basic_reform" => {}
                    "legacy_government" => {}
                    "exclusive_reforms" => {}
                    "color" => {}
                    _ => {}
                }
            }
        }

        governments.push(government);
    }

    governments
}

pub fn parse_governments() -> Vec<Government> {
    let mut governments = vec![];
    let paths = fs::read_dir("./anbennar/common/governments").expect("Missing governments directory");
    for path in paths {
        match path {
            Ok(file) => {
                let data = fs::read(file.path()).expect("error reading file");
                let parsed = parse_government(data.as_slice());
                governments.extend(parsed);
            }
            _ => {}
        }
    }

    governments
}

pub fn parse_government_reform_file(data: &[u8], localisations: Option<&HashMap<String, String>>) -> Vec<GovernmentReform> {
    let mut reforms = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();

    for (key, _op, value) in reader.fields() {
        let key = key.read_str();
        let key = key.as_ref();
        match key {
            "defaults_reform" => {}
            key => {
                let mut reform = GovernmentReform{id: key.to_string(), ..Default::default()};
                if localisations.is_some() {
                    let localisations = localisations.unwrap();
                    reform.name = localisations.get(key).cloned();
                    reform.desc = localisations.get(&format!("{key}_desc").to_string()).cloned();
                }
                if let Ok(value) = value.read_object() {
                    for (key, _op, value) in value.fields() {
                        let key = key.read_str();
                        let key = key.as_ref();
                        match key {
                            "icon" => {
                                let value = value.read_string();
                                if value.is_ok() {
                                    reform.icon = Some(value.unwrap());
                                }
                            }
                            "modifiers" => {
                                let value = value.read_object();
                                if value.is_ok() {
                                    for (key, _op, value) in value.unwrap().fields() {
                                        let modifier = get_modifier(&key.read_string(), );
                                        if modifier.is_some() {
                                            reform.modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                        }
                                    }
                                }
                            }
                            "potential" => {
                                reform.potential = Some(jsonify(value))
                            }
                            _ => {}
                        }
                    }
                }
                reforms.push(reform);
            }
        }
    }

    reforms
}

pub fn parse_government_reforms(localisations: Option<&HashMap<String, String>>) -> Vec<GovernmentReform> {
    let mut reforms = vec![];
    let paths = fs::read_dir("./anbennar/common/government_reforms").expect("Missing government reforms directory");
    for path in paths {
        match path {
            Ok(file) => {
                let data = fs::read(file.path()).expect("error reading file");
                let parsed = parse_government_reform_file(data.as_slice(), localisations);
                reforms.extend(parsed);
            }
            _ => {}
        }
    }

    reforms
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    pub fn test_governments_parse() {
        let paths = fs::read_dir("./anbennar/common/governments").expect("Missing governments directory");
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let governments = parse_government(data.as_slice());
                }
                _ => {}
            }
        }
    }

    #[test]
    pub fn test_government_reform_parse() {
        let paths = fs::read_dir("./anbennar/common/government_reforms").expect("Missing government reforms directory");
        let localisations = parse_all_localisations();
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let reforms = parse_government_reform_file(data.as_slice(), Some(&localisations));
                    for reform in reforms {
                        assert!(!reform.id.is_empty());
                        for (_, modifier) in reform.modifiers {
                            Scalar::new(modifier.as_slice());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}