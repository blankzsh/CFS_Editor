use std::path::PathBuf;
use std::time::{Duration, Instant};

use egui::{Context, CentralPanel, SidePanel, TopBottomPanel, Ui, Color32};
use eframe::{App, CreationContext, Frame};
use log::{error, info};
use native_dialog::FileDialog;

use crate::data::database::Database;
use crate::error::Result;
use crate::ui::dialogs::{BulkEditDialog, ConfirmDialog, MessageDialog as UiMessageDialog, StaffEditDialog};
use crate::ui::staff_list::StaffListView;
use crate::ui::team_details::TeamDetailsView;
use crate::ui::team_list::TeamListView;
use crate::ui::visualization::VisualizationView;
use crate::ui::widgets;
use crate::utils;

// 自动保存间隔（秒）
const AUTO_SAVE_INTERVAL: u64 = 30;

#[derive(PartialEq, Clone, Copy)]
enum ActiveTab {
    TeamDetails,
    Visualization,
}

impl ActiveTab {
    fn as_str(&self) -> &'static str {
        match self {
            ActiveTab::TeamDetails => "球队详情",
            ActiveTab::Visualization => "数据可视化",
        }
    }
}

pub struct TeamEditorApp {
    // 数据
    database: Database,
    
    // UI组件
    team_list: TeamListView,
    team_details: TeamDetailsView,
    staff_list: StaffListView,
    visualization: VisualizationView,
    active_tab: ActiveTab,
    
    // 对话框
    staff_edit_dialog: StaffEditDialog,
    message_dialog: UiMessageDialog,
    confirm_dialog: ConfirmDialog,
    bulk_edit_dialog: BulkEditDialog,
    
    // 状态
    status_message: String,
    export_path: Option<PathBuf>,
    
    // 自动保存
    auto_save_enabled: bool,
    last_auto_save: Instant,
    has_unsaved_changes: bool,
    auto_save_countdown: u64,
}

impl TeamEditorApp {
    pub fn new(cc: &CreationContext) -> Self {
        // 应用Mac风格主题
        crate::ui::theme::setup_mac_theme(&cc.egui_ctx);

        TeamEditorApp {
            database: Database::new(),
            team_list: TeamListView::new(),
            team_details: TeamDetailsView::new(),
            staff_list: StaffListView::new(),
            visualization: VisualizationView::new(),
            active_tab: ActiveTab::TeamDetails,
            staff_edit_dialog: StaffEditDialog::new(),
            message_dialog: UiMessageDialog::new(),
            confirm_dialog: ConfirmDialog::new(),
            bulk_edit_dialog: BulkEditDialog::new(),
            status_message: "就绪".to_string(),
            export_path: None,
            auto_save_enabled: true,
            last_auto_save: Instant::now(),
            has_unsaved_changes: false,
            auto_save_countdown: AUTO_SAVE_INTERVAL,
        }
    }

    fn load_database(&mut self, ctx: &Context) {
        let dialog = FileDialog::new()
            .add_filter("SQLite 数据库", &["db"])
            .add_filter("所有文件", &["*"])
            .show_open_single_file();
        
        if let Ok(Some(path)) = dialog {
            match self.database.connect(&path) {
                Ok(_) => {
                    if let Err(e) = self.load_data(ctx) {
                        self.show_message("错误", &format!("加载数据失败: {}", e));
                        error!("加载数据失败: {}", e);
                        return;
                    }
                    
                    let filename = path.file_name().unwrap_or_default().to_string_lossy();
                    self.show_message("成功", &format!("数据库加载成功: {}", filename));
                    self.set_status(&format!("已加载数据库: {}", filename));
                },
                Err(e) => {
                    self.show_message("错误", &format!("加载数据库失败: {}", e));
                    error!("加载数据库失败: {}", e);
                }
            }
        }
    }

    fn load_data(&mut self, _ctx: &Context) -> Result<()> {
        // 加载球队数据
        let teams = self.database.load_teams()?;
        self.team_list.set_teams(teams.clone());
        
        // 加载联赛数据
        let leagues = self.database.load_leagues()?;
        self.team_details.set_leagues(leagues);
        
        // 加载员工数据
        let staff = self.database.load_staff()?;
        self.staff_list.set_all_staff(staff);
        
        // 更新可视化数据
        self.visualization.set_teams(teams);
        
        info!("已加载 {} 个球队", self.team_list.teams.len());
        Ok(())
    }

    fn save_team_changes(&mut self) {
        if !self.database.is_connected() {
            self.show_message("警告", "请先加载数据库");
            return;
        }

        if let Some(_team) = self.team_details.get_edited_team() {
            self.confirm_dialog.show_confirm(
                "确认保存",
                "您确定要保存对球队数据的修改吗？"
            );
        } else {
            self.show_message("警告", "请先选择一个球队");
        }
    }

    fn handle_confirm_save(&mut self, ctx: &Context) {
        if self.confirm_dialog.confirmed {
            if let Some(team) = self.team_details.get_edited_team() {
                match self.database.update_team(&team) {
                    Ok(_) => {
                        // 刷新数据
                        if let Err(e) = self.load_data(ctx) {
                            error!("刷新数据失败: {}", e);
                        }
                        
                        // 重新选择当前球队
                        self.team_list.select_team_by_id(team.id);
                        if let Some(team_id) = self.team_list.get_selected_team_id() {
                            self.select_team(team_id, ctx);
                        }
                        
                        self.show_message("成功", "球队数据已保存");
                        self.set_status(&format!("已保存球队 {} 的修改", team.name));
                        
                        // 重置自动保存状态
                        self.has_unsaved_changes = false;
                        self.last_auto_save = Instant::now();
                    },
                    Err(e) => {
                        self.show_message("错误", &format!("保存失败: {}", e));
                        error!("保存球队数据失败: {}", e);
                    }
                }
            }
        }
    }

    fn auto_save(&mut self, ctx: &Context) -> bool {
        if !self.auto_save_enabled || !self.has_unsaved_changes || !self.database.is_connected() {
            return false;
        }
        
        if let Some(team) = self.team_details.get_edited_team() {
            match self.database.update_team(&team) {
                Ok(_) => {
                    // 刷新数据但不显示消息
                    if let Err(e) = self.load_data(ctx) {
                        error!("自动保存后刷新数据失败: {}", e);
                    }
                    
                    // 重新选择当前球队
                    self.team_list.select_team_by_id(team.id);
                    if let Some(team_id) = self.team_list.get_selected_team_id() {
                        self.select_team(team_id, ctx);
                    }
                    
                    self.set_status(&format!("已自动保存球队 {} 的修改", team.name));
                    info!("自动保存成功: 球队 {}", team.name);
                    
                    // 重置自动保存状态
                    self.has_unsaved_changes = false;
                    self.last_auto_save = Instant::now();
                    self.auto_save_countdown = AUTO_SAVE_INTERVAL;
                    
                    return true;
                },
                Err(e) => {
                    error!("自动保存失败: {}", e);
                    return false;
                }
            }
        }
        
        false
    }

    fn toggle_auto_save(&mut self) {
        self.auto_save_enabled = !self.auto_save_enabled;
        if self.auto_save_enabled {
            self.set_status("自动保存已启用");
        } else {
            self.set_status("自动保存已禁用");
        }
    }

    fn update_auto_save_timer(&mut self) {
        if !self.auto_save_enabled || !self.has_unsaved_changes {
            return;
        }
        
        let elapsed = self.last_auto_save.elapsed().as_secs();
        if elapsed >= AUTO_SAVE_INTERVAL {
            self.auto_save_countdown = 0;
        } else {
            self.auto_save_countdown = AUTO_SAVE_INTERVAL - elapsed;
        }
    }

    fn select_team(&mut self, team_id: i64, ctx: &Context) {
        if let Some(team) = self.team_list.teams.iter().find(|t| t.id == team_id).cloned() {
            // 更新球队详情
            self.team_details.set_team(team);
            
            // 加载Logo
            if let Some(db_dir) = self.database.get_db_directory() {
                if let Err(e) = self.team_details.load_logo(ctx, &db_dir, team_id) {
                    error!("加载Logo失败: {}", e);
                }
            }
            
            // 更新员工列表
            self.staff_list.update_team_staff(team_id);
            
            self.set_status(&format!("已选择球队: ID={}", team_id));
            
            // 重置自动保存状态
            self.has_unsaved_changes = false;
            self.last_auto_save = Instant::now();
        }
    }

    fn replace_logo(&mut self, ctx: &Context) {
        if !self.database.is_connected() {
            self.show_message("警告", "请先加载数据库");
            return;
        }

        if let Some(team_id) = self.team_list.get_selected_team_id() {
            if let Some(db_dir) = self.database.get_db_directory() {
                let dialog = FileDialog::new()
                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif"])
                    .add_filter("所有文件", &["*"])
                    .show_open_single_file();
                
                if let Ok(Some(path)) = dialog {
                    let logo_path = utils::create_logo_path(&db_dir, team_id);
                    
                    match utils::load_and_resize_image(&path, 128, 128) {
                        Ok(img) => {
                            if let Err(e) = utils::save_image(&img, &logo_path) {
                                self.show_message("错误", &format!("保存Logo失败: {}", e));
                                error!("保存Logo失败: {}", e);
                            } else {
                                // 重新加载Logo
                                if let Err(e) = self.team_details.load_logo(ctx, &db_dir, team_id) {
                                    error!("重新加载Logo失败: {}", e);
                                }
                                self.show_message("成功", "Logo已替换");
                            }
                        },
                        Err(e) => {
                            self.show_message("错误", &format!("加载图片失败: {}", e));
                            error!("加载图片失败: {}", e);
                        }
                    }
                }
            }
        } else {
            self.show_message("警告", "请先选择一个球队");
        }
    }

    fn edit_staff(&mut self, staff_idx: usize) {
        if let Some(staff) = self.staff_list.team_staff.get(staff_idx).cloned() {
            if let Err(e) = self.staff_edit_dialog.open(staff) {
                error!("打开员工编辑对话框失败: {}", e);
            }
        }
    }

    fn handle_staff_edit(&mut self, _ctx: &Context) {
        if self.staff_edit_dialog.confirmed {
            match self.staff_edit_dialog.get_updated_staff() {
                Ok(updated_staff) => {
                    match self.database.update_staff(&updated_staff) {
                        Ok(_) => {
                            // 刷新员工数据
                            match self.database.load_staff() {
                                Ok(staff) => {
                                    self.staff_list.set_all_staff(staff);
                                    if let Some(team_id) = self.team_list.get_selected_team_id() {
                                        self.staff_list.update_team_staff(team_id);
                                    }
                                    self.show_message("成功", &format!("已更新员工: {}", updated_staff.name));
                                    self.set_status(&format!("已更新员工: {}", updated_staff.name));
                                },
                                Err(e) => {
                                    error!("刷新员工数据失败: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            self.show_message("错误", &format!("更新员工失败: {}", e));
                            error!("更新员工失败: {}", e);
                        }
                    }
                },
                Err(e) => {
                    self.show_message("错误", &format!("获取更新后的员工数据失败: {}", e));
                    error!("获取更新后的员工数据失败: {}", e);
                }
            }
        }
    }

    fn export_team_list(&mut self) {
        if self.team_list.teams.is_empty() {
            self.show_message("警告", "没有可导出的数据");
            return;
        }

        let dialog = FileDialog::new()
            .add_filter("CSV文件", &["csv"])
            .add_filter("所有文件", &["*"])
            .show_save_single_file();
        
        if let Ok(Some(path)) = dialog {
            self.export_path = Some(path);
            self.confirm_dialog.show_confirm(
                "确认导出",
                &format!("确定要导出 {} 个球队数据吗？", self.team_list.teams.len())
            );
        }
    }

    fn handle_confirm_export(&mut self) {
        if self.confirm_dialog.confirmed {
            if let Some(path) = self.export_path.clone() {
                // 导出CSV
                let mut content = String::from("ID,球队名称,球队财富,成立年份,所在地区,支持者数量,主场名称,球队昵称,联赛ID\n");
                let teams_len = self.team_list.teams.len();
                
                for team in &self.team_list.teams {
                    content.push_str(&format!(
                        "{},{},{},{},{},{},{},{},{}\n",
                        team.id, team.name, team.wealth, team.found_year,
                        team.location, team.supporter_count, team.stadium_name,
                        team.nickname, team.league_id
                    ));
                }
                
                match std::fs::write(&path, content) {
                    Ok(_) => {
                        let path_str = path.display().to_string();
                        self.show_message(
                            "成功",
                            &format!("已导出 {} 个球队数据", teams_len)
                        );
                        self.set_status(&format!("已导出球队数据至: {}", path_str));
                    },
                    Err(e) => {
                        self.show_message("错误", &format!("导出失败: {}", e));
                        error!("导出球队列表失败: {}", e);
                    }
                }
                
                self.export_path = None;
            }
        }
    }

    fn open_bulk_edit(&mut self) {
        if !self.database.is_connected() {
            self.show_message("警告", "请先加载数据库");
            return;
        }

        if self.team_list.teams.is_empty() {
            self.show_message("警告", "没有可编辑的球队数据");
            return;
        }

        self.bulk_edit_dialog.open(self.team_list.teams.clone());
    }

    fn handle_bulk_edit(&mut self, ctx: &Context) {
        if self.bulk_edit_dialog.confirmed {
            let modified_teams = self.bulk_edit_dialog.get_modified_teams();
            
            if !modified_teams.is_empty() {
                match self.database.update_teams_batch(&modified_teams) {
                    Ok(count) => {
                        // 刷新数据
                        if let Err(e) = self.load_data(ctx) {
                            error!("刷新数据失败: {}", e);
                        }
                        
                        self.show_message("成功", &format!("已批量更新 {} 个球队", count));
                        self.set_status(&format!("已批量更新 {} 个球队", count));
                    },
                    Err(e) => {
                        self.show_message("错误", &format!("批量更新失败: {}", e));
                        error!("批量更新球队失败: {}", e);
                    }
                }
            }
        }
    }

    fn show_message(&mut self, title: &str, message: &str) {
        self.message_dialog.show_message(title, message);
    }

    fn set_status(&mut self, message: &str) {
        self.status_message = message.to_string();
        info!("状态: {}", message);
    }

    fn ui_top_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(5.0);
            if crate::ui::widgets::mac_button(ui, "加载数据库") {
                self.load_database(ctx);
            }
            
            if crate::ui::widgets::mac_primary_button(ui, "保存球队修改") {
                self.save_team_changes();
            }
            
            if crate::ui::widgets::mac_button(ui, "批量编辑") {
                self.open_bulk_edit();
            }
            
            // 自动保存开关
            let auto_save_text = if self.auto_save_enabled {
                format!("自动保存: 开 ({}秒)", self.auto_save_countdown)
            } else {
                "自动保存: 关".to_string()
            };
            
            if ui.button(auto_save_text).clicked() {
                self.toggle_auto_save();
            }
            
            ui.separator();
            
            ui.strong("状态:");
            ui.add_space(5.0);
            ui.label(&self.status_message);
            ui.add_space(5.0);
        });
    }

    fn ui_bottom_panel(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.strong("球队总数:");
            ui.add_space(2.0);
            ui.label(format!("{}", self.team_list.teams.len()));
            ui.separator();
            ui.label("CFS球队编辑器 BY.卡尔纳斯 | Rust版本 v1.0.0");
            ui.add_space(5.0);
        });
    }
}

impl App for TeamEditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // 更新自动保存计时器
        self.update_auto_save_timer();
        
        // 检查是否需要自动保存
        if self.auto_save_enabled && self.has_unsaved_changes && self.auto_save_countdown == 0 {
            self.auto_save(ctx);
        }
        
        // 处理对话框
        self.message_dialog.show(ctx);
        
        if self.confirm_dialog.show(ctx) {
            self.handle_confirm_save(ctx);
            self.handle_confirm_export();
        }
        
        if self.staff_edit_dialog.show(ctx) {
            self.handle_staff_edit(ctx);
        }
        
        if self.bulk_edit_dialog.show(ctx) {
            self.handle_bulk_edit(ctx);
        }
        
        // 顶部面板
        TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none()
                .fill(Color32::from_rgb(245, 245, 245))
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .shadow(egui::epaint::Shadow {
                    extrusion: 1.0,
                    color: Color32::from_black_alpha(20),
                }))
            .show(ctx, |ui| {
                self.ui_top_panel(ctx, ui);
            });
        
        // 底部状态栏
        TopBottomPanel::bottom("bottom_panel")
            .frame(egui::Frame::none()
                .fill(Color32::from_rgb(245, 245, 245))
                .inner_margin(egui::Margin::symmetric(10.0, 4.0)))
            .show(ctx, |ui| {
                self.ui_bottom_panel(ctx, ui);
            });
        
        // 左侧球队列表
        SidePanel::left("team_list_panel")
            .frame(egui::Frame::none()
                .fill(Color32::from_rgb(250, 250, 250))
                .inner_margin(egui::Margin::same(10.0)))
            .resizable(true)
            .default_width(250.0)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                if let Some(team_id) = self.team_list.ui(ui) {
                    self.select_team(team_id, ctx);
                }
                
                // 处理刷新和导出按钮
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if crate::ui::widgets::mac_button(ui, "刷新列表") {
                        if let Err(e) = self.load_data(ctx) {
                            self.show_message("错误", &format!("刷新数据失败: {}", e));
                            error!("刷新数据失败: {}", e);
                        } else {
                            self.set_status("列表已刷新");
                        }
                    }
                    
                    if crate::ui::widgets::mac_button(ui, "导出列表") {
                        self.export_team_list();
                    }
                });
            });
        
        // 主内容区
        CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(Color32::from_rgb(240, 240, 240))
                .inner_margin(egui::Margin::same(15.0)))
            .show(ctx, |ui| {
                // 添加标签页
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, ActiveTab::TeamDetails, "球队详情");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Visualization, "数据可视化");
                });
                
                ui.add_space(10.0);
                
                match self.active_tab {
                    ActiveTab::TeamDetails => {
                        widgets::rounded_frame(ui, |ui| {
                            // 球队详情
                            let (logo_clicked, field_changed) = self.team_details.ui(ui);
                            if logo_clicked {
                                self.replace_logo(ctx);
                            }
                            
                            // 如果有字段被修改，设置未保存更改标志
                            if field_changed {
                                self.has_unsaved_changes = true;
                                self.last_auto_save = Instant::now();
                            }
                            
                            ui.add_space(10.0);
                            widgets::horizontal_separator(ui);
                            ui.add_space(10.0);
                            
                            // 员工列表
                            if let Some(staff_idx) = self.staff_list.ui(ui) {
                                self.edit_staff(staff_idx);
                            }
                        });
                    },
                    ActiveTab::Visualization => {
                        widgets::rounded_frame(ui, |ui| {
                            // 数据可视化
                            self.visualization.ui(ui);
                        });
                    }
                }
            });
    }
} 