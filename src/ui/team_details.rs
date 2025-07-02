use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Image, Label, Layout, RichText, Ui};
use egui::widgets::TextEdit;
use log::{error, info};

use crate::data::team::Team;
use crate::error::Result;
use crate::ui::widgets;
use crate::utils;

pub struct TeamDetailsView {
    pub team: Option<Team>,
    pub leagues: HashMap<i64, String>,
    pub logo_texture: Option<egui::TextureHandle>,
    pub edited_fields: EditableTeamFields,
    pub has_changes: bool,
}

#[derive(Debug, Clone, Default)]
pub struct EditableTeamFields {
    pub name: String,
    pub wealth: String,
    pub found_year: String,
    pub location: String,
    pub supporter_count: String,
    pub stadium_name: String,
    pub nickname: String,
}

impl TeamDetailsView {
    pub fn new() -> Self {
        TeamDetailsView {
            team: None,
            leagues: HashMap::new(),
            logo_texture: None,
            edited_fields: EditableTeamFields::default(),
            has_changes: false,
        }
    }

    pub fn set_team(&mut self, team: Team) {
        self.edited_fields = EditableTeamFields {
            name: team.name.clone(),
            wealth: team.wealth.to_string(),
            found_year: team.found_year.to_string(),
            location: team.location.clone(),
            supporter_count: team.supporter_count.to_string(),
            stadium_name: team.stadium_name.clone(),
            nickname: team.nickname.clone(),
        };
        self.team = Some(team);
        self.has_changes = false;
    }

    pub fn set_leagues(&mut self, leagues: HashMap<i64, String>) {
        self.leagues = leagues;
    }

    pub fn load_logo(&mut self, ctx: &egui::Context, db_dir: &Path, team_id: i64) -> Result<()> {
        let logo_path = utils::create_logo_path(db_dir, team_id);
        
        if utils::file_exists(&logo_path) {
            match utils::load_and_resize_image(&logo_path, 128, 128) {
                Ok(img) => {
                    let width = img.width() as usize;
                    let height = img.height() as usize;
                    let rgba8 = utils::image_to_rgba8_bytes(&img);
                    
                    self.logo_texture = Some(ctx.load_texture(
                        format!("team_logo_{}", team_id),
                        egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba8),
                        egui::TextureOptions::LINEAR
                    ));
                    Ok(())
                },
                Err(e) => {
                    error!("加载Logo失败: {}", e);
                    self.logo_texture = None;
                    Err(e)
                }
            }
        } else {
            self.logo_texture = None;
            Ok(())
        }
    }

    pub fn get_edited_team(&self) -> Option<Team> {
        let team = self.team.as_ref()?;
        
        // 尝试解析编辑后的字段
        let parse_wealth = self.edited_fields.wealth.parse::<i64>().unwrap_or(0);
        let parse_found_year = self.edited_fields.found_year.parse::<i64>().unwrap_or(0);
        let parse_supporter_count = self.edited_fields.supporter_count.parse::<i64>().unwrap_or(0);
        
        Some(Team {
            id: team.id,
            name: self.edited_fields.name.clone(),
            wealth: parse_wealth,
            found_year: parse_found_year,
            location: self.edited_fields.location.clone(),
            supporter_count: parse_supporter_count,
            stadium_name: self.edited_fields.stadium_name.clone(),
            nickname: self.edited_fields.nickname.clone(),
            league_id: team.league_id,
        })
    }

    pub fn ui(&mut self, ui: &mut Ui) -> (bool, bool) {
        let mut logo_clicked = false;
        let mut field_changed = false;

        if let Some(team) = &self.team {
            widgets::titled_frame("球队详情", ui, |ui| {
                // Logo区域
                ui.vertical_centered(|ui| {
                    // 创建一个可视的框架，使Logo区域看起来可点击
                    egui::Frame::none()
                        .fill(Color32::from_rgb(245, 245, 245))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)))
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                let mut response = if let Some(texture) = &self.logo_texture {
                                    // 显示Logo图像
                                    let logo = Image::new(texture)
                                        .max_size(egui::vec2(128.0, 128.0));
                                    ui.add(logo.sense(egui::Sense::click()))
                                } else {
                                    // 显示"无Logo"文本
                                    ui.add(Label::new(
                                        RichText::new("无Logo\n点击添加").heading().color(Color32::GRAY)
                                    ).sense(egui::Sense::click()))
                                };
                                
                                // 添加明确的按钮提示
                                ui.add_space(5.0);
                                ui.add(egui::Label::new(
                                    RichText::new("📷 点击更换Logo")
                                        .strong()
                                        .color(Color32::from_rgb(50, 100, 200))
                                ));
                                
                                // 检查点击
                                if response.clicked() {
                                    logo_clicked = true;
                                }
                                
                                // 鼠标悬停效果
                                if response.hovered() {
                                    response.mark_changed(); // 确保UI会更新
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                }
                            });
                        });
                });

                ui.add_space(10.0);
                ui.heading("基本信息");
                widgets::horizontal_separator(ui);

                // 基本信息表单
                ui.columns(2, |columns| {
                    // 左列
                    let mut changed = false;
                    let id_str = team.id.to_string();
                    widgets::readonly_form_row(&mut columns[0], "编号:", &id_str);
                    changed |= widgets::form_row(&mut columns[0], "球队名称:", &mut self.edited_fields.name);
                    changed |= widgets::form_row(&mut columns[0], "球队财富（万）:", &mut self.edited_fields.wealth);
                    changed |= widgets::form_row(&mut columns[0], "成立年份:", &mut self.edited_fields.found_year);
                    
                    // 右列
                    changed |= widgets::form_row(&mut columns[1], "所在地区:", &mut self.edited_fields.location);
                    changed |= widgets::form_row(&mut columns[1], "支持者数量:", &mut self.edited_fields.supporter_count);
                    changed |= widgets::form_row(&mut columns[1], "主场名称:", &mut self.edited_fields.stadium_name);
                    changed |= widgets::form_row(&mut columns[1], "球队昵称:", &mut self.edited_fields.nickname);
                    
                    self.has_changes |= changed;
                    field_changed = changed;
                });

                // 联赛信息
                ui.horizontal(|ui| {
                    ui.label("所在联赛:");
                    let league_name = self.leagues.get(&team.league_id)
                        .map(|name| format!("{} (ID: {})", name, team.league_id))
                        .unwrap_or_else(|| format!("未知联赛 (ID: {})", team.league_id));
                    ui.label(league_name);
                });
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.label("请选择一个球队");
            });
        }

        (logo_clicked, field_changed)
    }
} 