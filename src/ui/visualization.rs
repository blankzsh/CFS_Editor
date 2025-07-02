use std::collections::HashMap;
use egui::{Color32, Ui, ScrollArea, ComboBox, Grid};
use crate::data::team::Team;
use crate::ui::widgets;

pub struct VisualizationView {
    pub teams: Vec<Team>,
    pub chart_type: ChartType,
    pub wealth_ranges: Vec<(i64, i64)>,
    pub supporter_ranges: Vec<(i64, i64)>,
    pub location_counts: HashMap<String, i64>,
    pub league_counts: HashMap<i64, i64>,
    pub selected_location: Option<String>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChartType {
    WealthDistribution,
    SupporterDistribution,
    LocationDistribution,
    LeagueDistribution,
}

impl ChartType {
    fn as_str(&self) -> &'static str {
        match self {
            ChartType::WealthDistribution => "球队财富分布",
            ChartType::SupporterDistribution => "球迷数量分布",
            ChartType::LocationDistribution => "地区分布",
            ChartType::LeagueDistribution => "联赛分布",
        }
    }
}

impl VisualizationView {
    pub fn new() -> Self {
        VisualizationView {
            teams: Vec::new(),
            chart_type: ChartType::WealthDistribution,
            wealth_ranges: vec![
                (0, 1000),
                (1001, 5000),
                (5001, 10000),
                (10001, 50000),
                (50001, i64::MAX),
            ],
            supporter_ranges: vec![
                (0, 10000),
                (10001, 50000),
                (50001, 100000),
                (100001, 500000),
                (500001, i64::MAX),
            ],
            location_counts: HashMap::new(),
            league_counts: HashMap::new(),
            selected_location: None,
        }
    }

    pub fn set_teams(&mut self, teams: Vec<Team>) {
        self.teams = teams;
        self.update_statistics();
    }

    pub fn update_statistics(&mut self) {
        // 更新地区统计
        self.location_counts.clear();
        for team in &self.teams {
            *self.location_counts.entry(team.location.clone()).or_insert(0) += 1;
        }

        // 更新联赛统计
        self.league_counts.clear();
        for team in &self.teams {
            *self.league_counts.entry(team.league_id).or_insert(0) += 1;
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        widgets::titled_frame("数据可视化", ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("选择图表类型:");
                ComboBox::from_id_source("chart_type")
                    .selected_text(self.chart_type.as_str())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.chart_type, ChartType::WealthDistribution, ChartType::WealthDistribution.as_str());
                        ui.selectable_value(&mut self.chart_type, ChartType::SupporterDistribution, ChartType::SupporterDistribution.as_str());
                        ui.selectable_value(&mut self.chart_type, ChartType::LocationDistribution, ChartType::LocationDistribution.as_str());
                        ui.selectable_value(&mut self.chart_type, ChartType::LeagueDistribution, ChartType::LeagueDistribution.as_str());
                    });
            });

            ui.add_space(10.0);

            match self.chart_type {
                ChartType::WealthDistribution => self.show_wealth_distribution(ui),
                ChartType::SupporterDistribution => self.show_supporter_distribution(ui),
                ChartType::LocationDistribution => self.show_location_distribution(ui),
                ChartType::LeagueDistribution => self.show_league_distribution(ui),
            }
        });
    }

    fn show_wealth_distribution(&self, ui: &mut Ui) {
        let mut values = vec![0; self.wealth_ranges.len()];
        let mut labels = Vec::new();

        // 计算每个财富范围的球队数量
        for team in &self.teams {
            for (i, (min, max)) in self.wealth_ranges.iter().enumerate() {
                if team.wealth >= *min && team.wealth <= *max {
                    values[i] += 1;
                    break;
                }
            }
        }

        // 创建标签
        for (min, max) in &self.wealth_ranges {
            if *max == i64::MAX {
                labels.push(format!("{}+", min));
            } else {
                labels.push(format!("{}-{}", min, max));
            }
        }

        // 绘制条形图
        widgets::draw_bar_chart(ui, &values, &labels, "球队财富分布", 300.0);

        // 显示详细数据表格
        ui.add_space(20.0);
        ui.heading("财富分布详情");
        ui.add_space(5.0);

        Grid::new("wealth_distribution_grid")
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                ui.strong("财富范围");
                ui.strong("球队数量");
                ui.strong("占比");
                ui.end_row();

                let total_teams = self.teams.len() as f32;
                for (i, (min, max)) in self.wealth_ranges.iter().enumerate() {
                    let range_text = if *max == i64::MAX {
                        format!("{}+", min)
                    } else {
                        format!("{}-{}", min, max)
                    };
                    
                    ui.label(range_text);
                    ui.label(values[i].to_string());
                    
                    let percentage = if total_teams > 0.0 {
                        (values[i] as f32 / total_teams) * 100.0
                    } else {
                        0.0
                    };
                    ui.label(format!("{:.1}%", percentage));
                    
                    ui.end_row();
                }
            });
    }

    fn show_supporter_distribution(&self, ui: &mut Ui) {
        let mut values = vec![0; self.supporter_ranges.len()];
        let mut labels = Vec::new();

        // 计算每个球迷数量范围的球队数量
        for team in &self.teams {
            for (i, (min, max)) in self.supporter_ranges.iter().enumerate() {
                if team.supporter_count >= *min && team.supporter_count <= *max {
                    values[i] += 1;
                    break;
                }
            }
        }

        // 创建标签
        for (min, max) in &self.supporter_ranges {
            if *max == i64::MAX {
                labels.push(format!("{}+", min));
            } else {
                labels.push(format!("{}-{}", min, max));
            }
        }

        // 绘制条形图
        widgets::draw_bar_chart(ui, &values, &labels, "球迷数量分布", 300.0);

        // 显示详细数据表格
        ui.add_space(20.0);
        ui.heading("球迷数量分布详情");
        ui.add_space(5.0);

        Grid::new("supporter_distribution_grid")
            .num_columns(3)
            .striped(true)
            .show(ui, |ui| {
                ui.strong("球迷数量范围");
                ui.strong("球队数量");
                ui.strong("占比");
                ui.end_row();

                let total_teams = self.teams.len() as f32;
                for (i, (min, max)) in self.supporter_ranges.iter().enumerate() {
                    let range_text = if *max == i64::MAX {
                        format!("{}+", min)
                    } else {
                        format!("{}-{}", min, max)
                    };
                    
                    ui.label(range_text);
                    ui.label(values[i].to_string());
                    
                    let percentage = if total_teams > 0.0 {
                        (values[i] as f32 / total_teams) * 100.0
                    } else {
                        0.0
                    };
                    ui.label(format!("{:.1}%", percentage));
                    
                    ui.end_row();
                }
            });
    }

    fn show_location_distribution(&self, ui: &mut Ui) {
        if self.location_counts.is_empty() {
            ui.label("没有地区数据可供显示");
            return;
        }

        // 准备数据
        let mut locations: Vec<(String, i64)> = self.location_counts.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        
        // 按数量排序
        locations.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 如果超过10个地区，只显示前10个
        let (top_locations, other_count) = if locations.len() > 10 {
            let top = locations[0..10].to_vec();
            let other: i64 = locations[10..].iter().map(|(_, count)| count).sum();
            (top, Some(other))
        } else {
            (locations.clone(), None)
        };
        
        // 准备图表数据
        let mut values = top_locations.iter().map(|(_, count)| *count).collect::<Vec<_>>();
        let mut labels = top_locations.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
        
        // 添加"其他"类别
        if let Some(count) = other_count {
            values.push(count);
            labels.push("其他".to_string());
        }
        
        // 绘制饼图
        widgets::draw_pie_chart(ui, &values, &labels, "地区分布", 400.0);
        
        // 显示详细数据表格
        ui.add_space(20.0);
        ui.heading("地区分布详情");
        ui.add_space(5.0);
        
        // 创建可滚动区域
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            Grid::new("location_distribution_grid")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("地区");
                    ui.strong("球队数量");
                    ui.strong("占比");
                    ui.end_row();
                    
                    let total_teams = self.teams.len() as f32;
                    for (location, count) in locations.iter() {
                        ui.label(location);
                        ui.label(count.to_string());
                        
                        let percentage = if total_teams > 0.0 {
                            (*count as f32 / total_teams) * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!("{:.1}%", percentage));
                        
                        ui.end_row();
                    }
                });
        });
    }

    fn show_league_distribution(&self, ui: &mut Ui) {
        if self.league_counts.is_empty() {
            ui.label("没有联赛数据可供显示");
            return;
        }

        // 准备数据
        let mut leagues: Vec<(i64, i64)> = self.league_counts.iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        
        // 按数量排序
        leagues.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 如果超过10个联赛，只显示前10个
        let (top_leagues, other_count) = if leagues.len() > 10 {
            let top = leagues[0..10].to_vec();
            let other: i64 = leagues[10..].iter().map(|(_, count)| count).sum();
            (top, Some(other))
        } else {
            (leagues.clone(), None)
        };
        
        // 准备图表数据
        let mut values = top_leagues.iter().map(|(_, count)| *count).collect::<Vec<_>>();
        let mut labels = top_leagues.iter().map(|(id, _)| format!("联赛 {}", id)).collect::<Vec<_>>();
        
        // 添加"其他"类别
        if let Some(count) = other_count {
            values.push(count);
            labels.push("其他".to_string());
        }
        
        // 绘制饼图
        widgets::draw_pie_chart(ui, &values, &labels, "联赛分布", 400.0);
        
        // 显示详细数据表格
        ui.add_space(20.0);
        ui.heading("联赛分布详情");
        ui.add_space(5.0);
        
        // 创建可滚动区域
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            Grid::new("league_distribution_grid")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("联赛ID");
                    ui.strong("球队数量");
                    ui.strong("占比");
                    ui.end_row();
                    
                    let total_teams = self.teams.len() as f32;
                    for (league_id, count) in leagues.iter() {
                        ui.label(format!("联赛 {}", league_id));
                        ui.label(count.to_string());
                        
                        let percentage = if total_teams > 0.0 {
                            (*count as f32 / total_teams) * 100.0
                        } else {
                            0.0
                        };
                        ui.label(format!("{:.1}%", percentage));
                        
                        ui.end_row();
                    }
                });
        });
    }
} 