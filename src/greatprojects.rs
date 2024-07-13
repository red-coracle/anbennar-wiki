use std::collections::BTreeMap;
use std::fs;

use jomini::{DeserializeError, Scalar, TextTape, Windows1252Encoding};
use jomini::text::ObjectReader;
use serde::Deserialize;

use crate::graphics::SpriteType;

#[derive(Debug, Default)]
pub struct GreatProject {
    id: String,
    start: Option<u64>,
    starting_tier: Option<u64>,
    project_type: String,
    sprite: Option<SpriteType>,
    // on_built
    // on_destroyed
    // can_be_moved
    // starting_tier
    // can_use_modifiers_trigger
    // can_upgrade_trigger
    // keep_trigger
    //tier_0: Option<Tier>, TODO: is this ever used?
    tier_1: Option<Tier>,
    tier_2: Option<Tier>,
    tier_3: Option<Tier>,
}

#[derive(Debug, Default)]
pub struct Tier {
    upgrade_time: Option<u64>,
    cost_to_upgrade: Option<u64>,
    province_modifiers: BTreeMap<String, Vec<u8>>,
    area_modifier: BTreeMap<String, Vec<u8>>,
    region_modifier: BTreeMap<String, Vec<u8>>,
    country_modifiers: BTreeMap<String, Vec<u8>>,
    // TODO: on_upgraded
}

pub fn parse_great_projects(data: &[u8]) -> Vec<GreatProject> {
    let mut gps = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();
    for (key, _op, value) in reader.fields() {
        let mut gp = GreatProject::default();
        gp.id = key.read_string();
        if let Ok(value) = value.read_object() {
            for (key, _op, value) in value.fields() {
                let key = key.read_str();
                match key.as_ref() {
                    "start" => {
                        if let Ok(start) = value.read_scalar() {
                            if let Ok(start) = start.to_u64() {
                                gp.start = Some(start);
                            }
                        }
                    },
                    "starting_tier" => {
                        if let Ok(starting_tier) = value.read_scalar() {
                            if let Ok(starting_tier) = starting_tier.to_u64() {
                                gp.starting_tier = Some(starting_tier);
                            }
                        }
                    }
                    "type" => {
                        if let Ok(project_type) = value.read_str() {
                            gp.project_type = project_type.as_ref().to_string();
                        }
                    }
                    "tier_1" => {
                        gp.tier_1 = Some(parse_tier(value.read_object()));
                    }
                    "tier_2" => {
                        gp.tier_2 = Some(parse_tier(value.read_object()));
                    }
                    "tier_3" => {
                        gp.tier_3 = Some(parse_tier(value.read_object()));
                    }
                    _ => {}
                }
            }
        }
        gps.push(gp);
    }

    gps
}

fn parse_tier(value: Result<ObjectReader<Windows1252Encoding>, DeserializeError>) -> Tier {
    let mut tier = Tier::default();
    for (key, _op, value) in value.unwrap().fields() {
        let key = key.read_str();
        if let Ok(value) = value.read_object() {
            match key.as_ref() {
                "upgrade_time" => {
                    let modifiers = extract_modifiers(value);
                    if let Some(months) = modifiers.get("months") {
                        tier.upgrade_time = Some(Scalar::new(months).to_u64().unwrap());
                    }
                }
                "cost_to_upgrade" => {
                    let modifiers = extract_modifiers(value);
                    if let Some(months) = modifiers.get("factor") {
                        tier.cost_to_upgrade = Some(Scalar::new(months).to_u64().unwrap());
                    }
                }
                "province_modifiers" => {
                    tier.province_modifiers = extract_modifiers(value);
                },
                "area_modifier" => {
                    tier.area_modifier = extract_modifiers(value);
                },
                "region_modifier" => {
                    tier.region_modifier = extract_modifiers(value);
                },
                "country_modifiers" => {
                    tier.country_modifiers = extract_modifiers(value);
                },
                _ => {}
            }
        }

    }
    tier
}

fn extract_modifiers(value: ObjectReader<Windows1252Encoding>) -> BTreeMap<String, Vec<u8>> {
    let mut map = BTreeMap::new();
    for (key, _op, value) in value.fields() {
        map.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
    }
    map
}

pub fn parse_all_great_projects() -> Vec<GreatProject> {
    let mut gps = vec![];
    let paths = fs::read_dir("./anbennar/common/great_projects").expect("Missing great projects directory");
    for path in paths {
        match path {
            Ok(file) => {
                let data = fs::read(file.path()).expect("error reading file");
                let parsed = parse_great_projects(data.as_slice());
                gps.extend(parsed);
            }
            _ => {}
        }
    }

    gps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_great_projects() {
        let projects = parse_all_great_projects();
        for p in projects {
            // println!("{:?}", p);
        }
    }
}