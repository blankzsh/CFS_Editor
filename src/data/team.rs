use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub wealth: i64,
    pub found_year: i64,
    pub location: String,
    pub supporter_count: i64,
    pub stadium_name: String,
    pub nickname: String,
    pub league_id: i64,
}

impl Team {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Team {
            id: row.get(0)?,
            name: row.get(1)?,
            wealth: row.get(2)?,
            found_year: row.get(3)?,
            location: row.get(4)?,
            supporter_count: row.get(5)?,
            stadium_name: row.get(6)?,
            nickname: row.get(7)?,
            league_id: row.get(8)?,
        })
    }

    pub fn search_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.id,
            self.name,
            self.wealth,
            self.found_year,
            self.location,
            self.supporter_count,
            self.stadium_name,
            self.nickname,
            self.league_id
        )
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ID: {})", self.name, self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct League {
    pub id: i64,
    pub name: String,
} 