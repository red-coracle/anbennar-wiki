use std::collections::BTreeMap;
use std::fs;

use jomini::{JominiDeserialize, TextTape};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct CountryIdeaSets {
    pub idea_sets: BTreeMap<String, IdeaSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct IdeaSet {
    pub tags: Vec<String>,
    pub name: String,
    pub start: BTreeMap<String, String>,
    pub bonus: BTreeMap<String, String>,
    pub ideas: Vec<Idea>,
}

#[derive(Clone, JominiDeserialize, PartialEq, Debug, Default, Serialize)]
pub struct Idea {
    pub name: String,
    #[jomini(default)]
    pub description: String,
    pub effects: BTreeMap<String, String>,
}

pub fn parse_ideas() -> CountryIdeaSets {
    let data = fs::read("./anbennar/common/ideas/anb_country_ideas.txt")
        .expect("Ideas file not found");
    let mut idea_sets = CountryIdeaSets{
        idea_sets: Default::default()
    };

    let tape = TextTape::from_slice(data.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    for (key, _op, value) in reader.fields() {
        let mut set: IdeaSet = IdeaSet{
            tags: vec![],
            name: key.read_string(),
            start: Default::default(),
            bonus: Default::default(),
            ideas: vec![],
        };
        if let Ok(idea_group) = value.read_object() {
            for (key, _op, value) in idea_group.fields() {
                let key = key.read_str();
                if key == "start" {
                    if let Ok(modifiers) =  value.read_object() {
                        for (key, _op, value) in modifiers.fields() {
                            set.start.insert(key.read_string(), value.read_string().unwrap());
                        }
                    }
                } else if key == "bonus" {
                    if let Ok(modifiers) = value.read_object() {
                        for (key, _op, value) in modifiers.fields() {
                            set.bonus.insert(key.read_string(), value.read_string().unwrap());
                        }
                    }
                } else if key == "free" {
                    // pass
                } else if key == "trigger" {
                    if let Ok(modifiers) = value.read_object() {
                        for (key, _, value) in modifiers.fields() {
                            let key = key.read_str();
                            if key == "OR" {
                                if let Ok(tags) = value.read_object() {
                                    for (key, _op, value) in tags.fields() {
                                        if key.read_string().to_lowercase() == "tag" {
                                            set.tags.push(value.read_string().unwrap());
                                        }
                                    }
                                }
                            } else if key == "tag" {
                                set.tags.push(value.read_string().unwrap());
                            }
                        }
                    }
                } else {
                    if let Ok(modifiers) = value.read_object() {
                        let mut idea = Idea{
                            name: key.to_string(),
                            description: "".to_string(),
                            effects: Default::default(),
                        };
                        for (key, _op, value) in modifiers.fields() {
                            // there can be effects in ideas - skipping those for now
                            if let Ok(value) = value.read_string() {
                                idea.effects.insert(key.read_string(), value);
                            }
                        }
                        set.ideas.push(idea);
                    }
                }
            }
        }

        for tag in &set.tags {
            idea_sets.idea_sets.insert(tag.to_string(), set.clone());
        }
    }

    idea_sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_idea_parse() {
        let actual = parse_ideas();
        for (_, set) in actual.idea_sets.iter() {
            assert_ne!(set.name, "");
            assert_ne!(set.start.len(), 0);
            assert_ne!(set.tags.len(), 0);
            assert_eq!(set.ideas.len(), 7);
        }
    }
}
