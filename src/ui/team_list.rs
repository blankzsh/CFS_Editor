use egui::{Color32, RichText, ScrollArea, Ui, Stroke, Rounding, ComboBox};
use log::info;

use crate::data::team::Team;
use crate::ui::widgets;

#[derive(PartialEq, Clone, Copy)]
pub enum FilterField {
    Name,
    Location,
    League,
    All,
}

impl FilterField {
    fn as_str(&self) -> &'static str {
        match self {
            FilterField::Name => "球队名称",
            FilterField::Location => "地区",
            FilterField::League => "联赛",
            FilterField::All => "全部字段",
        }
    }
}

pub struct TeamListView {
    pub teams: Vec<Team>,
    pub filtered_teams: Vec<Team>,
    pub selected_index: Option<usize>,
    pub search_text: String,
    pub filter_field: FilterField,
    pub unique_locations: Vec<String>,
    pub selected_location: Option<String>,
    pub unique_leagues: Vec<i64>,
    pub selected_league: Option<i64>,
    pub show_advanced_filters: bool,
    pub min_wealth: Option<i64>,
    pub max_wealth: Option<i64>,
    pub min_year: Option<i64>,
    pub max_year: Option<i64>,
    pub wealth_filter_text: String,
    pub year_filter_text: String,
}

impl TeamListView {
    pub fn new() -> Self {
        TeamListView {
            teams: Vec::new(),
            filtered_teams: Vec::new(),
            selected_index: None,
            search_text: String::new(),
            filter_field: FilterField::All,
            unique_locations: Vec::new(),
            selected_location: None,
            unique_leagues: Vec::new(),
            selected_league: None,
            show_advanced_filters: false,
            min_wealth: None,
            max_wealth: None,
            min_year: None,
            max_year: None,
            wealth_filter_text: String::new(),
            year_filter_text: String::new(),
        }
    }

    pub fn set_teams(&mut self, teams: Vec<Team>) {
        self.teams = teams;
        self.update_filter_options();
        self.apply_filter();
    }

    pub fn update_filter_options(&mut self) {
        // 提取唯一地区
        let mut locations = self.teams.iter()
            .map(|t| t.location.clone())
            .collect::<Vec<_>>();
        locations.sort();
        locations.dedup();
        self.unique_locations = locations;

        // 提取唯一联赛ID
        let mut leagues = self.teams.iter()
            .map(|t| t.league_id)
            .collect::<Vec<_>>();
        leagues.sort();
        leagues.dedup();
        self.unique_leagues = leagues;
    }

    pub fn apply_filter(&mut self) {
        // 开始过滤
        self.filtered_teams = self.teams.clone();

        // 应用搜索文本过滤
        if !self.search_text.is_empty() {
            let search_term = self.search_text.to_lowercase();
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| {
                    match self.filter_field {
                        FilterField::Name => team.name.to_lowercase().contains(&search_term),
                        FilterField::Location => team.location.to_lowercase().contains(&search_term),
                        FilterField::League => team.league_id.to_string().contains(&search_term),
                        FilterField::All => team.search_string().to_lowercase().contains(&search_term),
                    }
                })
                .cloned()
                .collect();
        }

        // 应用地区过滤
        if let Some(location) = &self.selected_location {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| &team.location == location)
                .cloned()
                .collect();
        }

        // 应用联赛过滤
        if let Some(league_id) = self.selected_league {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| team.league_id == league_id)
                .cloned()
                .collect();
        }

        // 应用财富范围过滤
        if let Some(min) = self.min_wealth {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| team.wealth >= min)
                .cloned()
                .collect();
        }

        if let Some(max) = self.max_wealth {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| team.wealth <= max)
                .cloned()
                .collect();
        }

        // 应用成立年份范围过滤
        if let Some(min) = self.min_year {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| team.found_year >= min)
                .cloned()
                .collect();
        }

        if let Some(max) = self.max_year {
            self.filtered_teams = self.filtered_teams
                .iter()
                .filter(|team| team.found_year <= max)
                .cloned()
                .collect();
        }
    }

    pub fn parse_wealth_filter(&mut self) {
        let text = self.wealth_filter_text.trim();
        if text.is_empty() {
            self.min_wealth = None;
            self.max_wealth = None;
            return;
        }

        if text.contains('-') {
            let parts: Vec<&str> = text.split('-').collect();
            if parts.len() == 2 {
                self.min_wealth = parts[0].trim().parse().ok();
                self.max_wealth = parts[1].trim().parse().ok();
            }
        } else if let Ok(value) = text.parse::<i64>() {
            self.min_wealth = Some(value);
            self.max_wealth = None;
        }
    }

    pub fn parse_year_filter(&mut self) {
        let text = self.year_filter_text.trim();
        if text.is_empty() {
            self.min_year = None;
            self.max_year = None;
            return;
        }

        if text.contains('-') {
            let parts: Vec<&str> = text.split('-').collect();
            if parts.len() == 2 {
                self.min_year = parts[0].trim().parse().ok();
                self.max_year = parts[1].trim().parse().ok();
            }
        } else if let Ok(value) = text.parse::<i64>() {
            self.min_year = Some(value);
            self.max_year = None;
        }
    }

    pub fn get_selected_team(&self) -> Option<&Team> {
        self.selected_index.and_then(|idx| self.filtered_teams.get(idx))
    }

    pub fn get_selected_team_id(&self) -> Option<i64> {
        self.get_selected_team().map(|team| team.id)
    }

    pub fn select_team_by_id(&mut self, team_id: i64) {
        self.selected_index = self.filtered_teams
            .iter()
            .position(|team| team.id == team_id);
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<i64> {
        let mut selected_team_id = None;

        widgets::titled_frame("球队列表", ui, |ui| {
            // 基本搜索框
            ui.horizontal(|ui| {
                ui.strong("搜索:");
                
                let search_response = ui.add(egui::TextEdit::singleline(&mut self.search_text)
                    .hint_text("输入搜索关键词...")
                    .desired_width(150.0));
                
                ComboBox::from_id_source("filter_field")
                    .selected_text(self.filter_field.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.filter_field, FilterField::All, FilterField::All.as_str());
                        ui.selectable_value(&mut self.filter_field, FilterField::Name, FilterField::Name.as_str());
                        ui.selectable_value(&mut self.filter_field, FilterField::Location, FilterField::Location.as_str());
                        ui.selectable_value(&mut self.filter_field, FilterField::League, FilterField::League.as_str());
                    });
                
                if search_response.changed() {
                    self.apply_filter();
                }
                
                if widgets::mac_button(ui, "清除") {
                    self.search_text.clear();
                    self.apply_filter();
                }
            });

            // 高级过滤器切换
            ui.horizontal(|ui| {
                if widgets::mac_button(ui, if self.show_advanced_filters { "隐藏高级过滤" } else { "显示高级过滤" }) {
                    self.show_advanced_filters = !self.show_advanced_filters;
                }
                
                ui.label(format!("共计: {} 个球队", self.filtered_teams.len()));
            });

            // 高级过滤器
            if self.show_advanced_filters {
                ui.add_space(5.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(245, 245, 250))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 230)))
                    .rounding(Rounding::same(4.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("地区:");
                            ComboBox::from_id_source("location_filter")
                                .selected_text(self.selected_location.as_deref().unwrap_or("全部地区"))
                                .show_ui(ui, |ui| {
                                    if ui.selectable_label(self.selected_location.is_none(), "全部地区").clicked() {
                                        self.selected_location = None;
                                        self.apply_filter();
                                    }
                                    
                                    for location in &self.unique_locations.clone() {
                                        if ui.selectable_label(
                                            self.selected_location.as_deref() == Some(location), 
                                            location
                                        ).clicked() {
                                            self.selected_location = Some(location.clone());
                                            self.apply_filter();
                                        }
                                    }
                                });
                            
                            ui.add_space(10.0);
                            
                            ui.label("联赛:");
                            ComboBox::from_id_source("league_filter")
                                .selected_text(self.selected_league.map_or("全部联赛".to_string(), |id| format!("联赛 {}", id)))
                                .show_ui(ui, |ui| {
                                    if ui.selectable_label(self.selected_league.is_none(), "全部联赛").clicked() {
                                        self.selected_league = None;
                                        self.apply_filter();
                                    }
                                    
                                    for &league_id in &self.unique_leagues.clone() {
                                        if ui.selectable_label(
                                            self.selected_league == Some(league_id), 
                                            format!("联赛 {}", league_id)
                                        ).clicked() {
                                            self.selected_league = Some(league_id);
                                            self.apply_filter();
                                        }
                                    }
                                });
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("财富范围:");
                            let wealth_response = ui.add(egui::TextEdit::singleline(&mut self.wealth_filter_text)
                                .hint_text("例如: 1000-5000")
                                .desired_width(120.0));
                            
                            ui.label("成立年份:");
                            let year_response = ui.add(egui::TextEdit::singleline(&mut self.year_filter_text)
                                .hint_text("例如: 1900-2000")
                                .desired_width(120.0));
                            
                            if wealth_response.changed() {
                                self.parse_wealth_filter();
                                self.apply_filter();
                            }
                            
                            if year_response.changed() {
                                self.parse_year_filter();
                                self.apply_filter();
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            if widgets::mac_button(ui, "重置所有过滤") {
                                self.search_text.clear();
                                self.selected_location = None;
                                self.selected_league = None;
                                self.wealth_filter_text.clear();
                                self.year_filter_text.clear();
                                self.min_wealth = None;
                                self.max_wealth = None;
                                self.min_year = None;
                                self.max_year = None;
                                self.apply_filter();
                            }
                        });
                    });
            }

            ui.add_space(5.0);

            // 球队列表
            egui::Frame::none()
                .fill(Color32::from_rgb(255, 255, 255))
                .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
                .rounding(Rounding::same(6.0))
                .inner_margin(egui::Margin::same(8.0))
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            for (idx, team) in self.filtered_teams.iter().enumerate() {
                                let is_selected = Some(idx) == self.selected_index;
                                
                                let mut text = RichText::new(&team.name);
                                
                                if is_selected {
                                    text = text.strong().color(Color32::from_rgb(50, 100, 200));
                                }
                                
                                let response = ui.selectable_value(
                                    &mut self.selected_index, 
                                    Some(idx), 
                                    text
                                );
                                
                                if response.clicked() {
                                    selected_team_id = Some(team.id);
                                    info!("选择球队: {} (ID: {})", team.name, team.id);
                                }
                                
                                response.on_hover_ui(|ui| {
                                    widgets::mac_card(ui, |ui| {
                                        widgets::label_value(ui, "ID:", &team.id.to_string());
                                        widgets::label_value(ui, "地区:", &team.location);
                                        widgets::label_value(ui, "联赛ID:", &team.league_id.to_string());
                                        widgets::label_value(ui, "昵称:", &team.nickname);
                                        widgets::label_value(ui, "成立年份:", &team.found_year.to_string());
                                        widgets::label_value(ui, "财富:", &format!("{} 万", team.wealth));
                                        widgets::label_value(ui, "主场:", &team.stadium_name);
                                        widgets::label_value(ui, "球迷数:", &format!("{} 人", team.supporter_count));
                                    });
                                });
                            }
                        });
                });
        });

        selected_team_id
    }
} 