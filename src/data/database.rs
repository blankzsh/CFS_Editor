use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use log::{error, info};
use rusqlite::{Connection, Result as SqlResult, Transaction};

use crate::data::staff::Staff;
use crate::data::team::{League, Team};
use crate::error::{AppError, Result};

pub struct Database {
    conn: Option<Connection>,
    db_path: Option<PathBuf>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            conn: None,
            db_path: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.conn.is_some()
    }

    pub fn get_db_directory(&self) -> Option<PathBuf> {
        self.db_path.as_ref().map(|p| p.parent().unwrap().to_path_buf())
    }

    pub fn connect(&mut self, path: &Path) -> Result<()> {
        let conn = Connection::open(path)?;
        self.db_path = Some(path.to_path_buf());
        self.conn = Some(conn);
        info!("数据库连接成功: {}", path.display());
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if let Some(conn) = self.conn.take() {
            drop(conn);
            self.db_path = None;
            info!("数据库连接已关闭");
        }
        Ok(())
    }

    pub fn load_teams(&self) -> Result<Vec<Team>> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut stmt = conn.prepare(
            "SELECT ID, TeamName, TeamWealth, TeamFoundYear, TeamLocation, 
            SupporterCount, StadiumName, Nickname, BelongingLeague 
            FROM Teams ORDER BY TeamName"
        )?;
        
        let teams = stmt
            .query_map([], |row| Team::from_row(&row))?
            .collect::<SqlResult<Vec<_>>>()?;
        
        Ok(teams)
    }

    pub fn load_leagues(&self) -> Result<HashMap<i64, String>> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut stmt = conn.prepare("SELECT ID, LeagueName FROM League")?;
        
        let leagues = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<SqlResult<HashMap<_, _>>>()?;
        
        Ok(leagues)
    }

    pub fn load_staff(&self) -> Result<Vec<Staff>> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut stmt = conn.prepare(
            "SELECT ID, Name, AbilityJSON, Fame, EmployedTeamID 
            FROM Staff ORDER BY Name"
        )?;
        
        let staff = stmt
            .query_map([], |row| Staff::from_row(&row))?
            .collect::<SqlResult<Vec<_>>>()?;
        
        Ok(staff)
    }

    pub fn update_team(&self, team: &Team) -> Result<()> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        conn.execute(
            "UPDATE Teams SET 
            TeamName = ?1, 
            TeamWealth = ?2, 
            TeamFoundYear = ?3, 
            TeamLocation = ?4, 
            SupporterCount = ?5, 
            StadiumName = ?6, 
            Nickname = ?7,
            BelongingLeague = ?8
            WHERE ID = ?9",
            (
                &team.name,
                &team.wealth,
                &team.found_year,
                &team.location,
                &team.supporter_count,
                &team.stadium_name,
                &team.nickname,
                &team.league_id,
                &team.id,
            ),
        )?;
        
        Ok(())
    }
    
    pub fn update_teams_batch(&self, teams: &[Team]) -> Result<usize> {
        if teams.is_empty() {
            return Ok(0);
        }
        
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut updated_count = 0;
        
        for team in teams {
            conn.execute(
                "UPDATE Teams SET 
                TeamName = ?1, 
                TeamWealth = ?2, 
                TeamFoundYear = ?3, 
                TeamLocation = ?4, 
                SupporterCount = ?5, 
                StadiumName = ?6, 
                Nickname = ?7,
                BelongingLeague = ?8
                WHERE ID = ?9",
                (
                    &team.name,
                    &team.wealth,
                    &team.found_year,
                    &team.location,
                    &team.supporter_count,
                    &team.stadium_name,
                    &team.nickname,
                    &team.league_id,
                    &team.id,
                ),
            )?;
            
            updated_count += 1;
        }
        
        info!("批量更新了 {} 个球队", updated_count);
        
        Ok(updated_count)
    }

    pub fn update_staff(&self, staff: &Staff) -> Result<()> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        conn.execute(
            "UPDATE Staff SET 
            Name = ?1, 
            AbilityJSON = ?2, 
            Fame = ?3 
            WHERE ID = ?4",
            (&staff.name, &staff.ability_json, &staff.fame, &staff.id),
        )?;
        
        Ok(())
    }
    
    pub fn update_staff_batch(&self, staff_list: &[Staff]) -> Result<usize> {
        if staff_list.is_empty() {
            return Ok(0);
        }
        
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut updated_count = 0;
        
        for staff in staff_list {
            conn.execute(
                "UPDATE Staff SET 
                Name = ?1, 
                AbilityJSON = ?2, 
                Fame = ?3 
                WHERE ID = ?4",
                (&staff.name, &staff.ability_json, &staff.fame, &staff.id),
            )?;
            
            updated_count += 1;
        }
        
        info!("批量更新了 {} 个员工", updated_count);
        
        Ok(updated_count)
    }
} 