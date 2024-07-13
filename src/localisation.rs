use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;

use regex::Regex;

use crate::countries::Country;

#[derive(Default)]
pub struct Localisations {
    pub countries: HashMap<String, String>,
    pub ideas: HashMap<String, String>,
    pub cultures: HashMap<String, String>,
}

fn parse_localisation_file(data: &str) -> HashMap<String, String> {
    let mut localisations = HashMap::new();
    for mut line in data.lines().skip(1) {
        line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let rows = line.split_once(':');
        for pieces in rows.iter() {
            match pieces {
                (x, y) if x != &"" => {
                    if y.len() >= 4 {
                        // Valid entries should have number, space, double quote, (some string), double quote
                        // and can end with an inline comment
                        // TODO: not sure how inline comments containing double-quotes are handled

                        let mut val = y.trim_start_matches(char::is_numeric).trim_start();

                        if !val.ends_with('"') {
                            let last_quote = val.rfind('"');
                            if let Some(index) = last_quote {
                                val = &val[0..index]
                            }
                        }

                        val = val.strip_prefix('"').unwrap_or_else(|| { val });
                        val = val.strip_suffix('"').unwrap_or_else(|| { val });

                        // let quote_count: Vec<_> = val.match_indices("\"").collect();
                        // if quote_count.len() % 2 != 0 {
                            // eprintln!("mismatched quote count in {:?}", x)
                        // }

                        localisations.insert(x.to_string(), val.to_string());
                    } else {
                        localisations.insert(x.to_string(), "".to_string());
                    }
                },
                _ => {}
            }
        }
    }

    localisations
}

pub fn parse_country_localisations() -> Vec<Country> {
    let mut tag_map: HashMap<String, Country> = HashMap::new();
    let file = fs::read("./anbennar/localisation/anb_countries_l_english.yml")
        .expect("missing country localisation file");
    let parsed = parse_localisation_file(std::str::from_utf8(file.as_slice()).unwrap());
    for localisation in parsed {
        match localisation {
            (k, v) if k.ends_with("_ADJ") => {
                let tag = k.strip_suffix("_ADJ").unwrap_or_else(||{k.as_str()});
                match tag_map.get_mut(tag) {
                    Some(_) => {
                        tag_map.get_mut(tag).unwrap().adjective = v;
                    },
                    None => {
                        let mut country = Country::default();
                        country.tag = String::from(tag);
                        country.adjective = v.to_string();
                        tag_map.insert(country.tag.clone(), country);
                    }
                }
            },
            (k, v) if k.len() == 3 => {
                match tag_map.get_mut(&k) {
                    Some(_) => {
                        tag_map.get_mut(&k).unwrap().name = v.to_string();
                    },
                    None => {
                        let mut country = Country::default();
                        country.tag = String::from(&k);
                        country.name = v.to_string();
                        if country.name != "" {
                            tag_map.insert(country.tag.clone(), country);
                        }
                    }
                }
            },
            (_, _) => {}
        }
    }
    tag_map.into_values().collect()
}

pub fn parse_idea_localisations() -> HashMap<String, String> {
    let file = fs::read("./anbennar/localisation/anb_powers_and_ideas_l_english.yml")
        .expect("missing powers & ideas localisation file");
    parse_localisation_file(std::str::from_utf8(file.as_slice()).unwrap())
}

pub fn parse_culture_localisations() -> HashMap<String, String> {
    let file = fs::read("./anbennar/localisation/anb_cultures_l_english.yml")
        .expect("missing cultures localisation file");
    parse_localisation_file(std::str::from_utf8(file.as_slice()).unwrap())
}

pub fn parse_religion_localisations() -> HashMap<String, String> {
    // This file doesn't include all religions
    let file = fs::read("./anbennar/localisation/anb_religions_l_english.yml")
        .expect("missing religions localisation file");
    parse_localisation_file(std::str::from_utf8(file.as_slice()).unwrap())
}

pub fn parse_all_localisations() -> HashMap<String, String> {
    let mut localisations: HashMap<String, String> = HashMap::new();
    let paths = fs::read_dir("./anbennar/localisation").expect("Missing localisation directory");
    for path in paths {
        match path {
            Ok(file) => {
                let file = fs::read(file.path()).expect("error reading file");
                let parsed = parse_localisation_file(std::str::from_utf8(file.as_slice()).unwrap());
                localisations.extend(parsed);
            }
            _ => {}
        }
    }
    localisations
}

pub fn colourise(input: &String) -> String {
    // TODO: there's probably a way to do this all in one?
    let white = Regex::new(r"§W(.+?)§!").unwrap();
    let input = white.replace_all(input, "<span class=\"white\">$1</span>");

    let blue = Regex::new(r"§B(.+?)§!").unwrap();
    let input = blue.replace_all(input.borrow(), "<span class=\"blue\">$1</span>");

    let green = Regex::new(r"§G(.+?)§!").unwrap();
    let input = green.replace_all(input.borrow(), "<span class=\"green\">$1</span>");

    let red = Regex::new(r"§R(.+?)§!").unwrap();
    let input = red.replace_all(input.borrow(), "<span class=\"red\">$1</span>");

    let black = Regex::new(r"§b(.+?)§!").unwrap();
    let input = black.replace_all(input.borrow(), "<span class=\"black\">$1</span>");

    let grey = Regex::new(r"§g(.+?)§!").unwrap();
    let input = grey.replace_all(input.borrow(), "<span class=\"grey\">$1</span>");

    let yellow = Regex::new(r"§Y(.+?)§!").unwrap();
    let input = yellow.replace_all(input.borrow(), "<span class=\"yellow\">$1</span>");

    let marine = Regex::new(r"§M(.+?)§!").unwrap();
    let input = marine.replace_all(input.borrow(), "<span class=\"marine\">$1</span>");

    let teal = Regex::new(r"§T(.+?)§!").unwrap();
    let input = teal.replace_all(input.borrow(), "<span class=\"teal\">$1</span>");

    let orange = Regex::new(r"§O(.+?)§!").unwrap();
    let input = orange.replace_all(input.borrow(), "<span class=\"orange\">$1</span>");

    let lime = Regex::new(r"§l(.+?)§!").unwrap();
    let input = lime.replace_all(input.borrow(), "<span class=\"lime\">$1</span>");

    let jade = Regex::new(r"§J(.+?)§!").unwrap();
    let input = jade.replace_all(input.borrow(), "<span class=\"jade\">$1</span>");

    let purple = Regex::new(r"§P(.+?)§!").unwrap();
    let input = purple.replace_all(input.borrow(), "<span class=\"purple\">$1</span>");

    let violet = Regex::new(r"§V(.+?)§!").unwrap();
    let input = violet.replace_all(input.borrow(), "<span class=\"violet\">$1</span>");

    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_localisation_file() {
        let data = include_str!("../anbennar/localisation/anb_countries_l_english.yml");
        let parsed = parse_localisation_file(data);
        assert_eq!(parsed.get("Z35").unwrap_or(&"".to_string()), &"Rósande".to_string());
        assert_eq!(parsed.get("Z35_ADJ").unwrap_or(&"".to_string()), &"Rósanda".to_string());
        let data = include_str!("../anbennar/localisation/dwarven_pantheon_l_english.yml");
        let parsed = parse_localisation_file(data);
        assert_eq!(parsed.get("dwarven_pantheon.4.d").unwrap_or(&"".to_string()), include_str!("../tests/fixtures/dwarven_pantheon.4.d.txt"));
    }

    #[test]
    pub fn test_parse_single() {
        // test an entry with unusual properties
        let data = "l_english:\n   ABC:00001     \t\"HEL\"LO\\n\" \t  # this is a comment";
        let parsed = parse_localisation_file(data);
        assert_eq!(parsed.get("ABC").unwrap(), "HEL\"LO\\n");
    }

    #[test]
    pub fn test_parse_country_localisations() {
        let parsed = parse_country_localisations();
        for country in parsed {
            assert_ne!(country.tag, "");
            assert_ne!(country.name, "");
            assert_ne!(country.adjective, "");
        }
    }

    #[test]
    pub fn test_parse_idea_localisations() {
        let parsed = parse_idea_localisations();
        assert_eq!(parsed.get("A01_romance_and_chivalry").unwrap(), "Romance & Chivalry");
    }

    #[test]
    pub fn test_parse_culture_localisations() {
        let parsed = parse_culture_localisations();
        assert_eq!(parsed.get("moon_elf").unwrap(), "Moon Elf");
        assert_eq!(parsed.get("stalboric").unwrap(), "Stalbóric");
    }

    #[test]
    pub fn test_parse_religion_localisations() {
        let parsed = parse_religion_localisations();
        assert_eq!(parsed.get("regent_court").unwrap(), "Regent Court");
        assert_eq!(parsed.get("suhans_praxis").unwrap(), "Suhan's Praxis");
    }

    #[test]
    pub fn test_parse_all_localisations() {
        let parsed = parse_all_localisations();
        assert_eq!(parsed.get("regent_court").unwrap(), "Regent Court");
        assert_eq!(parsed.get("dwarven_pantheon").unwrap(), "Dwarven Pantheon");
    }

    #[test]
    pub fn test_colourise() {
        let data = include_str!("../anbennar/localisation/anb_startup_screen_l_english.yml");
        let parsed = parse_localisation_file(data);
        let start = parsed.get("string_start_lorent").unwrap();
        assert_eq!(colourise(start), include_str!("../tests/fixtures/string_start_lorent.txt"));
    }

    #[test]
    pub fn test_inline_comments() {
        let data = include_str!("../anbennar/localisation/anb_adventurers_wanted_l_english.yml");
        let parsed = parse_localisation_file(data);
        assert_eq!(parsed.get("aw_haunted_house.120.t").unwrap(), "The Starless Night");
    }
}
