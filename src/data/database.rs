use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use log::{error, info};
use rusqlite::{Connection, Result as SqlResult, Transaction};

use crate::data::staff::Staff;
use crate::data::team::{League, Team};
use crate::data::sponsor::{Sponsor, FA};
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

    pub fn load_sponsors(&self) -> Result<Vec<Sponsor>> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut stmt = conn.prepare(
            "SELECT SponsorName, Type, Unlocked, Description, BrandOffer, ChestOffer, 
            BackOffer, SleeveOffer, BillboardOffer, BibOffer, BannerOffer,
            HeadquarterLocation, Industry, LocationRestriction
            FROM Sponsor ORDER BY SponsorName"
        )?;
        
        // 直接使用 rows() 而不是 query_map 以便手动处理类型转换
        let mut rows = stmt.query([])?;
        let mut sponsors = Vec::new();
        
        while let Some(row) = rows.next()? {
            let sponsor_name: String = row.get(0)?;
            let sponsor_type: String = row.get(1)?;
            // 将整数类型转换为字符串
            let unlocked_int: i64 = row.get(2)?;
            let unlocked = unlocked_int.to_string();
            let description: String = row.get(3)?;
            
            // 将所有数值字段转换为字符串
            let brand_offer_int: i64 = row.get(4)?;
            let chest_offer_int: i64 = row.get(5)?;
            let back_offer_int: i64 = row.get(6)?;
            let sleeve_offer_int: i64 = row.get(7)?;
            let billboard_offer_int: i64 = row.get(8)?;
            let bib_offer_int: i64 = row.get(9)?;
            let banner_offer_int: i64 = row.get(10)?;
            
            let headquarter_location: String = row.get(11)?;
            let industry: String = row.get(12)?;
            let location_restriction: String = row.get(13)?;
            
            let sponsor = Sponsor {
                sponsor_name,
                sponsor_type,
                unlocked,
                description,
                brand_offer: brand_offer_int.to_string(),
                chest_offer: chest_offer_int.to_string(),
                back_offer: back_offer_int.to_string(),
                sleeve_offer: sleeve_offer_int.to_string(),
                billboard_offer: billboard_offer_int.to_string(),
                bib_offer: bib_offer_int.to_string(),
                banner_offer: banner_offer_int.to_string(),
                headquarter_location,
                industry,
                location_restriction,
                logo_path: None,
            };
            
            sponsors.push(sponsor);
        }
        
        // 设置logo路径
        let sponsors = sponsors.into_iter()
            .map(|mut sponsor| {
                if let Some(db_dir) = self.get_db_directory() {
                    let logo_dir = db_dir.join("SponsorLogos");
                    let logo_path = logo_dir.join(format!("{}.png", sponsor.sponsor_name));
                    if logo_path.exists() {
                        sponsor.logo_path = Some(logo_path);
                    }
                }
                sponsor
            })
            .collect();
        
        Ok(sponsors)
    }

    pub fn load_fas(&self) -> Result<Vec<FA>> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        let mut stmt = conn.prepare(
            "SELECT ID, Title, Location, SubsidyLevel, MainOperatorName, YouthOperatorName, 
            CompetitionOperatorName, YouthDevelopment, YouthOperatorRelation,
            YouthOperatorAbility, CompetitionOperatorRelation, CompetitionOperatorAbility, 
            MainOperatorRelation, MainOperatorAbility, MainOperatorFame, 
            YouthOperatorFame, CompetitionOperatorFame
            FROM FA ORDER BY Title"
        )?;
        
        // 直接使用 rows() 而不是 query_map 以便手动处理类型转换
        let mut rows = stmt.query([])?;
        let mut fas = Vec::new();
        
        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let location: String = row.get(2)?;
            
            // 将整数类型转换为字符串
            let subsidy_level_int: i64 = row.get(3)?;
            let subsidy_level = subsidy_level_int.to_string();
            
            let main_operator_name: String = row.get(4)?;
            let youth_operator_name: String = row.get(5)?;
            let competition_operator_name: String = row.get(6)?;
            
            // 将所有数值字段转换为字符串
            let youth_development_int: i64 = row.get(7)?;
            let youth_operator_relation_int: i64 = row.get(8)?;
            let youth_operator_ability_int: i64 = row.get(9)?;
            let competition_operator_relation_int: i64 = row.get(10)?;
            let competition_operator_ability_int: i64 = row.get(11)?;
            let main_operator_relation_int: i64 = row.get(12)?;
            let main_operator_ability_int: i64 = row.get(13)?;
            let main_operator_fame_int: i64 = row.get(14)?;
            let youth_operator_fame_int: i64 = row.get(15)?;
            let competition_operator_fame_int: i64 = row.get(16)?;
            
            let fa = FA {
                id,
                title,
                location,
                subsidy_level,
                main_operator_name,
                youth_operator_name,
                competition_operator_name,
                youth_development: youth_development_int.to_string(),
                youth_operator_relation: youth_operator_relation_int.to_string(),
                youth_operator_ability: youth_operator_ability_int.to_string(),
                competition_operator_relation: competition_operator_relation_int.to_string(),
                competition_operator_ability: competition_operator_ability_int.to_string(),
                main_operator_relation: main_operator_relation_int.to_string(),
                main_operator_ability: main_operator_ability_int.to_string(),
                main_operator_fame: main_operator_fame_int.to_string(),
                youth_operator_fame: youth_operator_fame_int.to_string(),
                competition_operator_fame: competition_operator_fame_int.to_string(),
            };
            
            fas.push(fa);
        }
        
        Ok(fas)
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

    pub fn update_sponsor(&self, sponsor: &Sponsor) -> Result<()> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        conn.execute(
            "UPDATE Sponsor SET 
            Type = ?1, 
            Unlocked = ?2, 
            Description = ?3, 
            BrandOffer = ?4, 
            ChestOffer = ?5, 
            BackOffer = ?6, 
            SleeveOffer = ?7, 
            BillboardOffer = ?8, 
            BibOffer = ?9, 
            BannerOffer = ?10, 
            HeadquarterLocation = ?11, 
            Industry = ?12, 
            LocationRestriction = ?13
            WHERE SponsorName = ?14",
            (
                &sponsor.sponsor_type,
                sponsor.unlocked.parse::<i64>().unwrap_or(0),
                &sponsor.description,
                sponsor.brand_offer.parse::<i64>().unwrap_or(0),
                sponsor.chest_offer.parse::<i64>().unwrap_or(0),
                sponsor.back_offer.parse::<i64>().unwrap_or(0),
                sponsor.sleeve_offer.parse::<i64>().unwrap_or(0),
                sponsor.billboard_offer.parse::<i64>().unwrap_or(0),
                sponsor.bib_offer.parse::<i64>().unwrap_or(0),
                sponsor.banner_offer.parse::<i64>().unwrap_or(0),
                &sponsor.headquarter_location,
                &sponsor.industry,
                &sponsor.location_restriction,
                &sponsor.sponsor_name,
            ),
        )?;
        
        Ok(())
    }

    pub fn create_new_sponsor(&self, sponsor: &Sponsor) -> Result<()> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        conn.execute(
            "INSERT INTO Sponsor (
            SponsorName, Type, Unlocked, Description, BrandOffer, ChestOffer, 
            BackOffer, SleeveOffer, BillboardOffer, BibOffer, BannerOffer,
            HeadquarterLocation, Industry, LocationRestriction)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            (
                &sponsor.sponsor_name,
                &sponsor.sponsor_type,
                sponsor.unlocked.parse::<i64>().unwrap_or(0),
                &sponsor.description,
                sponsor.brand_offer.parse::<i64>().unwrap_or(0),
                sponsor.chest_offer.parse::<i64>().unwrap_or(0),
                sponsor.back_offer.parse::<i64>().unwrap_or(0),
                sponsor.sleeve_offer.parse::<i64>().unwrap_or(0),
                sponsor.billboard_offer.parse::<i64>().unwrap_or(0),
                sponsor.bib_offer.parse::<i64>().unwrap_or(0),
                sponsor.banner_offer.parse::<i64>().unwrap_or(0),
                &sponsor.headquarter_location,
                &sponsor.industry,
                &sponsor.location_restriction,
            ),
        )?;
        
        Ok(())
    }

    pub fn update_fa(&self, fa: &FA) -> Result<()> {
        let conn = self.conn.as_ref().ok_or(AppError::DatabaseNotLoaded)?;
        
        conn.execute(
            "UPDATE FA SET 
            Title = ?1, 
            Location = ?2, 
            SubsidyLevel = ?3
            WHERE ID = ?4",
            (&fa.title, &fa.location, fa.subsidy_level.parse::<i64>().unwrap_or(0), &fa.id),
        )?;
        
        conn.execute(
            "UPDATE FA SET 
            MainOperatorName = ?1, 
            YouthOperatorName = ?2, 
            CompetitionOperatorName = ?3
            WHERE ID = ?4",
            (&fa.main_operator_name, &fa.youth_operator_name, &fa.competition_operator_name, &fa.id),
        )?;
        
        conn.execute(
            "UPDATE FA SET 
            YouthDevelopment = ?1, 
            YouthOperatorRelation = ?2, 
            YouthOperatorAbility = ?3
            WHERE ID = ?4",
            (
                fa.youth_development.parse::<i64>().unwrap_or(0), 
                fa.youth_operator_relation.parse::<i64>().unwrap_or(0), 
                fa.youth_operator_ability.parse::<i64>().unwrap_or(0), 
                &fa.id
            ),
        )?;
        
        conn.execute(
            "UPDATE FA SET 
            CompetitionOperatorRelation = ?1, 
            CompetitionOperatorAbility = ?2, 
            MainOperatorRelation = ?3
            WHERE ID = ?4",
            (
                fa.competition_operator_relation.parse::<i64>().unwrap_or(0), 
                fa.competition_operator_ability.parse::<i64>().unwrap_or(0), 
                fa.main_operator_relation.parse::<i64>().unwrap_or(0), 
                &fa.id
            ),
        )?;
        
        conn.execute(
            "UPDATE FA SET 
            MainOperatorAbility = ?1, 
            MainOperatorFame = ?2, 
            YouthOperatorFame = ?3,
            CompetitionOperatorFame = ?4
            WHERE ID = ?5",
            (
                fa.main_operator_ability.parse::<i64>().unwrap_or(0), 
                fa.main_operator_fame.parse::<i64>().unwrap_or(0), 
                fa.youth_operator_fame.parse::<i64>().unwrap_or(0), 
                fa.competition_operator_fame.parse::<i64>().unwrap_or(0), 
                &fa.id
            ),
        )?;
        
        Ok(())
    }
} 