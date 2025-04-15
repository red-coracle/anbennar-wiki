use std::collections::{BTreeMap, HashMap};
use std::fs;

use jomini::{TextTape, Windows1252Encoding};
use jomini::text::ValueReader;

use crate::localisation::parse_all_localisations;
use crate::modifiers::get_modifier;

#[derive(Debug, Default)]
pub struct ReligiousGroup {
    pub id: String,
    pub center_of_religion: u64,
    pub religions: BTreeMap<String, Religion>,
    pub harmonized_modifier: Option<String>,
    pub crusade_name: Option<String>,
    pub schools: Option<BTreeMap<String, Schools>>
}

#[derive(Debug, Default)]
pub struct Religion {
    pub id: String,
    pub color: Vec<u64>,
    pub icon: Option<u64>,
    pub country_modifiers: BTreeMap<String, Vec<u8>>, // lifetime problems on scalar
    pub country_as_secondary_modifiers: BTreeMap<String, Vec<u8>>, // lifetime problems on scalar
    pub province_modifiers: BTreeMap<String, Vec<u8>>, // lifetime problems on scalar
    pub aspects: Vec<String>, // Option<>
    pub holy_sites: Vec<u64>, // Option<>
    pub blessings: Vec<String>, // Option<>
    pub orthodox_icons: BTreeMap<String, OrthodoxIcons>, // Option<>
    pub papacy: Option<Papacy>
    // etc.
}

#[derive(Debug, Default)]
pub struct Schools {
    pub id: String,
    pub picture: Option<String>,
    pub country_modifiers: BTreeMap<String, Vec<u8>>
}

#[derive(Debug, Default)]
pub struct OrthodoxIcons {
    pub id: String,
    pub country_modifiers: BTreeMap<String, Vec<u8>>
}

#[derive(Debug, Default)]
pub struct Papacy {
    pub papal_tag: Option<String>,
    pub seat_of_papacy: Option<u64>,
    pub concessions: BTreeMap<String, Concession>,
    pub curia_interaction: BTreeMap<String, CuriaInteraction>
}

#[derive(Debug, Default)]
pub struct Concession {
    pub harsh: BTreeMap<String, Vec<u8>>,
    pub concilatory: BTreeMap<String, Vec<u8>>
}

#[derive(Debug, Default)]
pub struct CuriaInteraction {
    pub id: String,
    // ALLOW
    pub cost: Option<u64>,
    // EFFECT
}

pub fn parse_religious_groups(localisations: Option<&HashMap<String, String>>) -> Vec<ReligiousGroup> {
    let mut religious_groups = vec![];
    let paths = fs::read_dir("./anbennar/common/religions").expect("Missing religion reforms directory");
    for path in paths {
        if let Ok(file) = path {
            let data = fs::read(file.path()).expect("error reading file");
            let parsed = parse_religious_groups_file(data.as_slice(), localisations);
            religious_groups.extend(parsed);
        }
    }

    religious_groups
}

fn parse_religious_groups_file(data: &[u8], localisations: Option<&HashMap<String, String>>) -> Vec<ReligiousGroup> {
    let mut religious_groups = vec![];
    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();

    for (key, _op, value) in reader.fields() {
        let mut religious_group = ReligiousGroup::default();
        religious_group.id = key.read_str().to_string();
        if let Ok(value) = value.read_object() {
            for (key, _op, value) in value.fields() {
                let key = key.read_str();
                let key = key.as_ref();
                match key {
                    "can_form_personal_unions" => {},
                    "defender_of_faith" => {},
                    "flags_with_emblem_percentage" => {},
                    "flag_emblem_index_range" => {},
                    "ai_will_propagate_through_trade" => {},
                    "center_of_religion" => {},
                    "religious_schools" => {
                        parse_religious_schools(&mut religious_group, &value);
                    }
                    "crusade_name" => {
                        let value = value.read_string();
                        if value.is_ok() {
                            religious_group.crusade_name = Some(value.unwrap());
                        }
                    },
                    "harmonized_modifier" => {
                        let value = value.read_string();
                        if value.is_ok() {
                            religious_group.harmonized_modifier = Some(value.unwrap());
                        }
                    }
                    _ => {
                        let mut religion = Religion::default();
                        religion.id = key.to_string();
                        if let Ok(value) = value.read_object() {
                            for (key, _op, value) in value.fields() {
                                let key = key.read_str();
                                let key = key.as_ref();
                                match key {
                                    "icon" => {
                                        let value = value.read_scalar();
                                        if value.is_ok() {
                                            religion.icon = Some(value.unwrap().to_u64().unwrap());
                                        }
                                    },
                                    "color" => {
                                        if let Ok(v) = value.read_array() {
                                            for value in v.values() {
                                                religion.color.push(value.read_scalar().unwrap().to_u64().unwrap());
                                            }
                                        }
                                    },
                                    "allowed_conversion" => {},
                                    "country" => {
                                        let value = value.read_object();
                                        if value.is_ok() {
                                            for (key, _op, value) in value.unwrap().fields() {
                                                // debug_modifiers(&key, &value);
                                                let modifier = get_modifier(&key.read_string());
                                                if modifier.is_some() {
                                                    religion.country_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                                }
                                            }
                                        }
                                    },
                                    "country_as_secondary" => {
                                        let value = value.read_object();
                                        if value.is_ok() {
                                            for (key, _op, value) in value.unwrap().fields() {
                                                // debug_modifiers(&key, &value);
                                                let modifier = get_modifier(&key.read_string());
                                                if modifier.is_some() {
                                                    religion.country_as_secondary_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                                }
                                            }
                                        }
                                    },
                                    "province" => {
                                        let value = value.read_object();
                                        if value.is_ok() {
                                            for (key, _op, value) in value.unwrap().fields() {
                                                // debug_modifiers(&key, &value);
                                                let modifier = get_modifier(&key.read_string());
                                                if modifier.is_some() {
                                                    religion.province_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                                }
                                            }
                                        }
                                    },
                                    "allow_female_defenders_of_the_faith" => {},
                                    "uses_church_power" => {},
                                    "uses_anglican_power" => {},
                                    "religious_reforms" => {
                                        // TODO
                                    },
                                    "personal_deity" => {
                                        // TODO
                                    },
                                    "hre_religion" => {},
                                    "on_convert" => {},
                                    "allowed_center_conversion" => {},
                                    "will_get_center" => {},
                                    "date" => {},
                                    "fetishist_cult" => {
                                        // TODO
                                    },
                                    "ancestors" => {
                                        // basegame totemism
                                    },
                                    "heretic" => {},
                                    "uses_judaism_power" => {
                                        // TODO
                                    },
                                    "aspects_name" => {},
                                    "aspects" => {
                                        if let Ok(v) = value.read_array() {
                                            // religion.aspects = Some(vec![]);
                                            for value in v.values() {
                                                religion.aspects.push(value.read_string().unwrap());
                                            }
                                        }
                                    },
                                    "holy_sites" => {
                                        if let Ok(v) = value.read_array() {
                                            // religion.holy_sites = Some(vec![]);
                                            for value in v.values() {
                                                religion.holy_sites.push(value.read_scalar().unwrap().to_u64().unwrap());
                                            }
                                        }
                                    },
                                    "blessings" => {
                                        if let Ok(v) = value.read_array() {
                                            for value in v.values() {
                                                religion.blessings.push(value.read_string().unwrap());
                                            }
                                        }
                                    },
                                    "harmonized_modifier" => {},
                                    "crusade_name" => {},
                                    "uses_isolationism" => {
                                        // TODO
                                    },
                                    "reform_tooltip" => {},
                                    "celebrate" => {
                                        // TODO?
                                    },
                                    "declare_war_in_regency" => {},
                                    "doom" => {
                                        // TODO
                                    },
                                    "hre_heretic_religion" => {},
                                    "gurus" => {
                                        // TODO immediate
                                    },
                                    "papacy" => {
                                        if let Ok(value) = value.read_object() {
                                            for (key, _op, value) in value.fields() {
                                                let key = key.read_str();
                                                let key = key.as_ref();
                                                let mut papacy = Papacy::default();
                                                match key {
                                                    "papal_tag" => {}
                                                    "election_cost" => {}
                                                    "seat_of_papacy" => {}
                                                    "harsh" => {}
                                                    "neutral" => {}
                                                    "concilatory" => {}
                                                    "concessions" => {
                                                        /*if let Ok(value) = value.read_object() {
                                                            for (key, _op, value) in value.fields() {
                                                                //let key = key.read_str();
                                                                //let key = key.as_ref();
                                                                match key.read_string().as_ref() {
                                                                    "potential_invite_scholar" => {},
                                                                    "can_invite_scholar" => {},
                                                                    "on_invite_scholar" => {},
                                                                    "invite_scholar_modifier_display" => {},
                                                                    "picture" => {
                                                                        let value = value.read_string();
                                                                        if value.is_ok() {
                                                                            papacy.picture = Some(value.unwrap());
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        // debug_modifiers(&key, &value);
                                                                        let modifier = get_modifier(&key.read_string());
                                                                        if modifier.is_some() {
                                                                            papacy.country_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }*/
                                                    }
                                                    _ => {
                                                        if let Ok(value) = value.read_object() {
                                                            for (key, _op, value) in value.fields() {
                                                                let key = key.read_str();
                                                                let key = key.as_ref();
                                                                let mut curia_interaction = CuriaInteraction::default();
                                                                curia_interaction.id = key.to_string();
                                                                match key {
                                                                    "cost" => {},
                                                                    "potential" => {},
                                                                    "allow" => {},
                                                                    "effect" => {},
                                                                    "ai_will_do" => {}
                                                                    _ => {
                                                                        todo!()
                                                                    }
                                                                }

                                                                papacy.curia_interaction.insert(key.to_string(), curia_interaction);
                                                            }
                                                        }
                                                    }
                                                }

                                                religion.papacy = Some(papacy);
                                            }
                                        }
                                    },
                                    "uses_karma" => {
                                        // TODO in 00_static_modifiers
                                    },
                                    "uses_harmony" => {
                                        // TODO
                                    },
                                    "has_patriarchs" => {
                                        // TODO
                                    },
                                    "orthodox_icons" => {
                                        if let Ok(value) = value.read_object() {
                                            for (key, _op, value) in value.fields() {
                                                let key = key.read_str();
                                                let key = key.as_ref();
                                                let mut orthodox_icons = OrthodoxIcons::default();
                                                orthodox_icons.id = key.to_string();
                                                if let Ok(value) = value.read_object() {
                                                    for (key, _op, value) in value.fields() {
                                                        //let key = key.read_str();
                                                        //let key = key.as_ref();
                                                        match key.read_string().as_ref() {
                                                            "allow" => {},
                                                            "ai_will_do" => {},
                                                            _ => {
                                                                // debug_modifiers(&key, &value);
                                                                let modifier = get_modifier(&key.read_string());
                                                                if modifier.is_some() {
                                                                    orthodox_icons.country_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                                                                }
                                                            }
                                                        }
                                                    }
                                                }

                                                religion.orthodox_icons.insert(key.to_string(), orthodox_icons);
                                            }
                                        }
                                    },
                                    "fervor" => {
                                        // TODO
                                    },
                                    "uses_hussite_power" => {
                                        // TODO
                                    },
                                    "can_have_secondary_religion" => {},
                                    "authority" => {},
                                    "misguided_heretic" => {},
                                    _ => {
                                        todo!()
                                        //println!("UNASSIGNED {}", key)
                                    }
                                }
                            }
                        }

                        religious_group.religions.insert(key.to_string(), religion);
                    }
                }
            }
        }
        religious_groups.push(religious_group);
    }

    religious_groups
}

fn parse_religious_schools(religious_group: &mut ReligiousGroup, value: &ValueReader<Windows1252Encoding>) {
    let mut schools_map = BTreeMap::new();

    if let Ok(value) = value.read_object() {
        for (key, _op, value) in value.fields() {
            let mut schools = Schools::default();
            schools.id = key.read_str().to_string();
            if let Ok(value) = value.read_object() {
                for (key, _op, value) in value.fields() {
                    match key.read_string().as_ref() {
                        "potential_invite_scholar" => {},
                        "can_invite_scholar" => {},
                        "on_invite_scholar" => {},
                        "invite_scholar_modifier_display" => {},
                        "picture" => {
                            schools.picture = value.read_string().ok()
                        }
                        _ => {
                            // debug_modifiers(&key, &value);
                            if let Some(modifier) = get_modifier(&key.read_string()) {
                                schools.country_modifiers.insert(key.read_string(), value.read_scalar().unwrap().as_bytes().to_vec());
                            }
                        }
                    }
                }
            }

            schools_map.insert(key.read_str().to_string(), schools);
        }
    }

    religious_group.schools = Some(schools_map);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    pub fn test_religious_groups_parse() {
        let paths = fs::read_dir("./anbennar/common/religions").expect("Missing religion reforms directory");
        let localisations = parse_all_localisations();
        for path in paths {
            if let Ok(file) = path {
                let data = fs::read(file.path()).expect("error reading file");
                let religious_groups = parse_religious_groups_file(data.as_slice(), Some(&localisations));
                for religious_group in religious_groups {
                    assert!(!religious_group.id.is_empty());
                    println!("RELIGIOUS GROUP {} {:?} {:?} {:?}", religious_group.id, religious_group.crusade_name, religious_group.harmonized_modifier, religious_group.schools);
                    for (_, religion) in religious_group.religions {
                        assert!(!religion.id.is_empty());
                        println!("RELIGION {} {:?} {:?}", religion.id, religion.country_modifiers, religion.orthodox_icons)
                    }
                }
            }
        }
    }
}