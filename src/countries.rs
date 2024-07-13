use std::collections::{HashMap, HashSet};
use std::fs;

use jomini::{JominiDeserialize, TextTape, Windows1252Encoding};
use jomini::text::ValueReader;
use serde::Serialize;

use crate::ideas::IdeaSet;
use crate::localisation::parse_all_localisations;

#[derive(Debug, Serialize, Default)]
pub struct Country {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub adjective: String,
    pub ideas: IdeaSet,
    pub history: CountryHistory,
    pub end_game_tag: bool,
}

#[derive(Debug, Serialize, JominiDeserialize, Default)]
pub struct CountryHistory {
    // pub tag: String,
    #[jomini(default)]
    pub setup_vision: bool,
    #[jomini(default)]
    pub government: String,
    #[jomini(alias = "add_government_reform", duplicated)]
    pub government_reforms: Vec<String>,
    #[jomini(default)]
    pub government_rank: usize,
    #[jomini(default)]
    pub primary_culture: String,
    #[jomini(alias = "add_accepted_culture", duplicated)]
    pub accepted_cultures: Vec<String>,
    #[jomini(default)]
    pub religion: String,
    #[jomini(default)]
    pub technology_group: String,
    #[jomini(default)]
    pub capital: usize,
    #[jomini(default)]
    pub fixed_capital: usize,
    #[jomini(alias = "historical_rival", duplicated)]
    pub historical_rivals: Vec<String>,
    #[jomini(alias = "historical_friend", duplicated)]
    pub historical_friends: Vec<String>,
}

// Returns (TAG, path)
pub fn parse_country_tags() -> Vec<(String, String)> {
    let mut tags = Vec::new();
    let file = fs::read("./anbennar/common/country_tags/anb_countries.txt")
        .expect("missing country tags file");
    let file = file.as_slice();
    let data = std::str::from_utf8(file).unwrap();

    for mut line in data.lines() {
       line = line.trim();
        if !line.is_empty() && !line.starts_with('#') {
            // TODO: a bit messy
            let tag = &line[..=2];
            let start = line.find('"');
            let end = line.rfind('"');
            match (start, end) {
                (Some(x), Some(y)) => {
                    if tag.to_string() == "NPC" {
                        continue
                    }
                    tags.push((tag.to_string(), line[x+1..y].to_string()));
                },
                _ => {}
            }
        }
    }

    tags
}

pub fn parse_history_for_tag(tag: String) -> Option<CountryHistory> {
    let paths = fs::read_dir("./anbennar/history/countries").expect("Missing country history directory");
    for path in paths {
        match path {
            Ok(file) => {
                let filename = file.file_name();
                let filename = String::from_utf8_lossy(filename.as_encoded_bytes());
                if filename.starts_with(&tag) {
                    let data = fs::read(file.path()).expect("error reading file");
                    let actual: CountryHistory = jomini::text::de::from_windows1252_slice(data.as_slice()).unwrap();
                    return Option::from(actual);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn parse_country_histories() -> HashMap<String, CountryHistory> {
    let mut histories: HashMap<String, CountryHistory> = HashMap::new();
    let paths = fs::read_dir("./anbennar/history/countries").expect("Missing country history directory");
    for path in paths {
        match path {
            Ok(file) => {
                let tag = String::from(file.file_name().to_str().unwrap().split('-').collect::<Vec<&str>>()[0].trim());
                let data = fs::read(file.path()).expect("error reading file");
                let history: CountryHistory = jomini::text::de::from_windows1252_slice(data.as_slice()).unwrap();
                histories.insert(tag, history);
            }
            _ => {}
        }
    }

    histories
}

pub fn parse_countries() -> Vec<Country>{
    let mut country_map: HashMap<String, Country> = HashMap::new();
    let mut histories = parse_country_histories();
    let localisations = parse_all_localisations();
    let end_game_tags = end_game_tags();

    for (tag, _path) in parse_country_tags() {
        let mut country = Country::default();
        country.tag = tag.clone();
        country.history = histories.remove(&tag).unwrap_or_else(|| CountryHistory::default());

        if let Some(localisation) = localisations.get(&tag) {
            country.name = localisation.to_string();
            if country.name == "" {
                // there are "dummy" tags?
                continue
            }
        } else {
            // skip missing
            continue;
        }
        if let Some(localisation) = localisations.get(&format!("{tag}_ADJ")) {
            country.adjective = localisation.to_string();
        }
        if let Some(culture) = localisations.get(&country.history.primary_culture) {
            country.history.primary_culture = culture.to_string();
        }
        if let Some(religion) = localisations.get(&country.history.religion) {
            country.history.religion = religion.to_string();
        }
        if end_game_tags.contains(&country.tag) {
            country.end_game_tag = true;
        }
        country_map.insert(tag, country);
    }

    country_map.into_values().collect()
}

pub fn end_game_tags() -> HashSet<String> {
    let mut results = HashSet::new();
    let file = fs::read("./anbennar/common/scripted_triggers/00_scripted_triggers.txt");
    let data = file.expect("Missing 00_scripted_triggers.txt");
    let tape = TextTape::from_slice(&*data).unwrap();
    let reader = tape.windows1252_reader();

    fn sift(mut tags: HashSet<String>, obj: ValueReader<Windows1252Encoding>) -> HashSet<String> {
        if let Ok(inner) = obj.read_object() {
            for (key, _op, value) in inner.fields() {
                if key.read_str() == "tag" || key.read_str() == "was_tag" {
                    tags.insert(value.read_string().expect("could not parse as string"));
                } else {
                    tags = sift(tags.clone(), value);
                }
            }
        }
        tags
    }

    for (key, _op, value) in reader.fields() {
        if key.read_str() == "was_never_end_game_tag_trigger" {
            results = sift(results.clone(), value);
            break;
        }
    }

    results
}

pub fn formable_tags() -> HashSet<String> {
    let mut tags = HashSet::new();
    let search_paths = vec!["./anbennar/decisions", "./anbennar/events"];

    fn sift(mut tags: HashSet<String>, obj: ValueReader<Windows1252Encoding>) -> HashSet<String> {
        if let Ok(inner) = obj.read_object() {
            for (key, _op, value) in inner.fields() {
                let key = key.read_str();

                if key == "potential" || key == "provinces_to_highlight" || key == "allow" {
                    continue;
                }
                if key == "change_tag" {
                    tags.insert(value.read_string().expect("could not parse as string"));
                    return tags
                } else {
                    tags = sift(tags.clone(), value);
                }
            }
        }
        tags
    }

    for path in search_paths {
        let paths = fs::read_dir(path).expect(format!("Missing {path} directory").as_str());
        for path in paths {
            match path {
                Ok(file) => {
                    let file = fs::read(file.path()).expect("error reading file");
                    let tape = TextTape::from_slice(file.as_slice()).unwrap();
                    let reader = tape.windows1252_reader();
                    for (_key, _op, value) in reader.fields() {
                        if let Ok(country_decisions) = value.read_object() {
                            for (_key, _op, value) in country_decisions.fields() {
                                let parsed = sift(HashSet::new(), value);
                                tags.extend(parsed);
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    tags
}


#[cfg(test)]
mod tests {
    use crate::countries::*;

    #[test]
    pub fn test_parse_country_tags() {
        parse_country_tags();
    }

    #[test]
    pub fn test_parse_country_histories() {
        parse_country_histories();
    }

    #[test]
    pub fn test_parse_end_game_tags() {
        let tags = end_game_tags();
        assert!(tags.len() > 0);
        assert!(tags.contains("Z01"));
    }

    #[test]
    pub fn test_parse_formable_tags() {
        let tags = formable_tags();
        assert!(tags.len() > 0);
        assert!(tags.contains("Z35")); // from decisions
        assert!(tags.contains("Z01")); // from events
    }

    #[test]
    pub fn test_parse_history_for_tag() {
        let lorent = parse_history_for_tag(String::from("A01")).unwrap();
        assert_eq!(lorent.primary_culture, "high_lorentish");
        let birzartanses = parse_history_for_tag(String::from("F21")).unwrap();
        assert_eq!(birzartanses.accepted_cultures, vec!["bahari", "kuzarami"]);
    }

    #[test]
    pub fn test_parse_countries() {
        let countries = parse_countries();
        for country in countries {
            assert_ne!(country.tag, "");
            assert_ne!(country.name, "");
        }
    }
}
