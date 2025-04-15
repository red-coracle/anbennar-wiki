use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

use jomini::text::ValueReader;
use jomini::{TextTape, Windows1252Encoding};

#[derive(Debug, Default)]
pub struct MissionTree {
    pub generic: bool,
    pub ai: bool,
    pub has_country_shield: bool,
    pub slot: Option<u64>,
    pub missions: Vec<Mission>,
    pub potential: Option<Trigger>
}

#[derive(Debug, Default)]
pub struct Mission {
    pub id: String,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub icon: Option<String>,
    pub position: Option<u64>,
    pub completed_by: Option<String>,
    pub required_missions: Vec<String>,
    pub trigger: Option<Trigger>,
    pub effect: Option<Effect>,
    pub provinces_to_highlight: Option<Trigger>
}

#[derive(Debug, Default)]
pub struct Trigger {
    pub misc: Option<String>,
    pub or: Option<Box<Trigger>>,
    pub and: Option<Box<Trigger>>,
    pub not: Option<Box<Trigger>>,
    pub custom_tooltip: Option<Box<Trigger>>,
    pub tooltip: Option<String>,
    pub tooltip_desc: Option<String>,
    pub tag: Option<String>,
    pub was_tag: Option<String>,
    pub primary_culture: Option<String>,
    pub religion: Option<String>,
    pub has_country_flag: Option<String>,
    pub dynasty: Option<String>
}

#[derive(Debug, Default)]
pub struct Effect {
    pub misc: Option<String>,
    pub custom_tooltip: Option<String>,
    pub country_events: Option<BTreeMap<String, CountryEvent>>,
    pub country_modifiers: Option<BTreeMap<String, CountryModifier>>,
    pub remove_country_modifiers: Option<BTreeMap<String, CountryModifier>>,
    pub set_country_flag: Option<BTreeMap<String, String>>,
    pub clr_country_flag: Option<BTreeMap<String, String>>
}

#[derive(Debug, Default)]
pub struct CountryModifier {
    pub name: String,
    pub duration: Option<i64>,
    pub desc: Option<String>,
    pub hidden: bool
}

#[derive(Debug, Default)]
pub struct CountryEvent {
    pub id: String,
    pub days: Option<u64>,
    pub random: Option<u64>,
    pub tooltip: Option<String>
}

pub fn parse_mission_file(data: &[u8], localisations: Option<&HashMap<String, String>>) -> Vec<MissionTree> {
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
                    "potential" => {
                        let mut trigger = Trigger::default();
                        parse_trigger(&mut trigger, &value, localisations);
                        tree.potential = Some(trigger);
                    }
                    "potential_on_load" => {}
                    _ => {
                        parse_mission(localisations, &mut tree, value, &key);
                    }
                }
            }
        }
        missions.push(tree);
    }

    missions
}

fn parse_mission(localisations: Option<&HashMap<String, String>>, tree: &mut MissionTree, value: ValueReader<Windows1252Encoding>, key: &&str) {
    let mut is_a_mission = false;
    let mut mission = Mission { ..Default::default() };
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
                "completed_by" => {todo!()}
                "required_missions" => {
                    if let Ok(value) = value.read_array() {
                        for v in value.values() {
                            if let Ok(name) = v.read_string() {
                                mission.required_missions.push(name);
                            }
                        }
                    }
                }
                "provinces_to_highlight" => {
                    let mut provinces_to_highlight = Trigger::default();
                    parse_trigger(&mut provinces_to_highlight, &value, localisations);
                    mission.provinces_to_highlight = Some(provinces_to_highlight);
                }
                "trigger" => {
                    is_a_mission = true;
                    let mut trigger = Trigger::default();
                    parse_trigger(&mut trigger, &value, localisations);
                    mission.trigger = Some(trigger);
                }
                "effect" => {
                    parse_effect(&mut mission, value);
                }
                "ai_weight" => {}
                _ => {
                    todo!()
                }
            }
        }
    }
    if is_a_mission {
        mission.id = key.to_string();
        if let Some(localisations) = localisations {
            mission.title = localisations.get(&(key.to_string() + "_title")).cloned();
            mission.desc = localisations.get(&(key.to_string() + "_desc")).cloned();
        }
        tree.missions.push(mission);
    }
}

fn parse_effect(mission: &mut Mission, value: ValueReader<Windows1252Encoding>) {
    if let Ok(value) = value.read_object() {
        let mut effect = Effect::default();
        for (key, _op, value) in value.fields() {
            let key = key.read_str();
            let key = key.as_ref();
            match key {
                "custom_tooltip" => {
                    effect.custom_tooltip = Some(value.read_string().unwrap());
                }
                "add_country_modifier" => {
                    if let Ok(value) = value.read_object() {
                        let mut country_modifier = CountryModifier::default();
                        for (key, _op, value) in value.fields() {
                            let key = key.read_str();
                            let key = key.as_ref();
                            match key {
                                "name" => {
                                    country_modifier.name = value.read_string().unwrap();
                                },
                                "duration" => {
                                    country_modifier.duration = Some(value.read_scalar().unwrap().to_i64().unwrap());
                                }
                                "desc" => {
                                    country_modifier.desc = Some(value.read_string().unwrap());
                                }
                                "hidden" => {
                                    country_modifier.hidden = true;
                                }
                                _ => {
                                    todo!()
                                }
                            }
                        }

                        if effect.country_modifiers.is_none() {
                            effect.country_modifiers = Some(BTreeMap::new());
                        }
                        effect.country_modifiers.as_mut().unwrap().insert(key.to_string(), country_modifier);
                    }
                }
                "remove_country_modifier" => {
                    if let Ok(value) = value.read_object() {
                        let mut remove_country_modifier = CountryModifier::default();
                        for (key, _op, value) in value.fields() {
                            let key = key.read_str();
                            let key = key.as_ref();
                            match key {
                                "name" => {
                                    remove_country_modifier.name = value.read_string().unwrap();
                                },
                                "duration" => {
                                    remove_country_modifier.duration = Some(value.read_scalar().unwrap().to_i64().unwrap());
                                }
                                "desc" => {
                                    remove_country_modifier.desc = Some(value.read_string().unwrap());
                                }
                                "hidden" => {
                                    remove_country_modifier.hidden = true;
                                }
                                _ => {
                                    todo!()
                                }
                            }
                        }

                        if effect.remove_country_modifiers.is_none() {
                            effect.remove_country_modifiers = Some(BTreeMap::new());
                        }
                        effect.remove_country_modifiers.as_mut().unwrap().insert(key.to_string(), remove_country_modifier);
                    }
                }
                "set_country_flag" => {
                    if (["@", ":"].contains(&key)) {
                        println!("Country flag {}", key);
                        todo!()
                    }
                    if effect.set_country_flag.is_none() {
                        effect.set_country_flag = Some(BTreeMap::new());
                    }
                    effect.set_country_flag.as_mut().unwrap().insert(key.to_string(), key.to_string());
                }
                "clr_country_flag" => {
                    if (["@", ":"].contains(&key)) {
                        println!("Country flag {}", key);
                        todo!()
                    }
                    if effect.clr_country_flag.is_none() {
                        effect.clr_country_flag = Some(BTreeMap::new());
                    }
                    effect.clr_country_flag.as_mut().unwrap().insert(key.to_string(), key.to_string());
                }
                "country_event" => {
                    if let Ok(value) = value.read_object() {
                        let mut country_event = CountryEvent::default();
                        for (key, _op, value) in value.fields() {
                            let key = key.read_str();
                            let key = key.as_ref();
                            match key {
                                "id" => {
                                    country_event.id = value.read_string().unwrap();
                                }
                                "days" => {
                                    country_event.days = Some(value.read_scalar().unwrap().to_u64().unwrap());
                                }
                                "random" => {
                                    country_event.random = Some(value.read_scalar().unwrap().to_u64().unwrap());
                                }
                                "tooltip" => {
                                    country_event.tooltip = Some(value.read_string().unwrap());
                                }
                                _ => {
                                    println!("Country event {}", key);
                                    todo!()
                                }
                            }
                        }

                        if effect.country_events.is_none() {
                            effect.country_events = Some(BTreeMap::new());
                        }
                        effect.country_events.as_mut().unwrap().insert(key.to_string(), country_event);
                    }
                }
                _ => {
                    let scope = value.read_object();
                    if scope.is_ok() { // Scope
                        println!("SCOPE {}", key);
                        // effect.misc = Some(value.unwrap().json().to_string());
                    } else { // Misc
                        println!("MISC {}", key);
                        if effect.misc.is_none() {
                            println!("VALUE {:?}", value.read_string());
                        } else {
                            // effect.misc = Some(effect.misc.unwrap() + value.unwrap().json().to_string().as_str());
                        }
                        // todo!()
                    }
                }
            }
        }

        mission.effect = Some(effect);
    }
}

fn parse_trigger(trigger: &mut Trigger, value: &ValueReader<Windows1252Encoding>, localisations: Option<&HashMap<String, String>>) {
    if let Ok(value) = value.read_object() {
        for (key, _op, value) in value.fields() {
            let key = key.read_str();
            let key = key.as_ref();
            match key {
                "OR" => {
                    let mut trigger_or = Trigger::default();
                    parse_trigger(&mut trigger_or, &value, localisations);
                    trigger.or = Some(trigger_or.into());
                }
                "AND" => {
                    let mut trigger_and = Trigger::default();
                    parse_trigger(&mut trigger_and, &value, localisations);
                    trigger.and = Some(trigger_and.into());
                }
                "NOT" => {
                    let mut trigger_not = Trigger::default();
                    parse_trigger(&mut trigger_not, &value, localisations);
                    trigger.not = Some(trigger_not.into());
                }
                "tooltip" => {
                    trigger.tooltip = Some(value.read_string().expect("Unable to parse mission icon as string"));
                    if let Some(localisations) = localisations {
                        trigger.tooltip_desc = Some(localisations.get(&(value.read_string().expect("Bob"))).cloned().unwrap())
                    }
                }
                "custom_trigger_tooltip" => {
                    let mut trigger_custom_tooltip = Trigger::default();
                    parse_trigger(&mut trigger_custom_tooltip, &value, localisations);
                    trigger.custom_tooltip = Some(trigger_custom_tooltip.into());
                }
                "tag" => {
                    trigger.tag = Some(value.read_string().expect("Unable to parse mission icon as string"));
                }
                "has_country_flag" => {
                    trigger.has_country_flag = Some(value.read_string().expect("Unable to parse mission icon as string"));
                }
                "religion" => {
                    trigger.religion = Some(value.read_string().expect("Unable to parse mission icon as string"));
                }
                "dynasty" => {
                    trigger.dynasty = Some(value.read_string().expect("Unable to parse mission icon as string"));
                }
                _ => {
                    trigger.misc = Some(value.json().to_string());
                }
            }
        }
    }
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

pub fn parse_missions(localisations: Option<&HashMap<String, String>>) -> Vec<MissionTree> {
    let mut mission_trees = vec![];
    let paths = fs::read_dir("./anbennar/missions").expect("Missing missions directory");
    for path in paths {
        match path {
            Ok(file) => {
                let data = fs::read(file.path()).expect("error reading file");
                let parsed = parse_mission_file(data.as_slice(), localisations);
                mission_trees.extend(parsed);
            }
            _ => {}
        }
    }

    mission_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_mission_parse() {
        let data = include_bytes!("../anbennar/missions/Adenica_Missions.txt");
        let actual = parse_mission_file(data, None);
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
                    let trees = parse_mission_file(data.as_slice(), None);
                    for tree in trees {
                        if let Some(potential) = &tree.potential {
                            if potential.tag.as_deref() == Some("H90") {
                                for mission in tree.missions {
                                    assert!(!mission.id.is_empty());
                                    println!("{:?}", mission);
                                    //io::stdout().flush().unwrap();
                                }
                            }
                        }
                        // break
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