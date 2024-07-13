use std::fs;

use jomini::JominiDeserialize;

#[derive(Clone, Debug, JominiDeserialize)]
pub struct EventSet {
    //namespace: Option<String>, // there are files with multiple namespaces
    #[jomini(alias = "country_event", duplicated)]
    events: Vec<Event>,
}

#[derive(Clone, Debug, JominiDeserialize)]
pub struct Event {
    id: String,
    #[jomini(take_last)]
    title: String,
    // picture: String, // can be an object
    // desc: String, // can be an object
}

pub fn parse_events() -> Vec<EventSet> {
    let mut results = Vec::with_capacity(100);
    let paths = fs::read_dir("./anbennar/events").expect("Missing events directory");
    for path in paths {
        match path {
            Ok(file) => {
                let data = fs::read(file.path()).expect("error reading file");
                let actual: EventSet = jomini::text::de::from_windows1252_slice(data.as_slice()).unwrap();
                if actual.events.len() < 1 {
                    continue;
                }
                results.push(actual);
            }
            _ => {}
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_events_parse() {
        let event_sets = parse_events();
        assert!(event_sets.len() > 0);
    }
}