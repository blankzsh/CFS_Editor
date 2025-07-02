use egui::{Color32, Context, Window, Rounding, Stroke, Vec2, Grid, ScrollArea};
use log::error;

use crate::data::staff::Staff;
use crate::data::team::Team;
use crate::error::{AppError, Result};
use crate::ui::widgets;

// Mac风格的窗口设置
fn setup_mac_window<'a>(title: &'a str) -> Window<'a> {
    Window::new(title)
        .frame(egui::Frame::none()
            .fill(Color32::from_rgb(245, 245, 245))
            .stroke(Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
            .rounding(Rounding::same(8.0))
            .shadow(egui::epaint::Shadow {
                extrusion: 5.0,
                color: Color32::from_black_alpha(40),
            }))
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
}

pub struct StaffEditDialog {
    pub open: bool,
    pub staff: Option<Staff>,
    pub name: String,
    pub ability: String,
    pub fame: String,
    pub confirmed: bool,
    pub error_message: Option<String>,
}

impl StaffEditDialog {
    pub fn new() -> Self {
        StaffEditDialog {
            open: false,
            staff: None,
            name: String::new(),
            ability: String::new(),
            fame: String::new(),
            confirmed: false,
            error_message: None,
        }
    }

    pub fn open(&mut self, staff: Staff) -> Result<()> {
        self.staff = Some(staff.clone());
        self.name = staff.name.clone();
        
        match staff.get_ability() {
            Ok(ability) => self.ability = ability.to_string(),
            Err(e) => {
                error!("获取员工能力值失败: {}", e);
                self.ability = "0".to_string();
            }
        }
        
        self.fame = staff.fame.to_string();
        self.confirmed = false;
        self.error_message = None;
        self.open = true;
        Ok(())
    }

    pub fn show(&mut self, ctx: &Context) -> bool {
        if !self.open {
            return false;
        }

        let mut closed = false;
        let mut confirmed = false;

        setup_mac_window("编辑员工")
            .fixed_size([400.0, 250.0])
            .show(ctx, |ui| {
                if let Some(staff) = &self.staff {
                    ui.add_space(5.0);
                    ui.heading(format!("编辑员工信息 (ID: {})", staff.id));
                    ui.add_space(10.0);
                    widgets::horizontal_separator(ui);
                    ui.add_space(10.0);

                    // 表单
                    widgets::form_row(ui, "姓名:", &mut self.name);
                    ui.add_space(5.0);
                    widgets::form_row(ui, "能力值:", &mut self.ability);
                    ui.add_space(5.0);
                    widgets::form_row(ui, "知名度:", &mut self.fame);

                    // 错误消息
                    if let Some(error) = &self.error_message {
                        ui.add_space(8.0);
                        widgets::error_message(ui, error);
                    }

                    ui.add_space(15.0);
                    widgets::horizontal_separator(ui);
                    ui.add_space(10.0);
                    
                    // 按钮
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::mac_primary_button(ui, "保存") {
                                // 验证
                                if self.name.trim().is_empty() {
                                    self.error_message = Some("姓名不能为空".to_string());
                                } else if let Err(e) = self.validate_numeric_inputs() {
                                    self.error_message = Some(e.to_string());
                                } else {
                                    confirmed = true;
                                    closed = true;
                                }
                            }
                            
                            ui.add_space(10.0);
                            
                            if widgets::mac_button(ui, "取消") {
                                closed = true;
                            }
                        });
                    });
                }
            });

        if closed && !confirmed {
            self.open = false;
        }

        self.confirmed = confirmed;
        confirmed
    }

    pub fn validate_numeric_inputs(&self) -> Result<()> {
        // 验证能力值
        match self.ability.trim().parse::<i64>() {
            Ok(n) if n >= 0 => {},
            Ok(_) => return Err(AppError::InvalidInput("能力值必须为正数".to_string())),
            Err(_) => return Err(AppError::InvalidInput("能力值必须为有效的整数".to_string())),
        }

        // 验证知名度
        match self.fame.trim().parse::<i64>() {
            Ok(n) if n >= 0 => {},
            Ok(_) => return Err(AppError::InvalidInput("知名度必须为正数".to_string())),
            Err(_) => return Err(AppError::InvalidInput("知名度必须为有效的整数".to_string())),
        }

        Ok(())
    }

    pub fn get_updated_staff(&self) -> Result<Staff> {
        let staff = self.staff.clone()
            .ok_or_else(|| AppError::Unknown("没有员工数据".to_string()))?;
        
        let ability = self.ability.trim().parse::<i64>()
            .map_err(|_| AppError::InvalidInput("无效的能力值".to_string()))?;
        
        let fame = self.fame.trim().parse::<i64>()
            .map_err(|_| AppError::InvalidInput("无效的知名度".to_string()))?;
        
        let mut updated = staff;
        updated.name = self.name.clone();
        updated.update_ability(ability)?;
        updated.fame = fame;
        
        Ok(updated)
    }
}

pub struct MessageDialog {
    pub title: String,
    pub message: String,
    pub open: bool,
}

impl MessageDialog {
    pub fn new() -> Self {
        MessageDialog {
            title: String::new(),
            message: String::new(),
            open: false,
        }
    }

    pub fn show_message(&mut self, title: &str, message: &str) {
        self.title = title.to_string();
        self.message = message.to_string();
        self.open = true;
    }

    pub fn show(&mut self, ctx: &Context) {
        if !self.open {
            return;
        }

        setup_mac_window(&self.title)
            .min_size(Vec2::new(300.0, 150.0))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(&self.message);
                ui.add_space(15.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::mac_primary_button(ui, "确定") {
                            self.open = false;
                        }
                    });
                });
            });
    }
}

pub struct ConfirmDialog {
    pub title: String,
    pub message: String,
    pub open: bool,
    pub confirmed: bool,
}

impl ConfirmDialog {
    pub fn new() -> Self {
        ConfirmDialog {
            title: String::new(),
            message: String::new(),
            open: false,
            confirmed: false,
        }
    }
    
    pub fn show_confirm(&mut self, title: &str, message: &str) {
        self.title = title.to_string();
        self.message = message.to_string();
        self.open = true;
        self.confirmed = false;
    }
    
    pub fn show(&mut self, ctx: &Context) -> bool {
        if !self.open {
            return false;
        }
        
        let mut closed = false;
        let mut confirmed = false;
        
        setup_mac_window(&self.title)
            .min_size(Vec2::new(350.0, 150.0))
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(&self.message);
                ui.add_space(15.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::mac_primary_button(ui, "确定") {
                            confirmed = true;
                            closed = true;
                        }
                        
                        ui.add_space(10.0);
                        
                        if widgets::mac_button(ui, "取消") {
                            closed = true;
                        }
                    });
                });
            });
        
        if closed {
            self.open = false;
            self.confirmed = confirmed;
        }
        
        confirmed
    }
}

pub struct BulkEditDialog {
    pub open: bool,
    pub teams: Vec<Team>,
    pub selected_teams: Vec<bool>,
    pub selected_count: usize,
    pub confirmed: bool,
    pub error_message: Option<String>,
    
    // 批量编辑字段
    pub edit_location: bool,
    pub location: String,
    pub edit_league: bool,
    pub league_id: String,
    pub edit_wealth_modifier: bool,
    pub wealth_modifier: String,
    pub wealth_modifier_type: WealthModifierType,
    pub edit_supporter_modifier: bool,
    pub supporter_modifier: String,
    pub supporter_modifier_type: WealthModifierType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum WealthModifierType {
    Absolute,
    Percentage,
}

impl WealthModifierType {
    fn as_str(&self) -> &'static str {
        match self {
            WealthModifierType::Absolute => "绝对值",
            WealthModifierType::Percentage => "百分比",
        }
    }
}

impl BulkEditDialog {
    pub fn new() -> Self {
        BulkEditDialog {
            open: false,
            teams: Vec::new(),
            selected_teams: Vec::new(),
            selected_count: 0,
            confirmed: false,
            error_message: None,
            
            edit_location: false,
            location: String::new(),
            edit_league: false,
            league_id: String::new(),
            edit_wealth_modifier: false,
            wealth_modifier: String::new(),
            wealth_modifier_type: WealthModifierType::Absolute,
            edit_supporter_modifier: false,
            supporter_modifier: String::new(),
            supporter_modifier_type: WealthModifierType::Absolute,
        }
    }

    pub fn open(&mut self, teams: Vec<Team>) {
        self.teams = teams.clone();
        self.selected_teams = vec![false; teams.len()];
        self.selected_count = 0;
        self.confirmed = false;
        self.error_message = None;
        
        // 重置编辑字段
        self.edit_location = false;
        self.location = String::new();
        self.edit_league = false;
        self.league_id = String::new();
        self.edit_wealth_modifier = false;
        self.wealth_modifier = String::new();
        self.wealth_modifier_type = WealthModifierType::Absolute;
        self.edit_supporter_modifier = false;
        self.supporter_modifier = String::new();
        self.supporter_modifier_type = WealthModifierType::Absolute;
        
        self.open = true;
    }

    pub fn show(&mut self, ctx: &Context) -> bool {
        if !self.open {
            return false;
        }

        let mut closed = false;
        let mut confirmed = false;

        setup_mac_window("批量编辑球队")
            .fixed_size([700.0, 500.0])
            .show(ctx, |ui| {
                ui.add_space(5.0);
                ui.heading("批量编辑球队数据");
                ui.add_space(5.0);
                widgets::horizontal_separator(ui);
                ui.add_space(10.0);

                // 分割为左右两栏
                ui.columns(2, |columns| {
                    // 左侧：球队选择列表
                    columns[0].vertical(|ui| {
                        ui.heading("选择要编辑的球队");
                        ui.add_space(5.0);
                        
                        // 全选/全不选按钮
                        ui.horizontal(|ui| {
                            if widgets::mac_button(ui, "全选") {
                                for selected in &mut self.selected_teams {
                                    *selected = true;
                                }
                                self.selected_count = self.teams.len();
                            }
                            
                            if widgets::mac_button(ui, "全不选") {
                                for selected in &mut self.selected_teams {
                                    *selected = false;
                                }
                                self.selected_count = 0;
                            }
                            
                            ui.label(format!("已选择: {}/{}", self.selected_count, self.teams.len()));
                        });
                        
                        ui.add_space(5.0);
                        
                        // 球队列表
                        egui::Frame::none()
                            .fill(Color32::from_rgb(255, 255, 255))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(egui::Margin::same(8.0))
                            .show(ui, |ui| {
                                ScrollArea::vertical()
                                    .max_height(300.0)
                                    .show(ui, |ui| {
                                        Grid::new("teams_grid")
                                            .num_columns(2)
                                            .spacing([10.0, 4.0])
                                            .striped(true)
                                            .show(ui, |ui| {
                                                for (idx, team) in self.teams.iter().enumerate() {
                                                    let checkbox = ui.checkbox(
                                                        &mut self.selected_teams[idx],
                                                        ""
                                                    );
                                                    
                                                    if checkbox.changed() {
                                                        self.selected_count = self.selected_teams.iter().filter(|&&selected| selected).count();
                                                    }
                                                    
                                                    ui.label(&team.name);
                                                    ui.end_row();
                                                }
                                            });
                                    });
                            });
                    });
                    
                    // 右侧：编辑选项
                    columns[1].vertical(|ui| {
                        ui.heading("批量编辑选项");
                        ui.add_space(5.0);
                        
                        // 地区编辑
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.edit_location, "修改地区");
                            ui.add_enabled(self.edit_location, egui::TextEdit::singleline(&mut self.location));
                        });
                        
                        // 联赛编辑
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.edit_league, "修改联赛ID");
                            ui.add_enabled(self.edit_league, egui::TextEdit::singleline(&mut self.league_id));
                        });
                        
                        ui.add_space(10.0);
                        ui.label("财富修改:");
                        
                        // 财富修改
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.edit_wealth_modifier, "修改财富");
                            ui.add_enabled(self.edit_wealth_modifier, egui::TextEdit::singleline(&mut self.wealth_modifier)
                                .hint_text("输入数值"));
                                
                            egui::ComboBox::from_id_source("wealth_modifier_type")
                                .selected_text(self.wealth_modifier_type.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.wealth_modifier_type, WealthModifierType::Absolute, "绝对值");
                                    ui.selectable_value(&mut self.wealth_modifier_type, WealthModifierType::Percentage, "百分比");
                                });
                        });
                        
                        ui.add_space(5.0);
                        ui.label("球迷数量修改:");
                        
                        // 球迷数量修改
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.edit_supporter_modifier, "修改球迷数量");
                            ui.add_enabled(self.edit_supporter_modifier, egui::TextEdit::singleline(&mut self.supporter_modifier)
                                .hint_text("输入数值"));
                                
                            egui::ComboBox::from_id_source("supporter_modifier_type")
                                .selected_text(self.supporter_modifier_type.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.supporter_modifier_type, WealthModifierType::Absolute, "绝对值");
                                    ui.selectable_value(&mut self.supporter_modifier_type, WealthModifierType::Percentage, "百分比");
                                });
                        });
                        
                        // 错误消息
                        if let Some(error) = &self.error_message {
                            ui.add_space(10.0);
                            widgets::error_message(ui, error);
                        }
                    });
                });

                ui.add_space(10.0);
                widgets::horizontal_separator(ui);
                ui.add_space(10.0);
                
                // 底部按钮
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if widgets::mac_primary_button(ui, "应用批量修改") {
                            if self.selected_count == 0 {
                                self.error_message = Some("请至少选择一个球队".to_string());
                            } else if !self.edit_location && !self.edit_league && 
                                    !self.edit_wealth_modifier && !self.edit_supporter_modifier {
                                self.error_message = Some("请至少选择一项要修改的属性".to_string());
                            } else if self.validate_inputs() {
                                confirmed = true;
                                closed = true;
                            }
                        }
                        
                        ui.add_space(10.0);
                        
                        if widgets::mac_button(ui, "取消") {
                            closed = true;
                        }
                    });
                });
            });

        if closed && !confirmed {
            self.open = false;
        }

        self.confirmed = confirmed;
        confirmed
    }
    
    fn validate_inputs(&mut self) -> bool {
        // 验证联赛ID
        if self.edit_league {
            if self.league_id.trim().is_empty() {
                self.error_message = Some("联赛ID不能为空".to_string());
                return false;
            }
            
            if let Err(_) = self.league_id.trim().parse::<i64>() {
                self.error_message = Some("联赛ID必须是有效的整数".to_string());
                return false;
            }
        }
        
        // 验证财富修改器
        if self.edit_wealth_modifier {
            if self.wealth_modifier.trim().is_empty() {
                self.error_message = Some("财富修改值不能为空".to_string());
                return false;
            }
            
            if let Err(_) = self.wealth_modifier.trim().parse::<i64>() {
                self.error_message = Some("财富修改值必须是有效的整数".to_string());
                return false;
            }
        }
        
        // 验证球迷数量修改器
        if self.edit_supporter_modifier {
            if self.supporter_modifier.trim().is_empty() {
                self.error_message = Some("球迷数量修改值不能为空".to_string());
                return false;
            }
            
            if let Err(_) = self.supporter_modifier.trim().parse::<i64>() {
                self.error_message = Some("球迷数量修改值必须是有效的整数".to_string());
                return false;
            }
        }
        
        true
    }
    
    pub fn get_modified_teams(&self) -> Vec<Team> {
        let mut modified_teams = Vec::new();
        
        for (idx, selected) in self.selected_teams.iter().enumerate() {
            if *selected {
                if let Some(team) = self.teams.get(idx).cloned() {
                    let mut modified_team = team;
                    
                    // 应用地区修改
                    if self.edit_location {
                        modified_team.location = self.location.clone();
                    }
                    
                    // 应用联赛ID修改
                    if self.edit_league {
                        if let Ok(league_id) = self.league_id.trim().parse::<i64>() {
                            modified_team.league_id = league_id;
                        }
                    }
                    
                    // 应用财富修改
                    if self.edit_wealth_modifier {
                        if let Ok(modifier) = self.wealth_modifier.trim().parse::<i64>() {
                            match self.wealth_modifier_type {
                                WealthModifierType::Absolute => {
                                    modified_team.wealth = modifier;
                                },
                                WealthModifierType::Percentage => {
                                    let percentage = (modifier as f64) / 100.0;
                                    modified_team.wealth = ((modified_team.wealth as f64) * (1.0 + percentage)) as i64;
                                }
                            }
                        }
                    }
                    
                    // 应用球迷数量修改
                    if self.edit_supporter_modifier {
                        if let Ok(modifier) = self.supporter_modifier.trim().parse::<i64>() {
                            match self.supporter_modifier_type {
                                WealthModifierType::Absolute => {
                                    modified_team.supporter_count = modifier;
                                },
                                WealthModifierType::Percentage => {
                                    let percentage = (modifier as f64) / 100.0;
                                    modified_team.supporter_count = ((modified_team.supporter_count as f64) * (1.0 + percentage)) as i64;
                                }
                            }
                        }
                    }
                    
                    modified_teams.push(modified_team);
                }
            }
        }
        
        modified_teams
    }
} 