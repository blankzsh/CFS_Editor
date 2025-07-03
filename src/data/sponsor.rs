use rusqlite::{Row, Result as SqlResult};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Sponsor {
    pub sponsor_name: String,
    pub sponsor_type: String,
    pub unlocked: String,
    pub description: String,
    pub brand_offer: String,
    pub chest_offer: String,
    pub back_offer: String,
    pub sleeve_offer: String,
    pub billboard_offer: String,
    pub bib_offer: String,
    pub banner_offer: String,
    pub headquarter_location: String,
    pub industry: String,
    pub location_restriction: String,
    pub logo_path: Option<PathBuf>,
}

impl Sponsor {
    pub fn new() -> Self {
        Self {
            sponsor_name: String::new(),
            sponsor_type: "Generic".to_string(),
            unlocked: "0".to_string(),
            description: String::new(),
            brand_offer: "0".to_string(),
            chest_offer: "0".to_string(),
            back_offer: "0".to_string(),
            sleeve_offer: "0".to_string(),
            billboard_offer: "0".to_string(),
            bib_offer: "0".to_string(),
            banner_offer: "0".to_string(),
            headquarter_location: String::new(),
            industry: String::new(),
            location_restriction: String::new(),
            logo_path: None,
        }
    }

    pub fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            sponsor_name: row.get(0)?,
            sponsor_type: row.get(1)?,
            unlocked: row.get(2)?,
            description: row.get(3)?,
            brand_offer: row.get(4)?,
            chest_offer: row.get(5)?,
            back_offer: row.get(6)?,
            sleeve_offer: row.get(7)?,
            billboard_offer: row.get(8)?,
            bib_offer: row.get(9)?,
            banner_offer: row.get(10)?,
            headquarter_location: row.get(11)?,
            industry: row.get(12)?,
            location_restriction: row.get(13)?,
            logo_path: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FA {
    pub id: i64,
    pub title: String,
    pub location: String,
    pub subsidy_level: String,
    pub main_operator_name: String,
    pub youth_operator_name: String,
    pub competition_operator_name: String,
    pub youth_development: String,
    pub youth_operator_relation: String,
    pub youth_operator_ability: String,
    pub competition_operator_relation: String,
    pub competition_operator_ability: String,
    pub main_operator_relation: String,
    pub main_operator_ability: String,
    pub main_operator_fame: String,
    pub youth_operator_fame: String,
    pub competition_operator_fame: String,
}

impl FA {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(),
            location: String::new(),
            subsidy_level: "0".to_string(),
            main_operator_name: String::new(),
            youth_operator_name: String::new(),
            competition_operator_name: String::new(),
            youth_development: "0".to_string(),
            youth_operator_relation: "0".to_string(),
            youth_operator_ability: "0".to_string(),
            competition_operator_relation: "0".to_string(),
            competition_operator_ability: "0".to_string(),
            main_operator_relation: "0".to_string(),
            main_operator_ability: "0".to_string(),
            main_operator_fame: "0".to_string(),
            youth_operator_fame: "0".to_string(),
            competition_operator_fame: "0".to_string(),
        }
    }

    pub fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            location: row.get(2)?,
            subsidy_level: row.get(3)?,
            main_operator_name: row.get(4)?,
            youth_operator_name: row.get(5)?,
            competition_operator_name: row.get(6)?,
            youth_development: row.get(7)?,
            youth_operator_relation: row.get(8)?,
            youth_operator_ability: row.get(9)?,
            competition_operator_relation: row.get(10)?,
            competition_operator_ability: row.get(11)?,
            main_operator_relation: row.get(12)?,
            main_operator_ability: row.get(13)?,
            main_operator_fame: row.get(14)?,
            youth_operator_fame: row.get(15)?,
            competition_operator_fame: row.get(16)?,
        })
    }
} 