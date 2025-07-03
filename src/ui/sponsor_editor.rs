use std::path::{Path, PathBuf};
use egui::{Color32, Context, Grid, Image, Layout, RichText, ScrollArea, Sense, TextEdit, Ui};
use image::GenericImageView;
use log::{error, info};
use native_dialog::FileDialog;

use crate::data::database::Database;
use crate::data::sponsor::{Sponsor, FA};
use crate::error::Result;

#[derive(PartialEq, Clone, Copy)]
pub enum SponsorEditorMode {
    Sponsor,
    FA,
}

pub struct SponsorEditorView {
    // 数据
    sponsors: Vec<Sponsor>,
    fas: Vec<FA>,
    displayed_sponsors: Vec<Sponsor>,
    displayed_fas: Vec<FA>,
    
    // 状态
    mode: SponsorEditorMode,
    search_query: String,
    selected_sponsor_idx: Option<usize>,
    selected_fa_idx: Option<usize>,
    edited_sponsor: Option<Sponsor>,
    edited_fa: Option<FA>,
    logo_texture: Option<egui::TextureHandle>,
    pub show_logo_dialog: bool,
    
    // 字段标签
    sponsor_field_labels: Vec<(&'static str, &'static str)>,
    fa_field_labels: Vec<(&'static str, &'static str)>,
}

impl SponsorEditorView {
    pub fn new() -> Self {
        Self {
            sponsors: Vec::new(),
            fas: Vec::new(),
            displayed_sponsors: Vec::new(),
            displayed_fas: Vec::new(),
            mode: SponsorEditorMode::Sponsor,
            search_query: String::new(),
            selected_sponsor_idx: None,
            selected_fa_idx: None,
            edited_sponsor: None,
            edited_fa: None,
            logo_texture: None,
            show_logo_dialog: false,
            sponsor_field_labels: vec![
                ("sponsor_name", "赞助商名称"),
                ("sponsor_type", "类型"),
                ("unlocked", "是否解锁"),
                ("description", "描述"),
                ("brand_offer", "装备赞助（万）"),
                ("chest_offer", "胸前广告（万）"),
                ("back_offer", "背部广告（万）"),
                ("sleeve_offer", "袖子广告（万）"),
                ("billboard_offer", "广告牌（万）"),
                ("bib_offer", "号码布广告（万）"),
                ("banner_offer", "横幅广告（万）"),
                ("headquarter_location", "总部地点"),
                ("industry", "行业"),
                ("location_restriction", "地域限制"),
            ],
            fa_field_labels: vec![
                ("id", "ID"),
                ("title", "标题"),
                ("location", "位置"),
                ("subsidy_level", "补贴级别"),
                ("main_operator_name", "主要运营商名称"),
                ("youth_operator_name", "青年运营商名称"),
                ("competition_operator_name", "竞赛运营商名称"),
                ("youth_development", "青年发展"),
                ("youth_operator_relation", "青年运营商关系"),
                ("youth_operator_ability", "青年运营商能力"),
                ("competition_operator_relation", "竞赛运营商关系"),
                ("competition_operator_ability", "竞赛运营商能力"),
                ("main_operator_relation", "主要运营商关系"),
                ("main_operator_ability", "主要运营商能力"),
                ("main_operator_fame", "主要运营商声望"),
                ("youth_operator_fame", "青年运营商声望"),
                ("competition_operator_fame", "竞赛运营商声望"),
            ],
        }
    }

    pub fn set_sponsors(&mut self, sponsors: Vec<Sponsor>) {
        self.sponsors = sponsors;
        self.apply_search_filter();
    }

    pub fn set_fas(&mut self, fas: Vec<FA>) {
        self.fas = fas;
        self.apply_search_filter();
    }

    pub fn get_edited_sponsor(&self) -> Option<Sponsor> {
        self.edited_sponsor.clone()
    }

    pub fn get_edited_fa(&self) -> Option<FA> {
        self.edited_fa.clone()
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            SponsorEditorMode::Sponsor => SponsorEditorMode::FA,
            SponsorEditorMode::FA => SponsorEditorMode::Sponsor,
        };
        
        // 清除选择和编辑状态
        self.selected_sponsor_idx = None;
        self.selected_fa_idx = None;
        self.edited_sponsor = None;
        self.edited_fa = None;
        self.logo_texture = None;
    }

    fn apply_search_filter(&mut self) {
        let query = self.search_query.to_lowercase();
        
        if query.is_empty() {
            self.displayed_sponsors = self.sponsors.clone();
            self.displayed_fas = self.fas.clone();
        } else {
            // 过滤赞助商
            self.displayed_sponsors = self.sponsors.iter()
                .filter(|s| {
                    s.sponsor_name.to_lowercase().contains(&query) ||
                    s.description.to_lowercase().contains(&query) ||
                    s.industry.to_lowercase().contains(&query) ||
                    s.headquarter_location.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
            
            // 过滤足协
            self.displayed_fas = self.fas.iter()
                .filter(|f| {
                    f.title.to_lowercase().contains(&query) ||
                    f.location.to_lowercase().contains(&query) ||
                    f.main_operator_name.to_lowercase().contains(&query) ||
                    f.youth_operator_name.to_lowercase().contains(&query) ||
                    f.competition_operator_name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.apply_search_filter();
    }

    pub fn create_new_sponsor(&mut self, db: &Database) -> Result<()> {
        // 获取最后一个赞助商作为模板
        if let Some(last_sponsor) = self.sponsors.last() {
            // 创建新的赞助商名称
            let new_name = format!("赞助商{}", self.sponsors.len() + 1);
            
            // 复制最后一个赞助商的数据
            let mut new_sponsor = last_sponsor.clone();
            new_sponsor.sponsor_name = new_name;
            
            // 保存到数据库
            db.create_new_sponsor(&new_sponsor)?;
            
            // 添加到列表
            self.sponsors.push(new_sponsor.clone());
            self.apply_search_filter();
            
            // 选择新创建的赞助商
            if let Some(idx) = self.displayed_sponsors.iter().position(|s| s.sponsor_name == new_sponsor.sponsor_name) {
                self.selected_sponsor_idx = Some(idx);
                self.edited_sponsor = Some(self.displayed_sponsors[idx].clone());
            }
        }
        
        Ok(())
    }

    pub fn replace_logo(&mut self, ctx: &Context, db: &Database) -> Result<()> {
        if let Some(sponsor) = &self.edited_sponsor {
            let sponsor_name = sponsor.sponsor_name.clone();
            
            let db_dir = db.get_db_directory().ok_or_else(|| {
                error!("无法获取数据库目录");
                crate::error::AppError::DatabaseError("无法获取数据库目录".to_string())
            })?;
            
            // 创建SponsorLogos目录（如果不存在）
            let logo_dir = db_dir.join("SponsorLogos");
            if !logo_dir.exists() {
                std::fs::create_dir_all(&logo_dir)?;
            }
            
            // 打开文件对话框
            let dialog = FileDialog::new()
                .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp"])
                .show_open_single_file();
            
            if let Ok(Some(path)) = dialog {
                // 加载图片
                let img = image::open(&path)?;
                
                // 调整大小并保持宽高比
                let scaled_img = img.resize(512, 512, image::imageops::FilterType::Lanczos3);
                
                // 保存为PNG
                let target_path = logo_dir.join(format!("{}.png", sponsor_name));
                scaled_img.save(&target_path)?;
                
                // 更新编辑中的赞助商
                if let Some(sponsor) = &mut self.edited_sponsor {
                    sponsor.logo_path = Some(target_path.clone());
                }
                
                // 更新列表中的赞助商
                if let Some(idx) = self.selected_sponsor_idx {
                    if idx < self.displayed_sponsors.len() {
                        self.displayed_sponsors[idx].logo_path = Some(target_path.clone());
                    }
                }
                
                // 更新原始列表中的赞助商
                if let Some(idx) = self.sponsors.iter().position(|s| s.sponsor_name == sponsor_name) {
                    self.sponsors[idx].logo_path = Some(target_path);
                }
                
                // 加载纹理
                self.load_logo_texture(ctx);
                
                info!("Logo替换成功");
            }
        }
        
        Ok(())
    }

    fn load_logo_texture(&mut self, ctx: &Context) {
        if let Some(sponsor) = &self.edited_sponsor {
            if let Some(logo_path) = &sponsor.logo_path {
                if logo_path.exists() {
                    // 加载图片
                    if let Ok(img) = image::open(logo_path) {
                        let size = [img.width() as _, img.height() as _];
                        let image_buffer = img.to_rgba8();
                        let pixels = image_buffer.into_raw();
                        
                        // 创建纹理
                        self.logo_texture = Some(ctx.load_texture(
                            "sponsor_logo",
                            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                            egui::TextureOptions::LINEAR
                        ));
                    }
                } else {
                    self.logo_texture = None;
                }
            } else {
                self.logo_texture = None;
            }
        } else {
            self.logo_texture = None;
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            // 模式切换按钮
            let mode_text = match self.mode {
                SponsorEditorMode::Sponsor => "切换到足协修改",
                SponsorEditorMode::FA => "切换到赞助商修改",
            };
            if ui.button(mode_text).clicked() {
                self.toggle_mode();
            }
            
            ui.separator();
            
            // 搜索框
            ui.label("搜索:");
            let search_response = ui.text_edit_singleline(&mut self.search_query);
            if search_response.changed() {
                self.apply_search_filter();
            }
            
            if ui.button("搜索").clicked() {
                self.apply_search_filter();
            }
            
            ui.separator();
            
            // 新建赞助商按钮（仅在赞助商模式下显示）
            if self.mode == SponsorEditorMode::Sponsor {
                if ui.button("新建赞助商").clicked() {
                    self.show_logo_dialog = true;
                }
            }
        });
        
        ui.separator();
        
        // 主内容区域
        ui.columns(2, |columns| {
            // 左侧列表
            self.ui_list(&mut columns[0], ctx);
            
            // 右侧详情
            self.ui_details(&mut columns[1], ctx);
        });
    }

    fn ui_list(&mut self, ui: &mut Ui, ctx: &Context) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading(match self.mode {
                SponsorEditorMode::Sponsor => "赞助商列表",
                SponsorEditorMode::FA => "足协列表",
            });
            
            ui.separator();
            
            match self.mode {
                SponsorEditorMode::Sponsor => {
                    for (idx, sponsor) in self.displayed_sponsors.iter().enumerate() {
                        let is_selected = self.selected_sponsor_idx == Some(idx);
                        let text = RichText::new(&sponsor.sponsor_name)
                            .color(if is_selected { Color32::BLUE } else { Color32::BLACK });
                        
                        if ui.selectable_label(is_selected, text).clicked() {
                            let sponsor_clone = sponsor.clone();
                            self.selected_sponsor_idx = Some(idx);
                            self.edited_sponsor = Some(sponsor_clone);
                            // 延迟加载纹理，避免借用冲突
                            let logo_path = sponsor.logo_path.clone();
                            if logo_path.is_some() {
                                ui.ctx().request_repaint(); // 请求重绘以加载纹理
                            }
                        }
                    }
                    
                    // 如果有选中的赞助商但没有加载纹理，尝试加载
                    if let Some(idx) = self.selected_sponsor_idx {
                        if self.logo_texture.is_none() {
                            if let Some(sponsor) = &self.edited_sponsor {
                                if sponsor.logo_path.is_some() {
                                    self.load_logo_texture(ctx);
                                }
                            }
                        }
                    }
                },
                SponsorEditorMode::FA => {
                    for (idx, fa) in self.displayed_fas.iter().enumerate() {
                        let is_selected = self.selected_fa_idx == Some(idx);
                        let text = RichText::new(&fa.title)
                            .color(if is_selected { Color32::BLUE } else { Color32::BLACK });
                        
                        if ui.selectable_label(is_selected, text).clicked() {
                            self.selected_fa_idx = Some(idx);
                            self.edited_fa = Some(fa.clone());
                        }
                    }
                },
            }
        });
    }

    fn ui_details(&mut self, ui: &mut Ui, ctx: &Context) {
        ScrollArea::vertical().show(ui, |ui| {
            match self.mode {
                SponsorEditorMode::Sponsor => {
                    if let Some(sponsor) = &mut self.edited_sponsor {
                        ui.heading("赞助商详情");
                        ui.separator();
                        
                        // Logo显示
                        ui.vertical_centered(|ui| {
                            if let Some(texture) = &self.logo_texture {
                                let size = 180.0;
                                let image = egui::widgets::Image::new(texture)
                                    .max_size(egui::vec2(size, size));
                                if ui.add(image).clicked() {
                                    self.show_logo_dialog = true;
                                }
                                
                                ui.add_space(5.0);
                                ui.label("点击更换Logo");
                            } else {
                                let button = ui.add_sized([180.0, 180.0], egui::Button::new("无Logo\n点击添加"));
                                if button.clicked() {
                                    self.show_logo_dialog = true;
                                }
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        // 表单
                        Grid::new("sponsor_form").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
                            // 赞助商名称
                            ui.label("赞助商名称:");
                            ui.text_edit_singleline(&mut sponsor.sponsor_name);
                            ui.end_row();
                            
                            // 类型
                            ui.label("类型:");
                            egui::ComboBox::from_id_source("sponsor_type")
                                .selected_text(&sponsor.sponsor_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut sponsor.sponsor_type, "Brand".to_string(), "Brand");
                                    ui.selectable_value(&mut sponsor.sponsor_type, "Generic".to_string(), "Generic");
                                });
                            ui.end_row();
                            
                            // 是否解锁
                            ui.label("是否解锁:");
                            egui::ComboBox::from_id_source("sponsor_unlocked")
                                .selected_text(if sponsor.unlocked == "0" { "否" } else { "是" })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut sponsor.unlocked, "0".to_string(), "否");
                                    ui.selectable_value(&mut sponsor.unlocked, "1".to_string(), "是");
                                });
                            ui.end_row();
                            
                            // 描述
                            ui.label("描述:");
                            ui.add(TextEdit::multiline(&mut sponsor.description).desired_rows(3));
                            ui.end_row();
                            
                            // 装备赞助
                            ui.label("装备赞助（万）:");
                            ui.text_edit_singleline(&mut sponsor.brand_offer);
                            ui.end_row();
                            
                            // 胸前广告
                            ui.label("胸前广告（万）:");
                            ui.text_edit_singleline(&mut sponsor.chest_offer);
                            ui.end_row();
                            
                            // 背部广告
                            ui.label("背部广告（万）:");
                            ui.text_edit_singleline(&mut sponsor.back_offer);
                            ui.end_row();
                            
                            // 袖子广告
                            ui.label("袖子广告（万）:");
                            ui.text_edit_singleline(&mut sponsor.sleeve_offer);
                            ui.end_row();
                            
                            // 广告牌
                            ui.label("广告牌（万）:");
                            ui.text_edit_singleline(&mut sponsor.billboard_offer);
                            ui.end_row();
                            
                            // 号码布广告
                            ui.label("号码布广告（万）:");
                            ui.text_edit_singleline(&mut sponsor.bib_offer);
                            ui.end_row();
                            
                            // 横幅广告
                            ui.label("横幅广告（万）:");
                            ui.text_edit_singleline(&mut sponsor.banner_offer);
                            ui.end_row();
                            
                            // 总部地点
                            ui.label("总部地点:");
                            ui.text_edit_singleline(&mut sponsor.headquarter_location);
                            ui.end_row();
                            
                            // 行业
                            ui.label("行业:");
                            ui.text_edit_singleline(&mut sponsor.industry);
                            ui.end_row();
                            
                            // 地域限制
                            ui.label("地域限制:");
                            ui.text_edit_singleline(&mut sponsor.location_restriction);
                            ui.end_row();
                        });
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一个赞助商");
                        });
                    }
                },
                SponsorEditorMode::FA => {
                    if let Some(fa) = &mut self.edited_fa {
                        ui.heading("足协详情");
                        ui.separator();
                        
                        // 表单
                        Grid::new("fa_form").num_columns(2).spacing([10.0, 10.0]).show(ui, |ui| {
                            // ID
                            ui.label("ID:");
                            ui.label(fa.id.to_string());
                            ui.end_row();
                            
                            // 标题
                            ui.label("标题:");
                            ui.text_edit_singleline(&mut fa.title);
                            ui.end_row();
                            
                            // 位置
                            ui.label("位置:");
                            ui.text_edit_singleline(&mut fa.location);
                            ui.end_row();
                            
                            // 补贴级别
                            ui.label("补贴级别:");
                            ui.text_edit_singleline(&mut fa.subsidy_level);
                            ui.end_row();
                            
                            // 主要运营商名称
                            ui.label("主要运营商名称:");
                            ui.text_edit_singleline(&mut fa.main_operator_name);
                            ui.end_row();
                            
                            // 青年运营商名称
                            ui.label("青年运营商名称:");
                            ui.text_edit_singleline(&mut fa.youth_operator_name);
                            ui.end_row();
                            
                            // 竞赛运营商名称
                            ui.label("竞赛运营商名称:");
                            ui.text_edit_singleline(&mut fa.competition_operator_name);
                            ui.end_row();
                            
                            // 青年发展
                            ui.label("青年发展:");
                            ui.text_edit_singleline(&mut fa.youth_development);
                            ui.end_row();
                            
                            // 青年运营商关系
                            ui.label("青年运营商关系:");
                            ui.text_edit_singleline(&mut fa.youth_operator_relation);
                            ui.end_row();
                            
                            // 青年运营商能力
                            ui.label("青年运营商能力:");
                            ui.text_edit_singleline(&mut fa.youth_operator_ability);
                            ui.end_row();
                            
                            // 竞赛运营商关系
                            ui.label("竞赛运营商关系:");
                            ui.text_edit_singleline(&mut fa.competition_operator_relation);
                            ui.end_row();
                            
                            // 竞赛运营商能力
                            ui.label("竞赛运营商能力:");
                            ui.text_edit_singleline(&mut fa.competition_operator_ability);
                            ui.end_row();
                            
                            // 主要运营商关系
                            ui.label("主要运营商关系:");
                            ui.text_edit_singleline(&mut fa.main_operator_relation);
                            ui.end_row();
                            
                            // 主要运营商能力
                            ui.label("主要运营商能力:");
                            ui.text_edit_singleline(&mut fa.main_operator_ability);
                            ui.end_row();
                            
                            // 主要运营商声望
                            ui.label("主要运营商声望:");
                            ui.text_edit_singleline(&mut fa.main_operator_fame);
                            ui.end_row();
                            
                            // 青年运营商声望
                            ui.label("青年运营商声望:");
                            ui.text_edit_singleline(&mut fa.youth_operator_fame);
                            ui.end_row();
                            
                            // 竞赛运营商声望
                            ui.label("竞赛运营商声望:");
                            ui.text_edit_singleline(&mut fa.competition_operator_fame);
                            ui.end_row();
                        });
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一个足协");
                        });
                    }
                },
            }
        });
    }
} 