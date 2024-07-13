use std::string::String;

use phf::phf_map;

use crate::modifiers::ModifierFormat::{Flat, Percent};
use crate::modifiers::ModifierNormal::{Negative, Positive};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum ModifierFormat {
    Percent,
    Flat,
    #[default]
    None,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum ModifierNormal {
    #[default]
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Modifier {
    pub id: &'static str,
    pub name: &'static str,
    pub format: ModifierFormat,
    pub normal: ModifierNormal,
    pub multiplier: usize,
}

impl Modifier {
    pub fn to_human_readable(self, amount: f32) -> String {
        let inverted = vec![
            "reduced_liberty_desire",
            "reduced_liberty_desire_on_same_continent"
        ];
        let mut sign = "+";
        // if (amount < 0.0) || (amount > 0.0 && self.normal == Negative) {
        if (inverted.contains(&self.id) && amount > 0.0) || (amount < 0.0 && !inverted.contains(&self.id)) {
            sign = "-";
        }
        match self.format {
            Percent => {
                format!("{sign}{amount}%", amount = (amount * self.multiplier as f32).abs().round())
            }
            Flat => {
                format!("{sign}{amount}", amount = (amount * self.multiplier as f32).abs())
            }
            ModifierFormat::None => {
                "".to_string()
            }
        }
    }
}

pub fn get_modifier(id: &String) -> Option<Modifier> {
    match MODIFIERS.get(id.to_lowercase().as_str()) {
        Some(modifier) => {
            return Some(modifier.clone())
        }
        None => {
            None
        }
    }
}

pub fn localise_strings(description: &String, value: &String) -> (String, String) {
    let mut desc: String = description.clone();
    let mut val: String = value.clone();
    match MODIFIERS.get(description.to_lowercase().as_str()) {
        Some(modifier) => {
            desc = modifier.name.to_string();
            if let Ok(parsed) = value.parse::<f32>() {
                val = modifier.to_human_readable(parsed);
            }
            (desc.clone(), val.clone())
        },
        None => {
            panic!("No mapping for {}", desc);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_modifier() {
        let modifier = get_modifier(&"discipline".to_string());
        assert!(modifier.is_some());
        let modifier = get_modifier(&"disciplined".to_string());
        assert!(!modifier.is_some());
    }

    #[test]
    pub fn test_readable() {
        let modifier = get_modifier(&"reduced_liberty_desire".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(10f32), "-10%");

        let modifier = get_modifier(&"reduced_liberty_desire".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(-10f32), "+10%");

        let modifier = get_modifier(&"ae_impact".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(-0.1), "-10%");

        let modifier = get_modifier(&"ae_impact".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(0.1), "+10%");

        let modifier = get_modifier(&"accept_vassalization_reasons".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(10f32), "+10");

        let modifier = get_modifier(&"burghers_loyalty_modifier".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(0.1), "+10%");

        let modifier = get_modifier(&"burghers_loyalty_modifier".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(-0.1), "-10%");

        let modifier = get_modifier(&"adm_tech_cost_modifier".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(-0.1), "-10%");

        let modifier = get_modifier(&"adm_tech_cost_modifier".to_string());
        assert_eq!(modifier.unwrap().to_human_readable(0.1), "+10%");
    }

    #[test]
    pub fn test_modifier_localisation() {
        let localised = localise_strings(&"discipline".to_string(), &"0.05".to_string());
        assert_eq!(localised, ("Discipline".to_string(), "+5%".to_string()));
        let localised = localise_strings(&"advisor_pool".to_string(), &"-1".to_string());
        assert_eq!(localised, ("Possible Advisors".to_string(), "-1".to_string()));
    }
}


pub const MODIFIERS: phf::Map<&'static str, Modifier> = phf_map!{
    "accept_vassalization_reasons" => Modifier{ id: "accept_vassalization_reasons", name: "Vassalization Acceptance", format: Flat, normal: Positive, multiplier: 1 },
    "acolytes_influence_modifier" => Modifier{ id: "acolytes_influence_modifier", name: "Acolytes Influence", format: Percent, normal: Positive, multiplier: 100 },
    "adeen_loyalty_modifier" => Modifier{ id: "adeen_loyalty_modifier", name: "Adeen Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "adm_advisor_cost" => Modifier{ id: "adm_advisor_cost", name: "Administrative Advisor Cost", format: Percent, normal: Negative, multiplier: 100 },
    "adm_tech_cost_modifier" => Modifier{ id: "adm_tech_cost_modifier", name: "Administrative Technology Cost", format: Percent, normal: Negative, multiplier: 100 },
    "administrative_efficiency" => Modifier{ id: "administrative_efficiency", name: "Administrative Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "admiral_cost" => Modifier{ id: "admiral_cost", name: "Admiral Cost", format: Percent, normal: Negative, multiplier: 100 },
    "adventurers_loyalty_modifier" => Modifier{ id: "adventurers_loyalty_modifier", name: "Adventurers Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "advisor_cost" => Modifier{ id: "advisor_cost", name: "Advisor Cost", format: Percent, normal: Negative, multiplier: 100 },
    "advisor_pool" => Modifier{ id: "advisor_pool", name: "Possible Advisors", format: Flat, normal: Positive, multiplier: 1 },
    "ae_impact" => Modifier{ id: "ae_impact", name: "Aggressive Expansion Impact", format: Percent, normal: Negative, multiplier: 100 },
    "ahati_loyalty_modifier" => Modifier{ id: "ahati_loyalty_modifier", name: "Ahati Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "all_estate_loyalty_equilibrium" => Modifier{ id: "all_estate_loyalty_equilibrium", name: "All Estates' Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "all_power_cost" => Modifier{ id: "all_power_cost", name: "All Power Costs", format: Percent, normal: Negative, multiplier: 100 },
    "allowed_marine_fraction" => Modifier{ id: "allowed_marine_fraction", name: "Marines Force Limit", format: Percent, normal: Positive, multiplier: 100 },
    "allowed_rajput_fraction" => Modifier{ id: "allowed_rajput_fraction", name: "Allowed Rajput Fraction", format: Percent, normal: Positive, multiplier: 100 },
    "army_tradition" => Modifier{ id: "army_tradition", name: "Yearly Army Tradition", format: Flat, normal: Positive, multiplier: 1 },
    "army_tradition_decay" => Modifier{ id: "army_tradition_decay", name: "Yearly Army Tradition Decay", format: Percent, normal: Negative, multiplier: 100 },
    "army_tradition_from_battle" => Modifier{ id: "army_tradition_from_battle", name: "Army Tradition From Battles", format: Percent, normal: Positive, multiplier: 100 },
    "artificers_loyalty_modifier" => Modifier{ id: "artificers_loyalty_modifier", name: "Artificers Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "artillery_barrage_cost" => Modifier{ id: "artillery_barrage_cost", name: "Artillery Barrage Cost", format: Percent, normal: Negative, multiplier: 100 },
    "artillery_cost" => Modifier{ id: "artillery_cost", name: "Artillery Cost", format: Percent, normal: Negative, multiplier: 100 },
    "artillery_fire" => Modifier{ id: "artillery_fire", name: "Artillery Fire", format: Flat, normal: Positive, multiplier: 1 },
    "artillery_power" => Modifier{ id: "artillery_power", name: "Artillery Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "artillery_shock" => Modifier{ id: "artillery_shock", name: "Artillery Shock", format: Flat, normal: Positive, multiplier: 1 },
    "assault_fort_ability" => Modifier{ id: "assault_fort_ability", name: "Assault Fort ability", format: Percent, normal: Positive, multiplier: 100 },
    "autonomy_change_time" => Modifier{ id: "autonomy_change_time", name: "Autonomy Change Cooldown", format: Percent, normal: Negative, multiplier: 100 },
    "available_province_loot" => Modifier{ id: "available_province_loot", name: "Available Loot", format: Percent, normal: Positive, multiplier: 100 },
    "backrow_artillery_damage" => Modifier{ id: "backrow_artillery_damage", name: "Artillery Damage from Back Row", format: Percent, normal: Positive, multiplier: 100 },
    "blockade_efficiency" => Modifier{ id: "blockade_efficiency", name: "Blockade Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "build_cost" => Modifier{ id: "build_cost", name: "Construction Cost", format: Percent, normal: Negative, multiplier: 100 },
    "build_time" => Modifier{ id: "build_time", name: "Construction Time", format: Percent, normal: Negative, multiplier: 100 },
    "burghers_loyalty_modifier" => Modifier{ id: "burghers_loyalty_modifier", name: "Burghers Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "can_fabricate_for_vassals" => Modifier{ id: "can_fabricate_for_vassals", name: "May Fabricate Claims for Subjects", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "candidate_random_bonus" => Modifier{ id: "candidate_random_bonus", name: "Random Candidate Bonus", format: Flat, normal: Positive, multiplier: 1 },
    "capture_ship_chance" => Modifier{ id: "capture_ship_chance", name: "Chance to Capture Enemy Ships", format: Percent, normal: Positive, multiplier: 100 },
    "caravan_power" => Modifier{ id: "caravan_power", name: "Caravan Power", format: Percent, normal: Positive, multiplier: 100 },
    "cav_to_inf_ratio" => Modifier{ id: "cav_to_inf_ratio", name: "Cavalry to Infantry Ratio", format: Percent, normal: Positive, multiplier: 100 },
    "cavalry_cost" => Modifier{ id: "cavalry_cost", name: "Cavalry Cost", format: Percent, normal: Negative, multiplier: 100 },
    "cavalry_fire" => Modifier{ id: "cavalry_fire", name: "Cavalry Fire", format: Flat, normal: Positive, multiplier: 1 },
    "cavalry_flanking" => Modifier{ id: "cavalry_flanking", name: "Cavalry Flanking Ability", format: Percent, normal: Positive, multiplier: 100 },
    "cavalry_power" => Modifier{ id: "cavalry_power", name: "Cavalry Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "cavalry_shock" => Modifier{ id: "cavalry_shock", name: "Cavalry Shock", format: Flat, normal: Positive, multiplier: 1 },
    "center_of_trade_upgrade_cost" => Modifier{ id: "center_of_trade_upgrade_cost", name: "Center of Trade Upgrade Cost", format: Percent, normal: Negative, multiplier: 100 },
    "church_loyalty_modifier" => Modifier{ id: "church_loyalty_modifier", name: "Clergy Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "church_power_modifier" => Modifier{ id: "church_power_modifier", name: "Religious Power", format: Percent, normal: Positive, multiplier: 100 },
    "claim_duration" => Modifier{ id: "claim_duration", name: "Claim Duration", format: Percent, normal: Positive, multiplier: 100 },
    "coast_raid_range" => Modifier{ id: "coast_raid_range", name: "Coastal Raiding Range", format: Flat, normal: Positive, multiplier: 1 },
    "colonist_placement_chance" => Modifier{ id: "colonist_placement_chance", name: "Settler Chance", format: Percent, normal: Positive, multiplier: 100 },
    "colonists" => Modifier{ id: "colonists", name: "Colonists", format: Flat, normal: Positive, multiplier: 1 },
    "core_creation" => Modifier{ id: "core_creation", name: "Core-Creation Cost", format: Percent, normal: Negative, multiplier: 100 },
    "core_decay_on_your_own" => Modifier{ id: "core_decay_on_your_own", name: "Foreign Core Duration", format: Percent, normal: Negative, multiplier: 100 },
    "culture_conversion_cost" => Modifier{ id: "culture_conversion_cost", name: "Culture Conversion Cost", format: Percent, normal: Negative, multiplier: 100 },
    "culture_conversion_time" => Modifier{ id: "culture_conversion_time", name: "Culture Conversion Time", format: Percent, normal: Negative, multiplier: 100 },
    "defensiveness" => Modifier{ id: "defensiveness", name: "Fort Defence", format: Percent, normal: Positive, multiplier: 100 },
    "development_cost" => Modifier{ id: "development_cost", name: "Development Cost", format: Percent, normal: Negative, multiplier: 100 },
    "development_cost_in_primary_culture" => Modifier{ id: "development_cost_in_primary_culture", name: "Development Cost in Primary Culture", format: Percent, normal: Negative, multiplier: 100 },
    "devotion" => Modifier{ id: "devotion", name: "Yearly Devotion", format: Flat, normal: Positive, multiplier: 1 },
    "dip_advisor_cost" => Modifier{ id: "dip_advisor_cost", name: "Diplomatic Advisor Cost", format: Percent, normal: Negative, multiplier: 100 },
    "dip_tech_cost_modifier" => Modifier{ id: "dip_tech_cost_modifier", name: "Diplomatic Technology Cost", format: Percent, normal: Negative, multiplier: 100 },
    "diplomatic_annexation_cost" => Modifier{ id: "diplomatic_annexation_cost", name: "Diplomatic Annexation Cost", format: Percent, normal: Negative, multiplier: 100 },
    "diplomatic_reputation" => Modifier{ id: "diplomatic_reputation", name: "Diplomatic Reputation", format: Flat, normal: Positive, multiplier: 1 },
    "diplomatic_upkeep" => Modifier{ id: "diplomatic_upkeep", name: "Diplomatic Relations", format: Flat, normal: Positive, multiplier: 1 },
    "diplomats" => Modifier{ id: "diplomats", name: "Diplomats", format: Flat, normal: Positive, multiplier: 1 },
    "discipline" => Modifier{ id: "discipline", name: "Discipline", format: Percent, normal: Positive, multiplier: 100 },
    "discovered_relations_impact" => Modifier{ id: "discovered_relations_impact", name: "Covert Action Relation Impact", format: Percent, normal: Negative, multiplier: 100 },
    "disengagement_chance" => Modifier{ id: "disengagement_chance", name: "Ship Disengagement Chance", format: Percent, normal: Positive, multiplier: 100 },
    "dragon_command_influence_modifier" => Modifier{ id: "dragon_command_influence_modifier", name: "Dragon Command Influence", format: Percent, normal: Positive, multiplier: 100 },
    "dragon_command_loyalty_modifier" => Modifier{ id: "dragon_command_loyalty_modifier", name: "Dragon Command Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "drill_decay_modifier" => Modifier{ id: "drill_decay_modifier", name: "Regiment Drill Loss", format: Percent, normal: Negative, multiplier: 100 },
    "drill_gain_modifier" => Modifier{ id: "drill_gain_modifier", name: "Army Drill Gain Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "elephant_command_influence_modifier" => Modifier{ id: "elephant_command_influence_modifier", name: "Elephant Command Influence", format: Percent, normal: Positive, multiplier: 100 },
    "elephant_command_loyalty_modifier" => Modifier{ id: "elephant_command_loyalty_modifier", name: "Elephant Command Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "embargo_efficiency" => Modifier{ id: "embargo_efficiency", name: "Embargo Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "embracement_cost" => Modifier{ id: "embracement_cost", name: "Institution Embracement Cost", format: Percent, normal: Negative, multiplier: 100 },
    "enforce_religion_cost" => Modifier{ id: "enforce_religion_cost", name: "Cost of enforcing religion through war", format: Percent, normal: Negative, multiplier: 100 },
    "envoy_travel_time" => Modifier{ id: "envoy_travel_time", name: "Envoy Travel Time", format: Percent, normal: Negative, multiplier: 100 },
    "establish_order_cost" => Modifier{ id: "establish_order_cost", name: "Establish Local Organization Cost", format: Percent, normal: Negative, multiplier: 100 },
    "expand_administration_cost" => Modifier{ id: "expand_administration_cost", name: "Expand Administration Cost", format: Percent, normal: Negative, multiplier: 100 },
    "fabricate_claims_cost" => Modifier{ id: "fabricate_claims_cost", name: "Cost to fabricate claims", format: Percent, normal: Negative, multiplier: 100 },
    "female_advisor_chance" => Modifier{ id: "female_advisor_chance", name: "Female Advisor Chance", format: Percent, normal: Positive, multiplier: 100 },
    "fire_damage" => Modifier{ id: "fire_damage", name: "Land Fire Damage", format: Percent, normal: Positive, multiplier: 100 },
    "fire_damage_received" => Modifier{ id: "fire_damage_received", name: "Fire Damage Received", format: Percent, normal: Negative, multiplier: 100 },
    "flagship_cost" => Modifier{ id: "flagship_cost", name: "Flagship Cost", format: Percent, normal: Negative, multiplier: 100 },
    "fort_maintenance_modifier" => Modifier{ id: "fort_maintenance_modifier", name: "Fort Maintenance", format: Percent, normal: Negative, multiplier: 100 },
    "free_adm_policy" => Modifier{ id: "free_adm_policy", name: "Administrative Free Policies", format: Flat, normal: Positive, multiplier: 1 },
    "free_dip_policy" => Modifier{ id: "free_dip_policy", name: "Diplomatic Free Policies", format: Flat, normal: Positive, multiplier: 1 },
    "free_land_leader_pool" => Modifier{ id: "free_land_leader_pool", name: "Land Leader(s) without Upkeep", format: Flat, normal: Positive, multiplier: 1 },
    "free_leader_pool" => Modifier{ id: "free_leader_pool", name: "Leader(s) without Upkeep", format: Flat, normal: Positive, multiplier: 1 },
    "free_mil_policy" => Modifier{ id: "free_mil_policy", name: "Military Free Policies", format: Flat, normal: Positive, multiplier: 1 },
    "free_policy" => Modifier{ id: "free_policy", name: "Free Policies", format: Flat, normal: Positive, multiplier: 1 },
    "galley_cost" => Modifier{ id: "galley_cost", name: "Galley Cost", format: Percent, normal: Negative, multiplier: 100 },
    "galley_power" => Modifier{ id: "galley_power", name: "Galley Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "garrison_size" => Modifier{ id: "garrison_size", name: "Garrison Size", format: Percent, normal: Positive, multiplier: 100 },
    "general_cost" => Modifier{ id: "general_cost", name: "General Cost", format: Percent, normal: Negative, multiplier: 100 },
    "global_autonomy" => Modifier{ id: "global_autonomy", name: "Monthly Autonomy Change", format: Flat, normal: Negative, multiplier: 1 },
    "global_colonial_growth" => Modifier{ id: "global_colonial_growth", name: "Global Settler Increase", format: Flat, normal: Positive, multiplier: 1 },
    "global_foreign_trade_power" => Modifier{ id: "global_foreign_trade_power", name: "Trade Power Abroad", format: Percent, normal: Positive, multiplier: 100 },
    "global_garrison_growth" => Modifier{ id: "global_garrison_growth", name: "National Garrison Growth", format: Percent, normal: Positive, multiplier: 100 },
    "global_heathen_missionary_strength" => Modifier{ id: "global_heathen_missionary_strength", name: "Missionary Strength vs Heathens", format: Percent, normal: Positive, multiplier: 100 },
    "global_heretic_missionary_strength" => Modifier{ id: "global_heretic_missionary_strength", name: "Missionary Strength vs Heretics", format: Percent, normal: Positive, multiplier: 100 },
    "global_institution_spread" => Modifier{ id: "global_institution_spread", name: "Institution Spread", format: Percent, normal: Positive, multiplier: 100 },
    "global_manpower_modifier" => Modifier{ id: "global_manpower_modifier", name: "National Manpower Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_missionary_strength" => Modifier{ id: "global_missionary_strength", name: "Missionary Strength", format: Percent, normal: Positive, multiplier: 100 },
    "global_monthly_devastation" => Modifier{ id: "global_monthly_devastation", name: "Global Monthly Devastation", format: Flat, normal: Negative, multiplier: 1 },
    "global_naval_barrage_cost" => Modifier{ id: "global_naval_barrage_cost", name: "Naval Barrage cost", format: Percent, normal: Negative, multiplier: 100 },
    "global_naval_engagement_modifier" => Modifier{ id: "global_naval_engagement_modifier", name: "Global Naval Engagement Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_own_trade_power" => Modifier{ id: "global_own_trade_power", name: "Domestic Trade Power", format: Percent, normal: Positive, multiplier: 100 },
    "global_prosperity_growth" => Modifier{ id: "global_prosperity_growth", name: "Global Prosperity Growth", format: Percent, normal: Positive, multiplier: 100 },
    "global_prov_trade_power_modifier" => Modifier{ id: "global_prov_trade_power_modifier", name: "Provincial Trade Power Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_rebel_suppression_efficiency" => Modifier{ id: "global_rebel_suppression_efficiency", name: "Rebel Suppression Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "global_regiment_cost" => Modifier{ id: "global_regiment_cost", name: "Regiment Costs", format: Percent, normal: Negative, multiplier: 100 },
    "global_regiment_recruit_speed" => Modifier{ id: "global_regiment_recruit_speed", name: "Recruitment Time", format: Percent, normal: Negative, multiplier: 100 },
    "global_sailors" => Modifier{ id: "global_sailors", name: "Sailor Increase", format: Flat, normal: Positive, multiplier: 1 },
    "global_sailors_modifier" => Modifier{ id: "global_sailors_modifier", name: "National Sailors Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_ship_cost" => Modifier{ id: "global_ship_cost", name: "Ship Costs", format: Percent, normal: Negative, multiplier: 100 },
    "global_ship_recruit_speed" => Modifier{ id: "global_ship_recruit_speed", name: "Shipbuilding Time", format: Percent, normal: Negative, multiplier: 100 },
    "global_ship_repair" => Modifier{ id: "global_ship_repair", name: "Global Ship Repair", format: Percent, normal: Positive, multiplier: 100 },
    "global_ship_trade_power" => Modifier{ id: "global_ship_trade_power", name: "Ship Trade Power", format: Percent, normal: Positive, multiplier: 100 },
    "global_spy_defence" => Modifier{ id: "global_spy_defence", name: "Foreign Spy Detection", format: Percent, normal: Positive, multiplier: 100 },
    "global_supply_limit_modifier" => Modifier{ id: "global_supply_limit_modifier", name: "National Supply Limit Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_tariffs" => Modifier{ id: "global_tariffs", name: "Merchants", format: Percent, normal: Positive, multiplier: 100 },
    "global_tax_modifier" => Modifier{ id: "global_tax_modifier", name: "National Tax Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_trade_goods_size_modifier" => Modifier{ id: "global_trade_goods_size_modifier", name: "Goods Produced Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "global_trade_power" => Modifier{ id: "global_trade_power", name: "Global Trade Power", format: Percent, normal: Positive, multiplier: 100 },
    "global_unrest" => Modifier{ id: "global_unrest", name: "National Unrest", format: Flat, normal: Negative, multiplier: 1 },
    "governing_capacity_modifier" => Modifier{ id: "governing_capacity_modifier", name: "Governing Capacity Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "great_project_upgrade_cost" => Modifier{ id: "great_project_upgrade_cost", name: "Great Project Upgrade Cost", format: Percent, normal: Negative, multiplier: 100 },
    "harsh_treatment_cost" => Modifier{ id: "harsh_treatment_cost", name: "Harsh Treatment Cost", format: Percent, normal: Negative, multiplier: 100 },
    "heavy_ship_cost" => Modifier{ id: "heavy_ship_cost", name: "Heavy Ship Cost", format: Percent, normal: Negative, multiplier: 100 },
    "heavy_ship_hull_size_modifier" => Modifier{ id: "heavy_ship_hull_size_modifier", name: "Heavy Ship Hull Size", format: Percent, normal: Positive, multiplier: 100 },
    "heavy_ship_power" => Modifier{ id: "heavy_ship_power", name: "Heavy Ship Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "heir_chance" => Modifier{ id: "heir_chance", name: "Chance of New Heir", format: Percent, normal: Positive, multiplier: 100 },
    "horde_unity" => Modifier{ id: "horde_unity", name: "Yearly Horde Unity", format: Flat, normal: Positive, multiplier: 1 },
    "hostile_attrition" => Modifier{ id: "hostile_attrition", name: "Attrition for Enemies", format: Flat, normal: Positive, multiplier: 1 },
    "hull_size_modifier" => Modifier{ id: "hull_size_modifier", name: "Ship Hull Size", format: Percent, normal: Positive, multiplier: 100 },
    "idea_claim_colonies" => Modifier{ id: "idea_claim_colonies", name: "Can Fabricate Claims in any colonial region belonging to another nation who are also overseas from the province, or to their colonial nations", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "idea_cost" => Modifier{ id: "idea_cost", name: "Idea Cost", format: Percent, normal: Negative, multiplier: 100 },
    "imperial_authority" => Modifier{ id: "imperial_authority", name: "Imperial Authority Growth Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "imperial_mandate" => Modifier{ id: "imperial_mandate", name: "Monthly Mandate", format: Flat, normal: Positive, multiplier: 1 },
    "improve_relation_modifier" => Modifier{ id: "improve_relation_modifier", name: "Improve Relations", format: Percent, normal: Positive, multiplier: 100 },
    "infantry_cost" => Modifier{ id: "infantry_cost", name: "Infantry Cost", format: Percent, normal: Negative, multiplier: 100 },
    "infantry_fire" => Modifier{ id: "infantry_fire", name: "Infantry Fire", format: Flat, normal: Positive, multiplier: 1 },
    "infantry_power" => Modifier{ id: "infantry_power", name: "Infantry Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "infantry_shock" => Modifier{ id: "infantry_shock", name: "Infantry Shock", format: Flat, normal: Positive, multiplier: 1 },
    "inflation_action_cost" => Modifier{ id: "inflation_action_cost", name: "Reduce Inflation Cost", format: Percent, normal: Negative, multiplier: 100 },
    "inflation_reduction" => Modifier{ id: "inflation_reduction", name: "Yearly Inflation Reduction", format: Flat, normal: Positive, multiplier: 1 },
    "innovativeness_gain" => Modifier{ id: "innovativeness_gain", name: "Innovativeness Gain", format: Percent, normal: Positive, multiplier: 100 },
    "institution_spread_from_true_faith" => Modifier{ id: "institution_spread_from_true_faith", name: "Institution Spread In True Faith Provinces", format: Percent, normal: Positive, multiplier: 100 },
    "interest" => Modifier{ id: "interest", name: "Interest Per Annum", format: Flat, normal: Negative, multiplier: 1 },
    "justify_trade_conflict_cost" => Modifier{ id: "justify_trade_conflict_cost", name: "Cost to justify trade conflict", format: Percent, normal: Negative, multiplier: 100 },
    "land_attrition" => Modifier{ id: "land_attrition", name: "Land Attrition", format: Percent, normal: Negative, multiplier: 100 },
    "land_forcelimit_modifier" => Modifier{ id: "land_forcelimit_modifier", name: "Land Force Limit Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "land_maintenance_modifier" => Modifier{ id: "land_maintenance_modifier", name: "Land Maintenance Modifier", format: Percent, normal: Negative, multiplier: 100 },
    "land_morale" => Modifier{ id: "land_morale", name: "Morale of Armies", format: Percent, normal: Positive, multiplier: 100 },
    "leader_cost" => Modifier{ id: "leader_cost", name: "Leader Cost", format: Percent, normal: Negative, multiplier: 100 },
    "leader_land_fire" => Modifier{ id: "leader_land_fire", name: "Land Leader Fire", format: Flat, normal: Positive, multiplier: 1 },
    "leader_land_manuever" => Modifier{ id: "leader_land_manuever", name: "Land Leader Manoeuvre", format: Flat, normal: Positive, multiplier: 1 },
    "leader_land_shock" => Modifier{ id: "leader_land_shock", name: "Land Leader Shock", format: Flat, normal: Positive, multiplier: 1 },
    "leader_naval_fire" => Modifier{ id: "leader_naval_fire", name: "Naval Leader Fire", format: Flat, normal: Positive, multiplier: 1 },
    "leader_naval_manuever" => Modifier{ id: "leader_naval_manuever", name: "Naval Leader Manoeuvre", format: Flat, normal: Positive, multiplier: 1 },
    "leader_naval_shock" => Modifier{ id: "leader_naval_shock", name: "Naval Leader Shock", format: Flat, normal: Positive, multiplier: 1 },
    "leader_siege" => Modifier{ id: "leader_siege", name: "Leader Siege", format: Flat, normal: Positive, multiplier: 1 },
    "legitimacy" => Modifier{ id: "legitimacy", name: "Yearly Legitimacy", format: Flat, normal: Positive, multiplier: 1 },
    "liberty_desire_from_subject_development" => Modifier{ id: "liberty_desire_from_subject_development", name: "Liberty Desire from Subject Development", format: Percent, normal: Negative, multiplier: 100 },
    "light_ship_cost" => Modifier{ id: "light_ship_cost", name: "Light Ship Cost", format: Percent, normal: Negative, multiplier: 100 },
    "light_ship_power" => Modifier{ id: "light_ship_power", name: "Light Ship Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "loot_amount" => Modifier{ id: "loot_amount", name: "Looting Speed", format: Percent, normal: Positive, multiplier: 100 },
    "lowercastes_loyalty_modifier" => Modifier{ id: "lowercastes_loyalty_modifier", name: "Lower Castes Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "mages_loyalty_modifier" => Modifier{ id: "mages_loyalty_modifier", name: "Mages Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "manpower_in_accepted_culture_provinces" => Modifier{ id: "manpower_in_accepted_culture_provinces", name: "Manpower in Accepted Culture provinces", format: Percent, normal: Positive, multiplier: 100 },
    "manpower_in_culture_group_provinces" => Modifier{ id: "manpower_in_culture_group_provinces", name: "Manpower in same Culture Group provinces", format: Percent, normal: Positive, multiplier: 100 },
    "manpower_in_own_culture_provinces" => Modifier{ id: "manpower_in_own_culture_provinces", name: "Manpower in Primary Culture provinces", format: Percent, normal: Positive, multiplier: 100 },
    "manpower_in_true_faith_provinces" => Modifier{ id: "manpower_in_true_faith_provinces", name: "Manpower in True Faith provinces", format: Percent, normal: Positive, multiplier: 100 },
    "manpower_recovery_speed" => Modifier{ id: "manpower_recovery_speed", name: "Manpower Recovery Speed", format: Percent, normal: Positive, multiplier: 100 },
    "max_absolutism" => Modifier{ id: "max_absolutism", name: "Maximum Absolutism", format: Flat, normal: Positive, multiplier: 1 },
    "max_absolutism_effect" => Modifier{ id: "max_absolutism_effect", name: "Max Effect of Absolutism", format: Percent, normal: Positive, multiplier: 100 },
    "max_general_maneuver" => Modifier{ id: "max_general_maneuver", name: "Max General Manoeuvre", format: Flat, normal: Positive, multiplier: 1 },
    "max_hostile_attrition" => Modifier{ id: "max_hostile_attrition", name: "Max Hostile Attrition", format: Flat, normal: Positive, multiplier: 1 },
    "max_revolutionary_zeal" => Modifier{ id: "max_revolutionary_zeal", name: "Maximum Revolutionary Zeal", format: Flat, normal: Positive, multiplier: 1 },
    "may_explore" => Modifier{ id: "may_explore", name: "Allows recruitment of explorers & conquistadors", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "may_perform_slave_raid" => Modifier{ id: "may_perform_slave_raid", name: "May Raid Coasts", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "may_perform_slave_raid_on_same_religion" => Modifier{ id: "may_perform_slave_raid_on_same_religion", name: "May Raid Coasts, including coasts of countries with same religion", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "may_recruit_female_generals" => Modifier{ id: "may_recruit_female_generals", name: "May Recruit Female Generals", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "merc_leader_army_tradition" => Modifier{ id: "merc_leader_army_tradition", name: "Mercenary Leader Army Tradition", format: Percent, normal: Positive, multiplier: 100 },
    "merc_maintenance_modifier" => Modifier{ id: "merc_maintenance_modifier", name: "Mercenary Maintenance", format: Percent, normal: Negative, multiplier: 100 },
    "mercantilism_cost" => Modifier{ id: "mercantilism_cost", name: "Cost to Promote Mercantilism", format: Percent, normal: Negative, multiplier: 100 },
    "mercenary_cost" => Modifier{ id: "mercenary_cost", name: "Mercenary Cost", format: Percent, normal: Negative, multiplier: 100 },
    "mercenary_discipline" => Modifier{ id: "mercenary_discipline", name: "Mercenary Discipline", format: Percent, normal: Positive, multiplier: 100 },
    "mercenary_manpower" => Modifier{ id: "mercenary_manpower", name: "Mercenary Manpower", format: Percent, normal: Positive, multiplier: 100 },
    "merchants" => Modifier{ id: "merchants", name: "Merchants", format: Flat, normal: Positive, multiplier: 1 },
    "migration_cost" => Modifier{ id: "migration_cost", name: "Migration Cost", format: Percent, normal: Negative, multiplier: 100 },
    "mil_advisor_cost" => Modifier{ id: "mil_advisor_cost", name: "Military Advisor Cost", format: Percent, normal: Negative, multiplier: 100 },
    "mil_tech_cost_modifier" => Modifier{ id: "mil_tech_cost_modifier", name: "Military Technology Cost", format: Percent, normal: Negative, multiplier: 100 },
    "missionaries" => Modifier{ id: "missionaries", name: "Missionaries", format: Flat, normal: Positive, multiplier: 1 },
    "missionary_maintenance_cost" => Modifier{ id: "missionary_maintenance_cost", name: "Missionary Maintenance Cost", format: Percent, normal: Negative, multiplier: 100 },
    "monarch_admin_power" => Modifier{ id: "monarch_admin_power", name: "Monarch Administrative Skill", format: Flat, normal: Positive, multiplier: 1 },
    "monarch_diplomatic_power" => Modifier{ id: "monarch_diplomatic_power", name: "Monarch Diplomatic Skill", format: Flat, normal: Positive, multiplier: 1 },
    "monarch_lifespan" => Modifier{ id: "monarch_lifespan", name: "Average Monarch Lifespan", format: Percent, normal: Positive, multiplier: 100 },
    "monarch_military_power" => Modifier{ id: "monarch_military_power", name: "Monarch Military Skill", format: Flat, normal: Positive, multiplier: 1 },
    "monstrous_tribes_loyalty_modifier" => Modifier{ id: "monstrous_tribes_loyalty_modifier", name: "Monstrous Tribes Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "monthly_church_power" => Modifier{ id: "monthly_church_power", name: "Monthly Faith Power", format: Percent, normal: Positive, multiplier: 100 },
    "monthly_fervor_increase" => Modifier{ id: "monthly_fervor_increase", name: "Monthly Fervour", format: Flat, normal: Positive, multiplier: 1 },
    "monthly_gold_inflation_modifier" => Modifier{ id: "monthly_gold_inflation_modifier", name: "Monthly Gold Inflation Multiplier", format: Percent, normal: Negative, multiplier: 100 },
    "monthly_heir_claim_increase" => Modifier{ id: "monthly_heir_claim_increase", name: "Monthly Heir Claim Increase", format: Flat, normal: Positive, multiplier: 1 },
    "monthly_reform_progress_modifier" => Modifier{ id: "monthly_reform_progress_modifier", name: "Monthly Reform Progress Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "monthly_splendor" => Modifier{ id: "monthly_splendor", name: "Monthly Splendour", format: Flat, normal: Positive, multiplier: 1 },
    "morale_damage" => Modifier{ id: "morale_damage", name: "Morale Damage", format: Percent, normal: Positive, multiplier: 100 },
    "morale_damage_received" => Modifier{ id: "morale_damage_received", name: "Morale Damage Received", format: Percent, normal: Negative, multiplier: 100 },
    "move_capital_cost_modifier" => Modifier{ id: "move_capital_cost_modifier", name: "Move Capital cost modifier", format: Percent, normal: Negative, multiplier: 100 },
    "movement_speed" => Modifier{ id: "movement_speed", name: "Movement Speed", format: Percent, normal: Positive, multiplier: 100 },
    "movement_speed_in_fleet_modifier" => Modifier{ id: "movement_speed_in_fleet_modifier", name: "Fleet Movement Speed", format: Percent, normal: Positive, multiplier: 100 },
    "movement_speed_onto_off_boat_modifier" => Modifier{ id: "movement_speed_onto_off_boat_modifier", name: "Movement Speed On and Off Ships", format: Percent, normal: Positive, multiplier: 100 },
    "national_focus_years" => Modifier{ id: "national_focus_years", name: "Change National Focus Cooldown Years", format: Flat, normal: Negative, multiplier: 1 },
    "native_assimilation" => Modifier{ id: "native_assimilation", name: "Native Assimilation", format: Percent, normal: Positive, multiplier: 100 },
    "native_uprising_chance" => Modifier{ id: "native_uprising_chance", name: "Native Uprising Chance", format: Percent, normal: Negative, multiplier: 100 },
    "naval_attrition" => Modifier{ id: "naval_attrition", name: "Naval Attrition", format: Percent, normal: Negative, multiplier: 100 },
    "naval_forcelimit_modifier" => Modifier{ id: "naval_forcelimit_modifier", name: "Naval Force Limit Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "naval_maintenance_modifier" => Modifier{ id: "naval_maintenance_modifier", name: "Naval Maintenance Modifier", format: Percent, normal: Negative, multiplier: 100 },
    "naval_morale" => Modifier{ id: "naval_morale", name: "Morale of Navies", format: Percent, normal: Positive, multiplier: 100 },
    "naval_morale_damage" => Modifier{ id: "naval_morale_damage", name: "Naval Morale Damage", format: Percent, normal: Positive, multiplier: 100 },
    "naval_morale_damage_received" => Modifier{ id: "naval_morale_damage_received", name: "Naval Morale Damage Received", format: Percent, normal: Negative, multiplier: 100 },
    "naval_tradition_from_battle" => Modifier{ id: "naval_tradition_from_battle", name: "Naval Tradition From Battles", format: Percent, normal: Positive, multiplier: 100 },
    "naval_tradition_from_trade" => Modifier{ id: "naval_tradition_from_trade", name: "Naval Tradition From Trade", format: Percent, normal: Positive, multiplier: 100 },
    "navy_tradition" => Modifier{ id: "navy_tradition", name: "Yearly Navy Tradition", format: Flat, normal: Positive, multiplier: 1 },
    "navy_tradition_decay" => Modifier{ id: "navy_tradition_decay", name: "Yearly Naval Tradition Decay", format: Percent, normal: Negative, multiplier: 100 },
    "no_religion_penalty" => Modifier{ id: "no_religion_penalty", name: "Heretic and heathen provinces do not give any penalties", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "nobles_influence_modifier" => Modifier{ id: "nobles_influence_modifier", name: "Nobility Influence", format: Percent, normal: Negative, multiplier: 100 },
    "nobles_loyalty_modifier" => Modifier{ id: "nobles_loyalty_modifier", name: "Nobility Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "num_accepted_cultures" => Modifier{ id: "num_accepted_cultures", name: "Max Promoted Cultures", format: Flat, normal: Positive, multiplier: 1 },
    "number_of_cannons_modifier" => Modifier{ id: "number_of_cannons_modifier", name: "Number of Cannons Modifier", format: Percent, normal: Negative, multiplier: 100 },
    "own_coast_naval_combat_bonus" => Modifier{ id: "own_coast_naval_combat_bonus", name: "Naval Combat Bonus off owned coast", format: Flat, normal: Positive, multiplier: 1 },
    "papal_influence" => Modifier{ id: "papal_influence", name: "Yearly Rectorate Influence", format: Flat, normal: Positive, multiplier: 1 },
    "papal_influence_from_cardinals" => Modifier{ id: "papal_influence_from_cardinals", name: "Rectorate Influence from Veridicals", format: Percent, normal: Positive, multiplier: 100 },
    "placed_merchant_power" => Modifier{ id: "placed_merchant_power", name: "Merchant Trade Power", format: Flat, normal: Positive, multiplier: 1 },
    "possible_adm_policy" => Modifier{ id: "possible_adm_policy", name: "Administrative Possible Policies", format: Flat, normal: Positive, multiplier: 1 },
    "possible_condottieri" => Modifier{ id: "possible_condottieri", name: "Possible Condottieri", format: Percent, normal: Positive, multiplier: 100 },
    "possible_dip_policy" => Modifier{ id: "possible_dip_policy", name: "Diplomatic Possible Policies", format: Flat, normal: Positive, multiplier: 1 },
    "possible_mil_policy" => Modifier{ id: "possible_mil_policy", name: "Military Possible Policies", format: Flat, normal: Positive, multiplier: 1 },
    "possible_policy" => Modifier{ id: "possible_policy", name: "Possible Policies", format: Flat, normal: Positive, multiplier: 1 },
    "power_projection_from_insults" => Modifier{ id: "power_projection_from_insults", name: "Power Projection From Insults", format: Percent, normal: Positive, multiplier: 100 },
    "prestige" => Modifier{ id: "prestige", name: "Yearly Prestige", format: Flat, normal: Positive, multiplier: 1 },
    "prestige_decay" => Modifier{ id: "prestige_decay", name: "Prestige Decay", format: Percent, normal: Negative, multiplier: 100 },
    "prestige_from_land" => Modifier{ id: "prestige_from_land", name: "Prestige from Land battles", format: Percent, normal: Positive, multiplier: 100 },
    "prestige_from_naval" => Modifier{ id: "prestige_from_naval", name: "Prestige from Naval battles", format: Percent, normal: Positive, multiplier: 100 },
    "prestige_per_development_from_conversion" => Modifier{ id: "prestige_per_development_from_conversion", name: "Prestige per Development From Conversion", format: Percent, normal: Positive, multiplier: 100 },
    "privateer_efficiency" => Modifier{ id: "privateer_efficiency", name: "Privateer Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "production_efficiency" => Modifier{ id: "production_efficiency", name: "Production Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "promote_culture_cost" => Modifier{ id: "promote_culture_cost", name: "Promote Culture Cost", format: Percent, normal: Negative, multiplier: 100 },
    "province_warscore_cost" => Modifier{ id: "province_warscore_cost", name: "Province War Score Cost", format: Percent, normal: Negative, multiplier: 100 },
    "raj_ministries_loyalty_modifier" => Modifier{ id: "raj_ministries_loyalty_modifier", name: "Raj Ministries Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "range" => Modifier{ id: "range", name: "Colonial Range", format: Percent, normal: Positive, multiplier: 100 },
    "raze_power_gain" => Modifier{ id: "raze_power_gain", name: "Razing Power Gain", format: Percent, normal: Positive, multiplier: 100 },
    "rebel_support_efficiency" => Modifier{ id: "rebel_support_efficiency", name: "Rebel Support Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "recover_army_morale_speed" => Modifier{ id: "recover_army_morale_speed", name: "Recover Army Morale Speed", format: Percent, normal: Positive, multiplier: 100 },
    "reduced_liberty_desire" => Modifier{ id: "reduced_liberty_desire", name: "Liberty Desire in Subjects", format: Percent, normal: Positive, multiplier: 1 },
    "reduced_liberty_desire_on_other_continent" => Modifier{ id: "reduced_liberty_desire_on_other_continent", name: "Liberty Desire in Other Continent Subjects", format: Percent, normal: Positive, multiplier: 1 },
    "reduced_liberty_desire_on_same_continent" => Modifier{ id: "reduced_liberty_desire_on_same_continent", name: "Liberty Desire in Same Continent Subjects", format: Percent, normal: Positive, multiplier: 1 },
    "reelection_cost" => Modifier{ id: "reelection_cost", name: "Reelection Cost", format: Percent, normal: Negative, multiplier: 100 },
    "reform_progress_growth" => Modifier{ id: "reform_progress_growth", name: "Reform Progress Growth", format: Percent, normal: Positive, multiplier: 100 },
    "reinforce_cost_modifier" => Modifier{ id: "reinforce_cost_modifier", name: "Reinforce Cost", format: Percent, normal: Negative, multiplier: 100 },
    "reinforce_speed" => Modifier{ id: "reinforce_speed", name: "Reinforce Speed", format: Percent, normal: Positive, multiplier: 100 },
    "religious_unity" => Modifier{ id: "religious_unity", name: "Religious Unity", format: Percent, normal: Positive, multiplier: 100 },
    "republican_tradition" => Modifier{ id: "republican_tradition", name: "Yearly Republican Tradition", format: Flat, normal: Positive, multiplier: 1 },
    "reserves_organisation" => Modifier{ id: "reserves_organisation", name: "Reduced Morale Damage Taken By Reserves", format: Percent, normal: Positive, multiplier: 100 },
    "rival_border_fort_maintenance" => Modifier{ id: "rival_border_fort_maintenance", name: "Fort Maintenance on Border with Rival", format: Percent, normal: Negative, multiplier: 100 },
    "rival_change_cost" => Modifier{ id: "rival_change_cost", name: "Change Rival Cost", format: Percent, normal: Negative, multiplier: 100 },
    "sailor_maintenance_modifer" => Modifier{ id: "sailor_maintenance_modifer", name: "Sailor Maintenance", format: Percent, normal: Negative, multiplier: 100 },
    "sailor_maintenance_modifier" => Modifier{ id: "sailor_maintenance_modifier", name: "Sailor Maintenance", format: Percent, normal: Negative, multiplier: 100 },
    "sailors_recovery_speed" => Modifier{ id: "sailors_recovery_speed", name: "Sailor Recovery Speed", format: Percent, normal: Positive, multiplier: 100 },
    "same_culture_advisor_cost" => Modifier{ id: "same_culture_advisor_cost", name: "Cost of Advisors with Ruler's Culture", format: Percent, normal: Negative, multiplier: 100 },
    "same_religion_advisor_cost" => Modifier{ id: "same_religion_advisor_cost", name: "Cost of Advisors with Ruler's Religion", format: Percent, normal: Negative, multiplier: 100 },
    "sea_repair" => Modifier{ id: "sea_repair", name: "Ships can repair when in coastal sea zones", format: ModifierFormat::None, normal: Positive, multiplier: 1 },
    "ship_durability" => Modifier{ id: "ship_durability", name: "Ship Durability", format: Percent, normal: Positive, multiplier: 100 },
    "ship_power_propagation" => Modifier{ id: "ship_power_propagation", name: "Ship Tradepower Propagation", format: Percent, normal: Positive, multiplier: 100 },
    "shock_damage" => Modifier{ id: "shock_damage", name: "Shock Damage", format: Percent, normal: Positive, multiplier: 100 },
    "shock_damage_received" => Modifier{ id: "shock_damage_received", name: "Shock Damage Received", format: Percent, normal: Negative, multiplier: 100 },
    "siege_ability" => Modifier{ id: "siege_ability", name: "Siege Ability", format: Percent, normal: Positive, multiplier: 100 },
    "siege_blockade_progress" => Modifier{ id: "siege_blockade_progress", name: "Blockade Impact on Siege", format: Flat, normal: Positive, multiplier: 1 },
    "spy_action_cost_modifier" => Modifier{ id: "spy_action_cost_modifier", name: "Spy Action Cost Modifier", format: Percent, normal: Negative, multiplier: 100 },
    "spy_offence" => Modifier{ id: "spy_offence", name: "Spy Network Construction", format: Percent, normal: Positive, multiplier: 100 },
    "stability_cost_modifier" => Modifier{ id: "stability_cost_modifier", name: "Stability Cost Modifier", format: Percent, normal: Negative, multiplier: 100 },
    "stability_cost_to_declare_war" => Modifier{ id: "stability_cost_to_declare_war", name: "Stability Hit to Declare War", format: Flat, normal: Negative, multiplier: 1 },
    "state_governing_cost" => Modifier{ id: "state_governing_cost", name: "States Governing Cost", format: Percent, normal: Negative, multiplier: 100 },
    "state_maintenance_modifier" => Modifier{ id: "state_maintenance_modifier", name: "State Maintenance", format: Percent, normal: Negative, multiplier: 100 },
    "sunk_ship_morale_hit_received" => Modifier{ id: "sunk_ship_morale_hit_received", name: "Morale Hit When Losing a Ship", format: Percent, normal: Negative, multiplier: 100 },
    "sunk_ship_morale_hit_recieved" => Modifier{ id: "sunk_ship_morale_hit_recieved", name: "Morale Hit When Losing a Ship", format: Percent, normal: Negative, multiplier: 100 },
    "supply_limit_modifier" => Modifier{ id: "supply_limit_modifier", name: "Supply Limit Modifier", format: Percent, normal: Positive, multiplier: 100 },
    "technology_cost" => Modifier{ id: "technology_cost", name: "Technology Cost", format: Percent, normal: Negative, multiplier: 100 },
    "tiger_command_influence_modifier" => Modifier{ id: "tiger_command_influence_modifier", name: "Tiger Command Influence", format: Percent, normal: Positive, multiplier: 100 },
    "tiger_command_loyalty_modifier" => Modifier{ id: "tiger_command_loyalty_modifier", name: "Tiger Command Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "tolerance_heathen" => Modifier{ id: "tolerance_heathen", name: "Tolerance of Heathens", format: Flat, normal: Positive, multiplier: 1 },
    "tolerance_heretic" => Modifier{ id: "tolerance_heretic", name: "Tolerance of Heretics", format: Flat, normal: Positive, multiplier: 1 },
    "tolerance_own" => Modifier{ id: "tolerance_own", name: "Tolerance of the True Faith", format: Flat, normal: Positive, multiplier: 1 },
    "trade_company_investment_cost" => Modifier{ id: "trade_company_investment_cost", name: "Trade Company Investment Cost", format: Percent, normal: Negative, multiplier: 100 },
    "trade_efficiency" => Modifier{ id: "trade_efficiency", name: "Trade Efficiency", format: Percent, normal: Positive, multiplier: 100 },
    "trade_range_modifier" => Modifier{ id: "trade_range_modifier", name: "Trade Range", format: Percent, normal: Positive, multiplier: 100 },
    "trade_steering" => Modifier{ id: "trade_steering", name: "Trade Steering", format: Percent, normal: Positive, multiplier: 100 },
    "transport_cost" => Modifier{ id: "transport_cost", name: "Transport Cost", format: Percent, normal: Negative, multiplier: 100 },
    "transport_power" => Modifier{ id: "transport_power", name: "Transport Ship Combat Ability", format: Percent, normal: Positive, multiplier: 100 },
    "unjustified_demands" => Modifier{ id: "unjustified_demands", name: "Unjustified Demands", format: Percent, normal: Negative, multiplier: 100 },
    "uppercastes_loyalty_modifier" => Modifier{ id: "uppercastes_loyalty_modifier", name: "Upper Castes Loyalty Equilibrium", format: Percent, normal: Positive, multiplier: 100 },
    "vassal_forcelimit_bonus" => Modifier{ id: "vassal_forcelimit_bonus", name: "Vassal Force Limit Contribution", format: Percent, normal: Positive, multiplier: 100 },
    "vassal_income" => Modifier{ id: "vassal_income", name: "Income from Vassals", format: Percent, normal: Positive, multiplier: 100 },
    "war_exhaustion" => Modifier{ id: "war_exhaustion", name: "Monthly War Exhaustion", format: Flat, normal: Negative, multiplier: 1 },
    "war_exhaustion_cost" => Modifier{ id: "war_exhaustion_cost", name: "Cost of Reducing War Exhaustion", format: Percent, normal: Negative, multiplier: 100 },
    "war_taxes_cost_modifier" => Modifier{ id: "war_taxes_cost_modifier", name: "War Taxes Cost", format: Percent, normal: Negative, multiplier: 100 },
    "warscore_cost_vs_other_religion" => Modifier{ id: "warscore_cost_vs_other_religion", name: "War Score Cost vs Other Religions", format: Percent, normal: Negative, multiplier: 100 },
    "yearly_absolutism" => Modifier{ id: "yearly_absolutism", name: "Yearly Absolutism", format: Flat, normal: Positive, multiplier: 1 },
    "yearly_army_professionalism" => Modifier{ id: "yearly_army_professionalism", name: "Yearly Army Professionalism", format: Percent, normal: Positive, multiplier: 100 },
    "yearly_corruption" => Modifier{ id: "yearly_corruption", name: "Yearly Corruption", format: Flat, normal: Negative, multiplier: 1 },
    "yearly_government_power" => Modifier{ id: "yearly_government_power", name: "Yearly Government Power", format: Flat, normal: Positive, multiplier: 1 },
    "yearly_harmony" => Modifier{ id: "yearly_harmony", name: "Yearly Harmony", format: Flat, normal: Negative, multiplier: 1 },
    "yearly_karma_decay" => Modifier{ id: "yearly_karma_decay", name: "Yearly Corinite Paragonhood Decay", format: Flat, normal: Positive, multiplier: 1 },
    "yearly_patriarch_authority" => Modifier{ id: "yearly_patriarch_authority", name: "Yearly Demonic Power", format: Flat, normal: Negative, multiplier: 1 },
    "years_of_nationalism" => Modifier{ id: "years_of_nationalism", name: "Years of Separatism", format: Flat, normal: Negative, multiplier: 1 },
};
