use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;

use jomini::json::{DuplicateKeyMode, JsonOptions};
use jomini::text::ValueReader;
use jomini::Windows1252Encoding;
use serde_json::Value;

pub fn read_all_files_in_path(directory: String) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let paths = fs::read_dir(directory).expect("Missing directory");
    for path in paths {
        if let Ok(path) = path {
            let path = path.path();
            if path.is_dir() {
                results.extend(read_all_files_in_path(path.to_str().unwrap().to_string()));
            } else {
                results.push(path)
            }
        }
    }
    results
}

pub fn get_git_changed_files(directory: String, path_prefix: String, commit_range: String) -> BTreeSet<String> {
    let process = std::process::Command::new("git")
        .arg("log")
        .arg("--pretty=format:")
        .arg("--name-only")
        .arg(commit_range)
        .arg(path_prefix)
        .current_dir(directory)
        .output();
    if process.as_ref().is_ok_and(|p|p.status.success()) {
        let stdout = process.unwrap().stdout;
        let output = std::str::from_utf8(stdout.as_slice()).unwrap();
        let files: BTreeSet<&str> = output.split("\n").filter(|l| l.ne(&"")).collect();
        return files.iter().map(|l| l.to_string()).collect()
    }
    BTreeSet::new()
}

pub fn jsonify(reader: ValueReader<Windows1252Encoding>) -> String {
    return reader
        .json()
        .with_options(
            JsonOptions::new()
            .with_prettyprint(true)
            .with_duplicate_keys(DuplicateKeyMode::Group)
        )
        .to_string()
}

pub fn htmlify(value: &Value) -> String {
    match value {
        Value::Object(obj) => {
            let mut items = Vec::new();
            for (key, val) in obj.iter() {
                match val {
                    Value::Array(arr) => {
                        for item in arr {
                            items.push(format!("<ul><li>{}: {}</li></ul>", translate(key), htmlify(item)));
                        }
                    },
                    _ => {
                        items.push(format!("<ul><li>{}: {}</li></ul>", translate(key), htmlify(val)));
                    }
                }
            }
            // TODO: I think the actual fix is to not round-trip the JSON value
            if let Some(eidx) = items.iter().position(|s| s.starts_with("<li>else")) {
                let iidx = items.iter().position(|s| s.starts_with("<li>if:")).unwrap();
                if iidx > eidx {
                    items.swap(iidx, eidx);
                }
            }
            return items.join("");
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter()
                .map(
                    | val
                    | format!("<li>{}</li>", htmlify(val))
                ).collect();
            format!("<ul>{}</ul>", items.join(""))
        }
        Value::String(s) => {
            format!("{}", s)
        },
        x => {
            let val = x.to_string();
            match val.as_str() {
                "true" => format!("{}", val),
                "false" => format!("{}", val),
                _ => format!("<ul><li>{}</li></ul>", val)
            }

        },
    }
}

pub fn translate(key: &str) -> &str {
    match key {
        "OR" => "One of the following",
        "AND" => "All of the following",
        "NOT" => "None of the following",
        "has_dlc" => "Has DLC Enabled",
        "has_reform" => "Have reform",
        "culture_group" => "Culture Group",
        "primary_culture" => "Primary Culture",
        "technology_group" => "Technology Group",
        &_ => key
    }
}

pub fn gather(path: String, files: &mut Vec<PathBuf>) -> Vec<PathBuf> {
    let directory = fs::read_dir(path);
    if let Ok(directory) = directory {
        for entry in directory {
            match entry {
                Ok(entry) => {
                    if entry.path().is_dir() {
                        gather(entry.path().to_str().unwrap().to_string(), files);
                    } else {
                        files.push(entry.path())
                    }
                }
                Err(_) => {}
            }
        }
    }
    files.to_vec()
}

pub fn parse_all_icons() -> HashMap<String, PathBuf> {
    let mut icon_paths: HashMap<String, PathBuf> = HashMap::new();
    let mut files: Vec<PathBuf> = vec![];
    let paths = vec![
        "./anbennar/gfx",
        "./basegame/gfx"
    ];

    for path in paths {
        gather(path.to_string(), &mut files);
    }

    for file in files {
        if file.exists() {
            let id = file.as_path().file_stem().unwrap().to_str().unwrap().to_string();
            icon_paths.insert(id, file);
        }
    }

    icon_paths
}

#[cfg(test)]
mod tests {
    use crate::governments::parse_government_reforms;

    use super::*;

    #[test]
    pub fn test_htmlify() {
        let reforms = parse_government_reforms(None);
        for reform in reforms {
            if reform.potential.is_some() {
                let potential = reform.potential.unwrap();
                let json = serde_json::from_str::<Value>(potential.as_str()).unwrap();
                // println!("{}", reform.id);
                // println!("{}", htmlify(&json));
            }
        }
    }

    #[test]
    pub fn test_git_file_changes() {
        let files = get_git_changed_files(
            "./anbennar".to_string(),
            "gfx/".to_string(),
            "d51ea5f58634dc52d802fb99a1c11b9160273cf2..48d1f3be419db55d6132f5ced622096e48b36288".to_string()
        );
        assert_eq!(files.len(), 1);
        assert_eq!(files.first().unwrap(), &"gfx/interface/great_projects/great_project_teal_keep.dds".to_string())
    }

    #[test]
    pub fn test_all_icons() {
        let map = parse_all_icons();
        for e in map.keys() {
            println!("{} {:?}", e, map.get(e).unwrap().to_str())
        }
    }
}