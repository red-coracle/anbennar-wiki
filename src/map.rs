use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use jomini::{JominiDeserialize, TextTape};

use crate::localisation::parse_all_localisations;

#[derive(Debug, Eq)]
pub struct Province {
    pub id: u64,
    pub name: String,
    pub adj: String,
    pub history: Option<ProvinceHistory>
    // pub owner: String,
    // pub culture: String,
    // pub religion: String,
}

impl PartialEq for Province {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Province {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Borrow<u64> for Province {
    fn borrow(&self) -> &u64 {
        &self.id
    }
}

#[derive(Debug, JominiDeserialize, Eq, PartialEq)]
#[derive(Clone)]
pub struct ProvinceHistory {
    #[jomini(take_last)]
    pub owner: Option<String>,
    pub controller: Option<String>,
    pub culture: Option<String>,
    pub religion: Option<String>,
    pub base_tax: Option<u64>,
    pub base_production: Option<u64>,
    pub base_manpower: Option<u64>,
    #[jomini(take_last)]
    pub trade_goods: Option<String>,
    pub is_city: Option<bool>
}

#[derive(Debug, PartialEq)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub provinces: HashSet<Province>,
}
impl Eq for Area {}

impl Hash for Area {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<String> for Area {
    fn borrow(&self) -> &String {
        &self.id
    }
}

#[derive(Debug, Eq)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub areas: HashSet<Area>
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Region {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Borrow<String> for Region {
    fn borrow(&self) -> &String {
        &self.id
    }
}

#[derive(Debug)]
pub struct SuperRegion {
    pub id: String,
    pub name: String,
    pub regions: HashSet<Region>,
    pub restrict_charter: bool
}

pub fn parse_continents() -> BTreeMap<u64, String> {
    let file = fs::read("./anbennar/map/continent.txt").expect("error reading file");
    let tape = TextTape::from_slice(file.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    let mut data: BTreeMap<u64, String> = BTreeMap::new();

    for (continent, _op, provinces) in reader.fields() {
        let continent = continent.read_string();

        if let Ok(provinces) = provinces.read_array() {
            for province in provinces.values() {
                data.insert(province.read_scalar().unwrap().to_u64().unwrap(), continent.clone());
            }
        }
    }
    data
}

pub fn parse_continents_inverse() -> HashMap<String, Vec<u64>> {
    let mut data =  HashMap::new();
    for (k, v) in parse_continents() {
        data.entry(v).or_insert_with(Vec::new).push(k);
    }
    data
}

pub fn parse_map() -> Vec<SuperRegion> {
    // Continent > Super region (subcontinent) > Region > Area > Province
    
    let localisations = parse_all_localisations();
    let histories = parse_province_histories();

    // AREAS & PROVINCES
    let file = fs::read("./anbennar/map/area.txt").expect("error reading file");
    let tape = TextTape::from_slice(file.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    let mut areas = HashSet::new();
    for (area_id, _op, value) in reader.fields() {
        let area_id_str = area_id.read_str();
        let name = localisations.get(&format!("{area_id_str}_name")).unwrap_or(&"".to_string()).clone();
        let mut area = Area{
            id: area_id.read_string(),
            name,
            provinces: Default::default(),
        };
        if let Ok(area_provinces) = value.read_array() {
            for province in area_provinces.values() {
                let id = province.read_scalar().unwrap().to_u64().unwrap();
                let name = localisations.get(&format!("PROV{id}")).unwrap_or(&"".to_string()).clone();
                let adj = localisations.get(&format!("PROV_ADJ{id}")).unwrap_or(&"".to_string()).clone();
                let history = histories.get(&id);
                let p = Province{
                    id,
                    name,
                    adj,
                    history: history.cloned(),
                };
                area.provinces.insert(p);
            }
        }
        areas.insert(area);

        // if area.provinces.len() > 0 {
        //     // https://bitbucket.org/JayBean/anbennar-eu4-fork-public-build/commits/afa40a19a69f78a208fc58cd941cda6e69a0fdc0#chg-map/area.txt
        //     areas.insert(area);
        // }
    }

    // REGIONS
    let file = fs::read("./anbennar/map/region.txt").expect("error reading file");
    let tape = TextTape::from_slice(file.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    let mut regions = HashSet::new();
    for (region_id, _op, value) in reader.fields() {
        let region_id = region_id.read_string();
        let mut region = Region{
            id: region_id.clone(),
            name: localisations.get(&format!("{region_id}_name")).unwrap_or(&"".to_string()).clone(),
            areas: Default::default(),
        };
        if let Ok(r) = value.read_object() {
            for (k, _op, v) in r.fields() {
                if k.read_str() == "areas" {
                    if let Ok(region_areas) = v.read_array() {
                        for area in region_areas.values() {
                            region.areas.insert(areas.take(&area.read_string().unwrap()).unwrap());
                            // if areas.take(&area.read_string().unwrap()).is_none() {
                            //     println!("{:?}", area.read_string());
                            // }
                        }
                    }
                }
            }
        }
        if region.areas.len() > 0 {
            regions.insert(region);
        }
    }

    // SUPER REGIONS
    let file = fs::read("./anbennar/map/superregion.txt").expect("error reading file");
    let tape = TextTape::from_slice(file.as_slice()).unwrap();
    let reader = tape.windows1252_reader();
    let mut super_regions = vec![];
    for (sr_id, _op, value) in reader.fields() {
        let sr_id = sr_id.read_string();
        let mut super_region = SuperRegion{
            id: sr_id.clone(),
            name: localisations.get(&format!("{sr_id}")).unwrap_or(&"".to_string()).clone(),
            regions: Default::default(),
            restrict_charter: false,
        };

        if let Ok(sr_regions) = value.read_array() {
            for region in sr_regions.values() {
                let region_id = region.read_str().unwrap();
                if region_id == "restrict_charter" {
                    super_region.restrict_charter = true;
                } else {
                    super_region.regions.insert(regions.take(&region_id.to_string()).unwrap());
                }
            }
        }

        if super_region.regions.len() > 0 {
            // ignore empty
            super_regions.push(super_region);
        }
    }

    super_regions
}

pub fn parse_province_histories() -> BTreeMap<u64, ProvinceHistory> {
    let mut histories = BTreeMap::new();
    let paths = fs::read_dir("./anbennar/history/provinces")
        .expect("Missing province history directory");
    for path in paths {
        match path {
            Ok(file) => {
                let file_name = file.path();
                let mut name = file_name.file_stem().unwrap().to_str().unwrap();
                if name.contains('-') {
                    name = name.split('-').next().unwrap().trim();
                }
                let id = name.parse::<u64>().unwrap();
                let file = fs::read(file.path()).expect("error reading file");
                let history = jomini::TextDeserializer::from_windows1252_slice(file.as_slice());
                let history: ProvinceHistory = history.unwrap().deserialize().unwrap();
                histories.insert(id, history);
            }
            _ => {}
        }
        // break;
    }
    histories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let map = parse_map();
    }

    #[test]
    fn test_parse_continents() {
        parse_continents();
    }

    #[test]
    fn test_parse_continents_inverse() {
        parse_continents_inverse();
    }
}