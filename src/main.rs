use std::{env, fs};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::Instant;

use deunicode::deunicode;
use jomini::{Scalar, TextTape};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::countries::{Country, formable_tags};
use crate::governments::{parse_government_reforms, parse_governments};
use crate::ideas::parse_ideas;
use crate::imagemagick::ImageMagick;
use crate::localisation::{parse_all_localisations, parse_idea_localisations};
use crate::map::{parse_continents, parse_map};
use crate::missions::tags_with_missions;
use crate::modifiers::get_modifier;
use crate::modifiers::ModifierNormal::{Negative, Positive};
use crate::utils::htmlify;

mod localisation;
mod ideas;
mod countries;
mod modifiers;
mod missions;
mod events;
mod imagemagick;
mod governments;
mod utils;
mod map;
mod greatprojects;
mod graphics;


fn main() {
    let args: Vec<String> = env::args().collect();
    let api_url = env::var("API_URL").unwrap();
    let bot_name = env::var("BOTNAME").unwrap();
    let bot_pass = env::var("BOTPASS").unwrap();

    let mut mwclient = MediaWikiClient::new(api_url, bot_name, bot_pass);
    mwclient.login();

    if args.contains(&String::from("--ideas")) {
        idea_pages(&mut mwclient);
    }
    if args.contains(&String::from("--countries")) {
        country_list_and_details(&mut mwclient);
    }
    if args.contains(&String::from("--flags")) {
        upload_flags(&mut mwclient);
    }
    if args.contains(&String::from("--racial-modifiers")) {
        racial_modifiers(&mut mwclient);
    }
    if args.contains(&String::from("--governments")) {
        run_governments(&mut mwclient);
    }
    if args.contains(&String::from("--gov-reform-icons")) {
        run_government_icons(&mut mwclient)
    }
    if args.contains(&String::from("--map")) {
        run_map(&mut mwclient)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaWikiResponse {
    batchcomplete: String,
    query: MediaWikiQuery
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaWikiQuery {
    tokens: HashMap<String, String>
}

struct MediaWikiClient {
    url: String,
    botname: String,
    botpass: String,
    csrf_token: Option<String>,
    csrf_time: Instant,
    csrf_counter: u8,
    httpclient: Client,
}

impl MediaWikiClient {
    pub fn new(url: String, botname: String, botpass: String) -> MediaWikiClient {
        MediaWikiClient{
            url,
            botname,
            botpass,
            csrf_token: None,
            csrf_time: Instant::now(),
            csrf_counter: 0,
            httpclient: Client::builder().cookie_store(true).build().unwrap(),
        }
    }

    pub fn login(&self) {
        let params = [("action", "query"), ("meta", "tokens"), ("type", "login"), ("format", "json")];
        let response = self.httpclient.get(self.url.as_str()).query(&params).send().unwrap().json::<MediaWikiResponse>().unwrap();
        let logintoken = response.query.tokens.get("logintoken").unwrap().clone();
        let params = [("action", "login"), ("lgname", self.botname.as_str()), ("lgpassword", self.botpass.as_str()), ("format", "json"), ("lgtoken", &*logintoken)];
        self.httpclient.post(self.url.as_str()).form(&params).send().unwrap().text().unwrap();
    }

    pub fn csrf(&mut self) -> String {
        // It's not clear how long the tokens last
        if self.csrf_token.is_some() && (self.csrf_time.elapsed().as_secs() < 120 && self.csrf_counter < 128) {
            return self.csrf_token.as_ref().unwrap().to_string();
        }

        if self.csrf_counter >= 128 {
            self.login();
            self.csrf_counter = 0;
        }

        let params = [("action", "query"), ("meta", "tokens"), ("format", "json")];
        let response = self.httpclient.get(self.url.as_str()).query(&params).send().unwrap().json::<MediaWikiResponse>().unwrap();
        let token = response.query.tokens.get("csrftoken").unwrap().to_string();
        self.csrf_token = Some(token.clone());
        self.csrf_time = Instant::now();
        token
    }

    pub fn upload(&mut self, filename: String, path: &PathBuf) {
        let form = reqwest::blocking::multipart::Form::new()
            .text("action", "upload")
            .text("filename", filename)
            .text("format", "json")
            .text("token", self.csrf())
            .text("ignorewarnings", "1")
            .file("file", path).unwrap();
        let x = self.httpclient.post(self.url.as_str()).multipart(form).send().unwrap();
        self.csrf_counter += 1;
        println!("{:?}", x.text())
    }

    pub fn add_edit_page(&mut self, title: &String, text: String) {
        if title.trim() == "" {
            panic!("Not editing page without title")
        }
        println!("Updating {title}");
        let csrf: String = self.csrf();
        let title = title.clone();
        let text = text.clone();
        let summary = format!("Add/edit {}", title);
        let form = reqwest::blocking::multipart::Form::new()
            .text("bot", "1")
            .text("action", "edit")
            .text("format", "json")
            .text("title", title)
            .text("text", text)
            .text("summary", summary)
            .text("token", csrf);
        let x = self.httpclient.post(self.url.as_str()).multipart(form).send().unwrap();
        self.csrf_counter += 1;
        println!("{:?}", x.text())
    }

    pub fn redirect(&mut self, source_title: &String, target_title: &String) {
        let text = format!("#REDIRECT [[{}]]", target_title);
        self.add_edit_page(source_title, text);
    }
}

fn title_case(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new()
    }
}

fn idea_pages(client: &mut MediaWikiClient) {
    let mut country_idea_sets = parse_ideas();
    let idea_localisations = parse_idea_localisations();

    for (_tag, set) in country_idea_sets.idea_sets.iter_mut() {
        let set_name = idea_localisations.get(&set.name);
        if let Some(set_name) = set_name {
            if set_name == "" {
                // there was a problem with the localisation
                continue;
            }
            let mut page_body = String::from("{{National Ideas\n");
            page_body += format!("<!-- {set_name} -->\n").as_str();
            let mut counter = 1;
            for idea in set.start.iter() {
                let (desc, value) = modifiers::localise_strings(idea.0, idea.1);
                page_body += format!("|tradition{}name={}\n", counter, desc).as_str();
                page_body += format!("|tradition{}effect={}\n", counter, value).as_str();
                counter += 1;
            }

            counter = 1;
            for idea in set.ideas.iter_mut() {
                let name = idea_localisations.get(&idea.name);
                let desc = idea_localisations.get((&format!("{}_desc", &idea.name)).into());
                if let Some(name) = name {
                    idea.name = name.to_string();
                }
                if let Some(desc) = desc {
                    idea.description = desc.to_string();
                }
                // TODO: Modifier localisations are scattered among many files

                page_body += format!("|idea{counter}name={name}\n", counter=counter, name=idea.name).as_str();
                page_body += format!("|idea{counter}desc={desc}\n", counter=counter, desc=idea.description).as_str();
                page_body += format!("|idea{counter}effect=").as_str();
                let mut counter2 = 0;
                for effect in idea.effects.iter_mut() {
                    let (desc, value) = modifiers::localise_strings(effect.0, effect.1);
                    if counter2 > 0 {
                        page_body += &*"<br />".to_string();
                    }
                    page_body += format!("{{{{Modifier|type=bonus|value={value}|description={desc} }}}}", value=value, desc=desc).as_str();
                    counter2 += 1;
                }
                page_body += &*"\n".to_string();
                counter += 1;
            }

            counter = 1;
            page_body += &*"|ambitioneffect=".to_string();
            for effect in set.bonus.iter() {
                let (desc, value) = modifiers::localise_strings(effect.0, effect.1);
                page_body += format!("{{{{Modifier|type=bonus|value={value}|description={desc} }}}}", value=value, desc=desc).as_str();
                counter += 1;
            }
            page_body += "}}\n\n<noinclude>[[Category:Country Ideas]]</noinclude>\n";
            let normal_set_name = deunicode(set_name);
            if String::ne(set_name, &normal_set_name) {
                client.redirect(set_name, &normal_set_name);
            }
            client.add_edit_page(&normal_set_name, page_body);
        }
    }
}

fn upload_flags(client: &mut MediaWikiClient) {
    // TODO: only upload changed flags
    let mut countries = countries::parse_countries();
    countries.sort_by_key(|k| k.tag.clone());
    for country in countries {
        let normal_name = deunicode(country.name.as_str());
        if normal_name == "" {
            continue;
        }
        let filename = format!("./anbennar/gfx/flags/{tag}.tga", tag=country.tag);
        let flag_file = Path::new(&filename);
        if flag_file.exists() {
            if let Some(converted) = ImageMagick::default().convert_to_png(flag_file) {
                client.upload(format!("{}_Flag.png", country.tag), &converted);
                let _ = fs::remove_file(converted);
            }
        }
    }
}

fn country_list_and_details(client: &mut MediaWikiClient) {
    let mut countries = countries::parse_countries();
    let mission_tags = tags_with_missions();
    let formable_tags = formable_tags();

    countries.sort_by(|a, b| a.tag.cmp(&b.tag));

    let mut page_str = String::new();
    page_str += "{| class=\"wikitable sortable\" style=\"text-align: center;\"\n";
    page_str += "|-\n";
    page_str += "! Flag !! Tag !! Name !! Culture !! Religion !! Continent !! Missions !! Formable !! End-game Tag\n";
    for country in countries {
        let normal_name = deunicode(country.name.as_str());
        if normal_name == "" {
            continue;
        }

        page_str += "|-\n";
        // TODO: add a template to display a placeholder flag when needed
        page_str += format!(
            "| [[File:{tag} Flag.png|link={normal_name}|center|64x64px]] || {tag} || [[{normal_name}]] || [[{primary_culture}]] || [[{religion}]] || || {missions} || {formable} || {egt}\n",
            tag=country.tag,
            normal_name=normal_name,
            primary_culture=deunicode(country.history.primary_culture.as_str()),
            religion=deunicode(country.history.religion.as_str()),
            missions=(||{if mission_tags.contains(&country.tag){"✅"} else {"❌"}})(),
            formable=(||{if formable_tags.contains(&country.tag){"✅"} else {"❌"}})(),
            egt=(||{if country.end_game_tag{"✅"} else {"❌"}})()
        ).as_str();
        country_detail_page(client, country, &mission_tags);
    }
    page_str += "|}\n";
    client.add_edit_page(&"Countries".to_string(), page_str);
}

fn country_detail_page(client: &mut MediaWikiClient, country: Country, mission_tags: &HashSet<String>) {
    let ideas = parse_ideas();
    let idea_localisations = parse_idea_localisations();

    let name = deunicode(country.name.as_str());
    let mut page_str = String::new();
    let mut set_name = "";
    if let Some(set) = ideas.idea_sets.get(&country.tag) {
        if let Some(name) = idea_localisations.get(&set.name) {
            set_name = name;
        }
    }
    page_str += format!(
        "{{{{Country Detail\n|tag={tag}\n|name={name}\n|primary_culture={culture}\n|religion={religion}\n|idea_group={ideas}\n}}}}\n",
        tag=country.tag,
        name=name,
        culture=deunicode(country.history.primary_culture.as_str()),
        religion=deunicode(country.history.religion.as_str()),
        ideas=deunicode(set_name)
    ).as_str();
    if mission_tags.contains(&country.tag) {
        page_str += "\n[[Category:Countries with missions]]\n";
    }
    client.add_edit_page(&name, page_str);
}

fn racial_modifiers(client: &mut MediaWikiClient) {
    let data = fs::read("./anbennar/common/event_modifiers/racial_admin_military.txt")
        .expect("Could not find racial modifiers file");
    let tape = TextTape::from_slice(data.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    let mut page_str = String::new();

    for (key, _op, value) in reader.fields() {
        if key.read_str().ends_with("administration") || key.read_str().ends_with("military") {
            let key = key.read_string();
            let title: Vec<_> = key.split('_').collect();
            let title = title.iter().map(|t| title_case(t)).collect::<Vec<_>>().join(" ");
            if let Ok(modifiers) = value.read_object() {
                page_str += format!("=== {} ===\n", title).as_str();
                for (key, _op, value) in modifiers.fields() {
                    if key.read_str() != "picture" {
                        if let Some(modifier) = get_modifier(&key.read_string()) {
                            if let Ok(value) = value.read_scalar() {
                                let mut colour = "bonus";
                                let scalar = value.to_f64();
                                if let Ok(value) = scalar {
                                    if value.is_sign_positive() && modifier.normal == Negative || value.is_sign_negative() && modifier.normal == Positive {
                                        colour = "malus";
                                    }
                                    let value = modifier.to_human_readable(value as f32);
                                    page_str += format!("* {{{{subst:Modifier |type={}|value={}|description={} }}}}\n", colour, value, modifier.name).as_str()
                                } else {
                                    page_str += format!("* {{{{subst:Modifier |type={}|value={}|description={} }}}}\n", colour, value, modifier.name).as_str()
                                }
                            }
                        }
                    }
                }
                page_str += "\n";
            }
        }
    }

    client.add_edit_page(&String::from("Racial_Modifiers"), page_str);
}

fn run_government_icons(client: &mut MediaWikiClient) {
    fn gather(path: String, files: &mut Vec<PathBuf>) -> Vec<PathBuf> {
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

    let mut files: Vec<PathBuf> = vec![];
    let paths = vec![
        "./anbennar/gfx/interface/government_reform_icons",
        "./basegame/gfx/interface/government_reform_icons"
    ];

    for path in paths {
        gather(path.to_string(), &mut files);
    }

    for file in files {
        if file.exists() {
            let name = file.as_path().to_str().unwrap().replace("\\", "/");
            if let Some(converted) = ImageMagick::default().convert_to_png(name.as_ref()) {
                client.upload(format!("gov_{}", converted.file_name().unwrap().to_str().unwrap()), &converted);
                let _ = fs::remove_file(converted);
            }
        }
    }
}

fn run_governments(client: &mut MediaWikiClient) {
    let governments = parse_governments();
    let localisations = parse_all_localisations();
    let reforms = parse_government_reforms(Some(&localisations));
    let reforms = reforms
        .iter()
        .map(|v|(v.id.clone(), v))
        .collect::<HashMap<_, _>>();

    for government in governments {
        match government.id.as_str() {
            "pre_dharma_mapping" => {}
            name => {
                let mut page_str = String::new();
                for level in government.reform_levels.keys() {
                    let reform_level = government.reform_levels.get(level).unwrap();
                    let level_name = localisations.get(&reform_level.id).unwrap();
                    page_str += format!("=== Tier {level}: {name} ===\n", level=level, name=level_name).as_str();
                    page_str += "{| class=\"reformtable\" \n|-\n! Icon !! Name !! Potential !! Effects !! Description\n";
                    for reform_id in &reform_level.reforms {
                        let reform = reforms.get(reform_id).unwrap();
                        if reform.name.as_ref().is_none() {
                            continue;
                        }
                        page_str += "|-\n";
                        match reform.icon.as_ref() {
                            None => {
                                page_str += "|| [[File:gov_placeholder.png]] "
                            }
                            Some(icon) => {
                                page_str += format!("|| [[File:gov_{}.png]] ", icon).as_str()
                            }
                        }
                        page_str += format!("|| {} \n", reform.name.as_ref().unwrap()).as_str();
                        if reform.potential.is_some() {
                            page_str += "|| ";
                            page_str += htmlify(&serde_json::from_str::<Value>(<Option<String> as Clone>::clone(&reform.potential).expect("could not unwrap").as_str()).unwrap()).as_str();
                        } else {
                            page_str += "|| ";
                        }
                        page_str += "|| ";
                        for (modifier_key, modifier_value) in &reform.modifiers {
                            let modifier = get_modifier(&modifier_key).unwrap();
                            let value = Scalar::new(modifier_value.as_slice());
                            if let Ok(value) = value.to_f64() {
                                let mut colour = "bonus";
                                if value.is_sign_positive() && modifier.normal == Negative || value.is_sign_negative() && modifier.normal == Positive {
                                    colour = "malus";
                                }
                                let value = modifier.to_human_readable(value as f32);
                                page_str += format!("\n* {{{{subst:Modifier |type={}|value={}|description={} }}}}", colour, value, modifier.name).as_str()
                            }
                        }
                        page_str += "\n|";

                        match reform.desc.as_ref() {
                            None => {page_str += "|\n"}
                            Some(mut desc) => {
                                let desc = desc.replace("\\n", "<br>");
                                page_str += format!("| {}\n", desc).as_str();
                            }
                        }
                    }
                    page_str += "|}\n";
                }
                // println!("{}", page_str);
                client.add_edit_page(&title_case(name), page_str);
            }
        }
    }
}

fn run_map(client: &mut MediaWikiClient) {
    let super_regions = parse_map();
    let continents = parse_continents();

    let mut province_list_page = String::new();
    province_list_page.push_str("{| class=\"wikitable sortable\" style=\"font-size:95%; text-align:left\"\n");
    province_list_page.push_str("! ID !! Name !! Continent !! Subcontinent !! Region !! Area\n");
    province_list_page.push_str("|-\n");

    let mut rows = BTreeMap::new();

    for subcontinent in super_regions {
        for region in subcontinent.regions {
            for area in region.areas {
                for province in area.provinces {
                    let continent = match continents.get(&province.id) {
                        Some(id) => {
                            match id.as_str() {
                                "africa" => "Sarhal",
                                "europe" => "Cannor",
                                "serpentspine" => "Serpentspine",
                                "asia" => "Haless",
                                "north_america" => "North Aelantir",
                                "south_america" => "South Aelantir",
                                "oceania" => "Insyaa",
                                _ => ""
                            }
                        }
                        None => {""}
                    };
                    rows.insert(
                        province.id,
                        format!(
                            "| {id} || {name} || {continent} || {subcontinent} || {region} || {area}\n",
                            id=province.id,
                            name=province.name.replace("|", "<nowiki>|</nowiki>"),
                            continent=continent,
                            subcontinent=subcontinent.name,
                            region=region.name,
                            area=area.name.replace("|", "<nowiki>|</nowiki>")
                        )
                    );
                }
            }
        }
    }

    for (province_id, _continent_id) in continents {
        if let Some(row) = rows.remove(&province_id) {
            province_list_page.push_str(row.as_str());
            province_list_page.push_str("|-\n");
        }
    }

    province_list_page.push_str("|}\n");
    client.add_edit_page(&"Geographical list of provinces".to_string(), province_list_page);
}