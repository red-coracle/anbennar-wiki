use std::collections::HashSet;
use std::fs;

use jomini::{TextTape, Windows1252Encoding};
use jomini::text::ValueReader;

#[derive(Debug, Default)]
pub struct MissionTree {
    pub generic: bool,
    pub ai: bool,
    pub has_country_shield: bool,
    pub slot: Option<u64>,
    pub missions: Vec<Mission>,
}

#[derive(Debug, Default)]
pub struct Mission {
    pub id: String,
    pub icon: Option<String>,
    pub position: Option<u64>,
    pub completed_by: Option<String>,
    pub required_missions: Vec<String>,
    pub trigger: Option<String>,
    pub effect: Option<String>
}

pub fn parse_mission_file(data: &[u8]) -> Vec<MissionTree> {
    let mut missions = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();

    for (_key, _op, value) in reader.fields() {
        let mut tree = MissionTree{
            generic: false,
            ai: false,
            ..Default::default()
        };
        if let Ok(mission_tree) = value.read_object() {
            for (key, _op, value) in mission_tree.fields() {
                let key = key.read_str();
                let key = key.as_ref();
                match key {
                    "generic" => {
                        let value = value.read_string().unwrap();
                        if value == "yes" {
                            tree.generic = true;
                        }
                    }
                    "ai" => {
                        let value = value.read_string().unwrap();
                        if value == "yes" {
                            tree.ai = true;
                        }
                    }
                    "slot" => {
                        let value = value.read_scalar().expect("Unable to parse slot as scalar");
                        tree.slot = Some(value.to_u64().expect("Unable to parse scalar as u64"));
                    }
                    "has_country_shield" => {
                        let value = value.read_string().unwrap();
                        if value == "yes" {
                            tree.has_country_shield = true;
                        }
                    }
                    "potential" => {}
                    "potential_on_load" => {}
                    _ => {
                        // should be a mission
                        let mut is_a_mission = false;
                        let mut mission = Mission{..Default::default()};
                        if let Ok(value) = value.read_object() {
                            for (key, _op, value) in value.fields() {
                                let key = key.read_str();
                                let key = key.as_ref();
                                match key {
                                    "icon" => {
                                        mission.icon = Some(value.read_string().expect("Unable to parse mission icon as string"));
                                    }
                                    "position" => {
                                        is_a_mission = true;
                                        let value = value.read_scalar().expect("Unable to parse position as scalar");
                                        mission.position = Some(value.to_u64().expect("Unable to parse mission position as u64"));
                                    }
                                    "completed_by" => {}
                                    "required_missions" => {
                                        if let Ok(value) = value.read_array() {
                                            for v in value.values() {
                                                if let Ok(name) = v.read_string() {
                                                    mission.required_missions.push(name);
                                                }
                                            }
                                        }
                                    }
                                    "provinces_to_highlight" => {} // object
                                    "trigger" => {
                                        is_a_mission = true;
                                        // let value = value.read_object().unwrap();
                                    }
                                    "effect" => {}
                                    _ => {}
                                }
                            }
                        }
                        if is_a_mission {
                            mission.id = key.to_string();
                            tree.missions.push(mission);
                        }
                    }
                }
            }
        }
        missions.push(tree);
    }

    missions
}

pub fn tags_with_missions() -> HashSet<String> {
    let mut tags = HashSet::new();
    let paths = fs::read_dir("./anbennar/missions").expect("Missing missions directory");

    fn recursively_find_tags(mut tags: HashSet<String>, obj: ValueReader<Windows1252Encoding>) -> HashSet<String> {
        if let Ok(inner) = obj.read_object() {
            for (key, _op, value) in inner.fields() {
                if key.read_str() == "NOT" {
                    continue;
                }
                if key.read_str() == "tag" || key.read_str() == "was_tag" {
                    tags.insert(value.read_string().expect("could not parse as string"));
                } else {
                    tags = recursively_find_tags(tags.clone(), value);
                }
            }
        }
        tags
    }

    for path in paths {
        match path {
            Ok(file) => {
                let file = fs::read(file.path()).expect("error reading file");
                let tape = TextTape::from_slice(file.as_slice()).unwrap();
                let reader = tape.windows1252_reader();
                for (_key, _op, value) in reader.fields() {
                    if let Ok(mission_tree) = value.read_object() {
                        for (key, _op, value) in mission_tree.fields() {
                            let key = key.read_str();
                            if key == "potential" {
                                tags.extend(recursively_find_tags(HashSet::new(), value));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_mission_parse() {
        let data = include_bytes!("../anbennar/missions/Adenica_Missions.txt");
        let actual = parse_mission_file(data);
        for tree in actual {
            assert_eq!(tree.generic, false);
        }
    }

    #[test]
    pub fn test_all_missions_parse() {
        let paths = fs::read_dir("./anbennar/missions").expect("Missing missions directory");
        for path in paths {
            match path {
                Ok(file) => {
                    let data = fs::read(file.path()).expect("error reading file");
                    let trees = parse_mission_file(data.as_slice());
                    for tree in trees {
                        for mission in tree.missions {
                            assert!(!mission.id.is_empty())
                        }
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    pub fn test_tags_with_missions() {
        let actual = tags_with_missions();
        assert!(actual.contains(&"Z43".to_string()));
        assert!(actual.contains(&"U08".to_string()));
    }
}