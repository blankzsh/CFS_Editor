use std::collections::HashMap;
use egui::{Color32, Ui, ScrollArea, ComboBox, Grid, RichText, Stroke, Rounding, pos2, Rect, Align2, Vec2};
use crate::data::team::Team;
use crate::ui::widgets;

pub struct VisualizationView {
    pub teams: Vec<Team>,
    pub chart_type: ChartType,
    pub chart_style: ChartStyle,
    pub wealth_ranges: Vec<(i64, i64)>,
    pub supporter_ranges: Vec<(i64, i64)>,
    pub location_counts: HashMap<String, i64>,
    pub league_counts: HashMap<i64, i64>,
    pub selected_location: Option<String>,
    pub show_data_table: bool,
    pub show_percentage: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChartType {
    WealthDistribution,
    SupporterDistribution,
    LocationDistribution,
    LeagueDistribution,
    FoundYearDistribution,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ChartStyle {
    BarChart,
    PieChart,
}

impl ChartType {
    fn as_str(&self) -> &'static str {
        match self {
            ChartType::WealthDistribution => "球队财富分布",
            ChartType::SupporterDistribution => "球迷数量分布",
            ChartType::LocationDistribution => "地区分布",
            ChartType::LeagueDistribution => "联赛分布",
            ChartType::FoundYearDistribution => "成立年份分布",
        }
    }
}

impl ChartStyle {
    fn as_str(&self) -> &'static str {
        match self {
            ChartStyle::BarChart => "条形图",
            ChartStyle::PieChart => "饼图",
        }
    }
}

impl VisualizationView {
    pub fn new() -> Self {
        VisualizationView {
            teams: Vec::new(),
            chart_type: ChartType::WealthDistribution,
            chart_style: ChartStyle::BarChart,
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
            show_data_table: true,
            show_percentage: true,
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
            // 图表控制面板
            egui::Frame::none()
                .fill(Color32::from_rgb(245, 245, 250))
                .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 230)))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.strong(RichText::new("图表类型:").color(Color32::from_rgb(40, 40, 80)).size(14.0));
                                ui.add_space(5.0);
                                
                                // 自定义下拉框样式
                                let dropdown_text = RichText::new(self.chart_type.as_str())
                                    .strong()
                                    .color(Color32::from_rgb(20, 20, 60))
                                    .size(14.0);
                                
                                ComboBox::from_id_source("chart_type")
                                    .selected_text(dropdown_text)
                                    .width(150.0)
                                    .show_ui(ui, |ui| {
                                        ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 20, 60));
                                        ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 40, 100));
                                        
                                        let chart_types = [
                                            ChartType::WealthDistribution,
                                            ChartType::SupporterDistribution,
                                            ChartType::LocationDistribution,
                                            ChartType::LeagueDistribution,
                                            ChartType::FoundYearDistribution,
                                        ];
                                        
                                        for chart_type in chart_types.iter() {
                                            let text = RichText::new(chart_type.as_str())
                                                .color(Color32::from_rgb(20, 20, 60))
                                                .size(14.0);
                                                
                                            if ui.selectable_label(self.chart_type == *chart_type, text).clicked() {
                                                self.chart_type = *chart_type;
                                            }
                                        }
                                    });
                            });
                            
                            ui.add_space(5.0);
                            
                            ui.horizontal(|ui| {
                                ui.strong(RichText::new("图表样式:").color(Color32::from_rgb(40, 40, 80)).size(14.0));
                                ui.add_space(5.0);
                                
                                // 自定义下拉框样式
                                let dropdown_text = RichText::new(self.chart_style.as_str())
                                    .strong()
                                    .color(Color32::from_rgb(20, 20, 60))
                                    .size(14.0);
                                
                                ComboBox::from_id_source("chart_style")
                                    .selected_text(dropdown_text)
                                    .width(150.0)
                                    .show_ui(ui, |ui| {
                                        ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 20, 60));
                                        ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 40, 100));
                                        
                                        let chart_styles = [
                                            ChartStyle::BarChart,
                                            ChartStyle::PieChart,
                                        ];
                                        
                                        for chart_style in chart_styles.iter() {
                                            let text = RichText::new(chart_style.as_str())
                                                .color(Color32::from_rgb(20, 20, 60))
                                                .size(14.0);
                                                
                                            if ui.selectable_label(self.chart_style == *chart_style, text).clicked() {
                                                self.chart_style = *chart_style;
                                            }
                                        }
                                    });
                            });
                        });
                        
                        ui.add_space(30.0);
                        
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                // 自定义复选框样式
                                ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 20, 60));
                                ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 40, 100));
                                
                                let checkbox_text = RichText::new("显示数据表格")
                                    .strong()
                                    .color(Color32::from_rgb(40, 40, 80))
                                    .size(14.0);
                                    
                                ui.checkbox(&mut self.show_data_table, checkbox_text);
                            });
                            
                            ui.add_space(5.0);
                            
                            ui.horizontal(|ui| {
                                // 自定义复选框样式
                                ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.5, Color32::from_rgb(20, 20, 60));
                                ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(40, 40, 100));
                                
                                let checkbox_text = RichText::new("显示百分比")
                                    .strong()
                                    .color(Color32::from_rgb(40, 40, 80))
                                    .size(14.0);
                                    
                                ui.checkbox(&mut self.show_percentage, checkbox_text);
                            });
                        });
                    });
                });

            ui.add_space(15.0);

            // 图表内容
            egui::Frame::none()
                .fill(Color32::from_rgb(255, 255, 255))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 230)))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .shadow(egui::epaint::Shadow {
                    extrusion: 2.0,
                    color: Color32::from_black_alpha(20),
                })
                .show(ui, |ui| {
                    match self.chart_type {
                        ChartType::WealthDistribution => self.show_wealth_distribution(ui),
                        ChartType::SupporterDistribution => self.show_supporter_distribution(ui),
                        ChartType::LocationDistribution => self.show_location_distribution(ui),
                        ChartType::LeagueDistribution => self.show_league_distribution(ui),
                        ChartType::FoundYearDistribution => self.show_found_year_distribution(ui),
                    }
                });
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

        // 绘制图表
        ui.heading(RichText::new("球队财富分布").size(20.0).strong().color(Color32::from_rgb(60, 60, 80)));
        ui.add_space(15.0);
        
        // 创建滚动区域以确保图表完整显示
        ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                match self.chart_style {
                    ChartStyle::BarChart => {
                        // 为条形图分配足够的高度
                        widgets::draw_bar_chart(ui, &values, &labels, "", 400.0);
                    },
                    ChartStyle::PieChart => {
                        // 为饼图分配足够的高度
                        self.draw_pie_chart(ui, &values, &labels, "", 500.0);
                    }
                }
            });

        // 显示详细数据表格
        if self.show_data_table {
            ui.add_space(20.0);
            ui.heading(RichText::new("财富分布详情").size(16.0).strong());
            ui.add_space(10.0);

            // 添加表格边框
            egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 252))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 240)))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    Grid::new("wealth_distribution_grid")
                        .num_columns(if self.show_percentage { 3 } else { 2 })
                        .striped(true)
                        .spacing([10.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong(RichText::new("财富范围").color(Color32::from_rgb(60, 60, 100)));
                            ui.strong(RichText::new("球队数量").color(Color32::from_rgb(60, 60, 100)));
                            if self.show_percentage {
                                ui.strong(RichText::new("占比").color(Color32::from_rgb(60, 60, 100)));
                            }
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
                                
                                if self.show_percentage {
                                    let percentage = if total_teams > 0.0 {
                                        (values[i] as f32 / total_teams) * 100.0
                                    } else {
                                        0.0
                                    };
                                    ui.label(format!("{:.1}%", percentage));
                                }
                                
                                ui.end_row();
                            }
                        });
                });
        }
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

        // 绘制图表
        ui.heading(RichText::new("球迷数量分布").size(20.0).strong().color(Color32::from_rgb(60, 60, 80)));
        ui.add_space(15.0);
        
        // 创建滚动区域以确保图表完整显示
        ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                match self.chart_style {
                    ChartStyle::BarChart => {
                        widgets::draw_bar_chart(ui, &values, &labels, "", 400.0);
                    },
                    ChartStyle::PieChart => {
                        self.draw_pie_chart(ui, &values, &labels, "", 500.0);
                    }
                }
            });

        // 显示详细数据表格
        if self.show_data_table {
            ui.add_space(20.0);
            ui.heading(RichText::new("球迷数量分布详情").size(16.0).strong());
            ui.add_space(10.0);

            // 添加表格边框
            egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 252))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 240)))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    Grid::new("supporter_distribution_grid")
                        .num_columns(if self.show_percentage { 3 } else { 2 })
                        .striped(true)
                        .spacing([10.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong(RichText::new("球迷数量范围").color(Color32::from_rgb(60, 60, 100)));
                            ui.strong(RichText::new("球队数量").color(Color32::from_rgb(60, 60, 100)));
                            if self.show_percentage {
                                ui.strong(RichText::new("占比").color(Color32::from_rgb(60, 60, 100)));
                            }
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
                                
                                if self.show_percentage {
                                    let percentage = if total_teams > 0.0 {
                                        (values[i] as f32 / total_teams) * 100.0
                                    } else {
                                        0.0
                                    };
                                    ui.label(format!("{:.1}%", percentage));
                                }
                                
                                ui.end_row();
                            }
                        });
                });
        }
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
        
        // 绘制图表
        ui.heading(RichText::new("地区分布").size(20.0).strong().color(Color32::from_rgb(60, 60, 80)));
        ui.add_space(15.0);
        
        // 创建滚动区域以确保图表完整显示
        ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                match self.chart_style {
                    ChartStyle::BarChart => {
                        widgets::draw_bar_chart(ui, &values, &labels, "", 400.0);
                    },
                    ChartStyle::PieChart => {
                        self.draw_pie_chart(ui, &values, &labels, "", 500.0);
                    }
                }
            });
        
        // 显示详细数据表格
        if self.show_data_table {
            ui.add_space(20.0);
            ui.heading(RichText::new("地区分布详情").size(16.0).strong());
            ui.add_space(10.0);
            
            // 添加表格边框
            egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 252))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 240)))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    // 创建可滚动区域
                    ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        Grid::new("location_distribution_grid")
                            .num_columns(if self.show_percentage { 3 } else { 2 })
                            .striped(true)
                            .spacing([10.0, 6.0])
                            .show(ui, |ui| {
                                ui.strong(RichText::new("地区").color(Color32::from_rgb(60, 60, 100)));
                                ui.strong(RichText::new("球队数量").color(Color32::from_rgb(60, 60, 100)));
                                if self.show_percentage {
                                    ui.strong(RichText::new("占比").color(Color32::from_rgb(60, 60, 100)));
                                }
                                ui.end_row();
                                
                                let total_teams = self.teams.len() as f32;
                                for (location, count) in locations.iter() {
                                    ui.label(location);
                                    ui.label(count.to_string());
                                    
                                    if self.show_percentage {
                                        let percentage = if total_teams > 0.0 {
                                            (*count as f32 / total_teams) * 100.0
                                        } else {
                                            0.0
                                        };
                                        ui.label(format!("{:.1}%", percentage));
                                    }
                                    
                                    ui.end_row();
                                }
                            });
                    });
                });
        }
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
        
        // 绘制图表
        ui.heading(RichText::new("联赛分布").size(20.0).strong().color(Color32::from_rgb(60, 60, 80)));
        ui.add_space(15.0);
        
        // 创建滚动区域以确保图表完整显示
        ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                match self.chart_style {
                    ChartStyle::BarChart => {
                        widgets::draw_bar_chart(ui, &values, &labels, "", 400.0);
                    },
                    ChartStyle::PieChart => {
                        self.draw_pie_chart(ui, &values, &labels, "", 500.0);
                    }
                }
            });
        
        // 显示详细数据表格
        if self.show_data_table {
            ui.add_space(20.0);
            ui.heading(RichText::new("联赛分布详情").size(16.0).strong());
            ui.add_space(10.0);
            
            // 添加表格边框
            egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 252))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 240)))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    // 创建可滚动区域
                    ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        Grid::new("league_distribution_grid")
                            .num_columns(if self.show_percentage { 3 } else { 2 })
                            .striped(true)
                            .spacing([10.0, 6.0])
                            .show(ui, |ui| {
                                ui.strong(RichText::new("联赛ID").color(Color32::from_rgb(60, 60, 100)));
                                ui.strong(RichText::new("球队数量").color(Color32::from_rgb(60, 60, 100)));
                                if self.show_percentage {
                                    ui.strong(RichText::new("占比").color(Color32::from_rgb(60, 60, 100)));
                                }
                                ui.end_row();
                                
                                let total_teams = self.teams.len() as f32;
                                for (league_id, count) in leagues.iter() {
                                    ui.label(format!("联赛 {}", league_id));
                                    ui.label(count.to_string());
                                    
                                    if self.show_percentage {
                                        let percentage = if total_teams > 0.0 {
                                            (*count as f32 / total_teams) * 100.0
                                        } else {
                                            0.0
                                        };
                                        ui.label(format!("{:.1}%", percentage));
                                    }
                                    
                                    ui.end_row();
                                }
                            });
                    });
                });
        }
    }
    
    fn show_found_year_distribution(&self, ui: &mut Ui) {
        if self.teams.is_empty() {
            ui.label("没有成立年份数据可供显示");
            return;
        }
        
        // 计算成立年份的范围
        let min_year = self.teams.iter().map(|t| t.found_year).min().unwrap_or(1800);
        let max_year = self.teams.iter().map(|t| t.found_year).max().unwrap_or(2023);
        
        // 创建年代范围（每20年一个区间）
        let period = 20;
        let mut year_ranges = Vec::new();
        let mut current = min_year - (min_year % period);
        while current <= max_year {
            year_ranges.push((current, current + period - 1));
            current += period;
        }
        
        // 计算每个年代范围的球队数量
        let mut values = vec![0; year_ranges.len()];
        for team in &self.teams {
            for (i, (min, max)) in year_ranges.iter().enumerate() {
                if team.found_year >= *min && team.found_year <= *max {
                    values[i] += 1;
                    break;
                }
            }
        }
        
        // 创建标签
        let labels = year_ranges.iter()
            .map(|(min, max)| format!("{}-{}", min, max))
            .collect::<Vec<_>>();
        
        // 绘制图表
        ui.heading(RichText::new("成立年份分布").size(20.0).strong().color(Color32::from_rgb(60, 60, 80)));
        ui.add_space(15.0);
        
        // 创建滚动区域以确保图表完整显示
        ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                match self.chart_style {
                    ChartStyle::BarChart => {
                        widgets::draw_bar_chart(ui, &values, &labels, "", 400.0);
                    },
                    ChartStyle::PieChart => {
                        self.draw_pie_chart(ui, &values, &labels, "", 500.0);
                    }
                }
            });
        
        // 显示详细数据表格
        if self.show_data_table {
            ui.add_space(20.0);
            ui.heading(RichText::new("成立年份分布详情").size(16.0).strong());
            ui.add_space(10.0);
            
            // 添加表格边框
            egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 252))
                .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 240)))
                .rounding(Rounding::same(4.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    Grid::new("found_year_distribution_grid")
                        .num_columns(if self.show_percentage { 3 } else { 2 })
                        .striped(true)
                        .spacing([10.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong(RichText::new("年份范围").color(Color32::from_rgb(60, 60, 100)));
                            ui.strong(RichText::new("球队数量").color(Color32::from_rgb(60, 60, 100)));
                            if self.show_percentage {
                                ui.strong(RichText::new("占比").color(Color32::from_rgb(60, 60, 100)));
                            }
                            ui.end_row();
                            
                            let total_teams = self.teams.len() as f32;
                            for (i, (min, max)) in year_ranges.iter().enumerate() {
                                ui.label(format!("{}-{}", min, max));
                                ui.label(values[i].to_string());
                                
                                if self.show_percentage {
                                    let percentage = if total_teams > 0.0 {
                                        (values[i] as f32 / total_teams) * 100.0
                                    } else {
                                        0.0
                                    };
                                    ui.label(format!("{:.1}%", percentage));
                                }
                                
                                ui.end_row();
                            }
                        });
                });
        }
    }

    fn draw_pie_chart(&self, ui: &mut Ui, values: &[i64], labels: &[String], title: &str, size: f32) {
        if values.is_empty() || labels.is_empty() {
            return;
        }
        
        ui.heading(title);
        ui.add_space(5.0);
        
        // 将数据转换为我们需要的格式
        let data: Vec<(String, i64)> = values.iter()
            .zip(labels.iter())
            .map(|(&value, label)| (label.clone(), value))
            .collect();
        
        // 计算总和
        let total: i64 = values.iter().sum();
        if total <= 0 {
            ui.label("没有数据可显示");
            return;
        }
        
        // 定义饼图尺寸和位置
        let available_width = ui.available_width();
        let chart_height = 400.0;
        
        // 创建滚动区域以确保图表完整显示
        egui::ScrollArea::vertical().show(ui, |ui| {
            // 计算饼图的合适大小
            let pie_size = f32::min(available_width * 0.8, chart_height * 0.8);
            let center_x = available_width / 2.0;
            
            // 为饼图和图例分配空间
            let (response, painter) = ui.allocate_painter(
                Vec2::new(available_width, chart_height),
                egui::Sense::hover()
            );
            let rect = response.rect;
            
            // 计算饼图中心点
            let center = pos2(center_x, rect.min.y + pie_size / 2.0 + 20.0);
            let radius = pie_size / 2.0;
            
            // 定义扇形颜色
            let colors = [
                Color32::from_rgb(100, 150, 250), // 蓝色
                Color32::from_rgb(250, 150, 100), // 橙色
                Color32::from_rgb(100, 250, 150), // 绿色
                Color32::from_rgb(250, 100, 150), // 粉色
                Color32::from_rgb(150, 100, 250), // 紫色
                Color32::from_rgb(150, 250, 100), // 黄绿色
                Color32::from_rgb(100, 200, 250), // 浅蓝色
                Color32::from_rgb(250, 200, 100), // 浅橙色
                Color32::from_rgb(100, 250, 200), // 浅绿色
                Color32::from_rgb(250, 100, 200), // 浅粉色
                Color32::from_rgb(200, 100, 250), // 浅紫色
                Color32::from_rgb(200, 250, 100), // 浅黄绿色
            ];
            
            // 绘制饼图
            let mut start_angle = 0.0;
            
            // 计算最小角度，小于这个角度的扇形将被特殊处理
            let min_angle_threshold = 0.05; // 约3度
            let mut small_slices = Vec::new();
            
            for (i, (label, value)) in data.iter().enumerate() {
                let angle = 2.0 * std::f32::consts::PI * (*value as f32 / total as f32);
                let end_angle = start_angle + angle;
                let color = colors[i % colors.len()];
                
                // 绘制扇形
                painter.add(egui::Shape::Path(egui::epaint::PathShape {
                    points: {
                        let mut points = Vec::new();
                        points.push(center);
                        
                        // 添加弧线上的点
                        let n_points = (angle * 30.0).ceil() as usize;
                        let n_points = n_points.max(4); // 至少4个点
                        
                        for i in 0..=n_points {
                            let a = start_angle + angle * (i as f32 / n_points as f32);
                            let x = center.x + radius * a.cos();
                            let y = center.y + radius * a.sin();
                            points.push(pos2(x, y));
                        }
                        
                        points
                    },
                    closed: true,
                    fill: color,
                    stroke: Stroke::new(1.0, Color32::WHITE),
                }));
                
                // 如果扇形角度太小，将标签信息保存起来，稍后单独处理
                if angle < min_angle_threshold {
                    small_slices.push((label.clone(), *value, color, (start_angle + end_angle) / 2.0));
                } else {
                    // 计算标签位置（在扇形中心位置的70%半径处）
                    let label_angle = (start_angle + end_angle) / 2.0;
                    let label_distance = radius * 0.7;
                    let label_x = center.x + label_distance * label_angle.cos();
                    let label_y = center.y + label_distance * label_angle.sin();
                    
                    // 绘制连接线
                    let outer_x = center.x + radius * 1.05 * label_angle.cos();
                    let outer_y = center.y + radius * 1.05 * label_angle.sin();
                    
                    painter.line_segment(
                        [pos2(label_x, label_y), pos2(outer_x, outer_y)],
                        Stroke::new(1.0, Color32::DARK_GRAY)
                    );
                    
                    // 计算百分比
                    let percentage = (*value as f32 / total as f32) * 100.0;
                    
                    // 确定文本对齐方式
                    let align = if label_angle < std::f32::consts::PI {
                        Align2::LEFT_CENTER
                    } else {
                        Align2::RIGHT_CENTER
                    };
                    
                    // 计算文本位置
                    let text_x = if label_angle < std::f32::consts::PI {
                        outer_x + 5.0
                    } else {
                        outer_x - 5.0
                    };
                    
                    // 绘制标签和百分比
                    let text = format!("{}: {} ({}%)", label, value, percentage as i32);
                    
                    // 绘制文本背景以提高可读性
                    let font_id = egui::FontId::proportional(10.0);
                    let galley = painter.layout_no_wrap(
                        text.clone(),
                        font_id.clone(),
                        Color32::DARK_GRAY
                    );
                    
                    let text_rect = Rect::from_min_size(
                        pos2(
                            if align == Align2::LEFT_CENTER { text_x } else { text_x - galley.size().x },
                            outer_y - galley.size().y / 2.0
                        ),
                        galley.size()
                    ).expand(2.0);
                    
                    painter.rect_filled(
                        text_rect,
                        Rounding::same(2.0),
                        Color32::from_rgba_unmultiplied(255, 255, 255, 220)
                    );
                    
                    // 绘制文本
                    painter.text(
                        pos2(text_x, outer_y),
                        align,
                        text,
                        font_id,
                        Color32::DARK_GRAY
                    );
                }
                
                start_angle = end_angle;
            }
            
            // 处理小扇形的标签，将它们放在图表右侧
            if !small_slices.is_empty() {
                let legend_x = center.x + radius + 20.0;
                let mut legend_y = center.y - radius;
                
                // 添加小扇形图例标题
                painter.text(
                    pos2(legend_x, legend_y),
                    Align2::LEFT_TOP,
                    "小比例项目:",
                    egui::FontId::proportional(12.0),
                    Color32::DARK_GRAY
                );
                
                legend_y += 20.0;
                
                // 为每个小扇形添加图例项
                for (label, value, color, _) in small_slices {
                    // 绘制颜色方块
                    let square_size = 10.0;
                    painter.rect_filled(
                        Rect::from_min_size(
                            pos2(legend_x, legend_y),
                            Vec2::new(square_size, square_size)
                        ),
                        Rounding::same(2.0),
                        color
                    );
                    
                    // 计算百分比
                    let percentage = (value as f32 / total as f32) * 100.0;
                    
                    // 绘制标签和百分比
                    painter.text(
                        pos2(legend_x + square_size + 5.0, legend_y),
                        Align2::LEFT_TOP,
                        format!("{}: {} ({}%)", label, value, percentage as i32),
                        egui::FontId::proportional(10.0),
                        Color32::DARK_GRAY
                    );
                    
                    legend_y += 15.0;
                }
            }
            
            // 绘制中心圆圈和总数
            painter.circle_filled(
                center,
                radius * 0.3,
                Color32::from_rgba_unmultiplied(255, 255, 255, 200)
            );
            
            painter.circle_stroke(
                center,
                radius * 0.3,
                Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
            );
            
            // 绘制总数标签
            painter.text(
                center,
                Align2::CENTER_CENTER,
                format!("总计\n{}", total),
                egui::FontId::proportional(14.0),
                Color32::DARK_GRAY
            );
        });
    }
} 