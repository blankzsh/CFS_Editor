use rusqlite::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i64,
    pub name: String,
    pub ability_json: String,
    pub fame: i64,
    pub team_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityData {
    pub raw_ability: i64,
}

impl Staff {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Staff {
            id: row.get(0)?,
            name: row.get(1)?,
            ability_json: row.get(2)?,
            fame: row.get(3)?,
            team_id: row.get(4)?,
        })
    }

    pub fn get_ability(&self) -> Result<i64> {
        let ability_data: Value = serde_json::from_str(&self.ability_json)
            .map_err(|e| AppError::JsonError(e))?;
        
        match ability_data.get("rawAbility") {
            Some(Value::Number(n)) => {
                if let Some(value) = n.as_i64() {
                    Ok(value)
                } else {
                    Ok(0)
                }
            },
            _ => Ok(0),
        }
    }

    pub fn update_ability(&mut self, new_ability: i64) -> Result<()> {
        let json = format!(r#"{{"rawAbility":{}}}"#, new_ability);
        self.ability_json = json;
        Ok(())
    }
}

impl fmt::Display for Staff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ID: {})", self.name, self.id)
    }
} 