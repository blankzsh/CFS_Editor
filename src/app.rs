use std::path::PathBuf;
use std::time::{Duration, Instant};

use egui::{Context, CentralPanel, SidePanel, TopBottomPanel, Ui, Color32, Layout, Align};
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
    // SponsorEditor已从实际功能中移除，但UI保留
    SponsorEditor,
}

impl ActiveTab {
    fn as_str(&self) -> &'static str {
        match self {
            ActiveTab::TeamDetails => "球队详情",
            ActiveTab::Visualization => "数据可视化",
            ActiveTab::SponsorEditor => "杂项编辑器",
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
        if self.database.is_connected() {
            self.show_message("警告", "已经连接到数据库");
            return;
        }
        
        // 使用native-dialog库打开文件对话框
        let dialog = FileDialog::new()
            .add_filter("SQLite数据库", &["db", "sqlite", "sqlite3"])
            .add_filter("所有文件", &["*"])
            .show_open_single_file();
        
        if let Ok(Some(path)) = dialog {
            match self.database.connect(&path) {
                Ok(_) => {
                    let path_str = path.display().to_string();
                    self.show_message("成功", &format!("已连接到数据库: {}", path_str));
                    self.set_status(&format!("已连接到数据库: {}", path_str));
                    
                    // 加载数据
                    if let Err(e) = self.load_data(ctx) {
                        self.show_message("错误", &format!("加载数据失败: {}", e));
                        error!("加载数据失败: {}", e);
                    }
                },
                Err(e) => {
                    self.show_message("错误", &format!("连接数据库失败: {}", e));
                    error!("连接数据库失败: {}", e);
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

        // 使用native-dialog库打开文件对话框
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
            // 文件菜单
            ui.menu_button("文件", |ui| {
                if ui.button("加载数据库").clicked() {
                    ui.close_menu();
                self.load_database(ctx);
            }
                
                if ui.button("关闭数据库").clicked() {
                    ui.close_menu();
                    if let Err(e) = self.database.close() {
                        self.show_message("错误", &format!("关闭数据库失败: {}", e));
                        error!("关闭数据库失败: {}", e);
                    } else {
                        self.set_status("数据库已关闭");
                    }
                }
                
                ui.separator();
                
                if ui.button("导出球队列表").clicked() {
                    ui.close_menu();
                    self.export_team_list();
                }
                
                ui.separator();
                
                if ui.button("退出").clicked() {
                    ui.close_menu();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            
            // 编辑菜单
            ui.menu_button("编辑", |ui| {
                if ui.button("保存球队修改").clicked() {
                    ui.close_menu();
                self.save_team_changes();
            }
            
                if ui.button("批量编辑").clicked() {
                    ui.close_menu();
                self.open_bulk_edit();
            }
            
                ui.separator();
                
            let auto_save_text = if self.auto_save_enabled {
                    "禁用自动保存"
            } else {
                    "启用自动保存"
            };
            
            if ui.button(auto_save_text).clicked() {
                    ui.close_menu();
                self.toggle_auto_save();
            }
            });
            
            // 视图菜单
            ui.menu_button("视图", |ui| {
                if ui.selectable_label(self.active_tab == ActiveTab::TeamDetails, "球队详情").clicked() {
                    ui.close_menu();
                    self.active_tab = ActiveTab::TeamDetails;
                }
                
                if ui.selectable_label(self.active_tab == ActiveTab::Visualization, "数据可视化").clicked() {
                    ui.close_menu();
                    self.active_tab = ActiveTab::Visualization;
                }
                
                if ui.selectable_label(self.active_tab == ActiveTab::SponsorEditor, "杂项编辑器").clicked() {
                    ui.close_menu();
                    self.active_tab = ActiveTab::SponsorEditor;
                }
            });
            
            // 帮助菜单
            ui.menu_button("帮助", |ui| {
                if ui.button("关于").clicked() {
                    ui.close_menu();
                    self.show_message(
                        "关于",
                        "CFS球队编辑器 v0.1.0\n作者: 卡尔纳斯\n\n用于编辑和管理CFS游戏的球队数据。"
                    );
                }
            });
            
            // 显示当前标签页
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(self.active_tab.as_str());
            });
        });
    }

    fn ui_bottom_panel(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.strong("状态:");
            ui.add_space(5.0);
            ui.label(&self.status_message);
            
            // 显示自动保存状态
            if self.auto_save_enabled {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(format!("自动保存: {}秒", self.auto_save_countdown));
                });
            }
        });
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
            let dialog = FileDialog::new()
                .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp"])
                .show_open_single_file();
            
            if let Ok(Some(path)) = dialog {
                if let Some(db_dir) = self.database.get_db_directory() {
                    // 创建logos目录（如果不存在）
                    let logos_dir = db_dir.join("logos");
                    if !logos_dir.exists() {
                        if let Err(e) = std::fs::create_dir_all(&logos_dir) {
                            self.show_message("错误", &format!("创建logos目录失败: {}", e));
                            error!("创建logos目录失败: {}", e);
                            return;
                        }
                    }
                    
                    // 保存Logo
                    let target_path = logos_dir.join(format!("{}.png", team_id));
                    if let Err(e) = utils::save_image_as_png(&path, &target_path, 256, 256) {
                        self.show_message("错误", &format!("保存Logo失败: {}", e));
                        error!("保存Logo失败: {}", e);
                        return;
                    }
                    
                    // 重新加载Logo
                    if let Err(e) = self.team_details.load_logo(ctx, &db_dir, team_id) {
                        self.show_message("错误", &format!("加载Logo失败: {}", e));
                        error!("加载Logo失败: {}", e);
                        return;
                    }
                    
                    self.set_status("Logo已替换");
                }
            }
        } else {
            self.show_message("警告", "请先选择一个球队");
        }
    }
}

impl App for TeamEditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
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
        
        // 自动保存
        self.update_auto_save_timer();
        if self.auto_save_countdown == 0 {
            self.auto_save(ctx);
        }
        
        // 顶部面板
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
                self.ui_top_panel(ctx, ui);
            });
        
        // 底部状态栏
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
                self.ui_bottom_panel(ctx, ui);
            });
        
        // 左侧面板 - 球队列表
        SidePanel::left("team_list_panel")
            .resizable(true)
            .min_width(200.0)
            .default_width(250.0)
            .show(ctx, |ui| {
                if let Some(team_id) = self.team_list.ui(ui) {
                    self.select_team(team_id, ctx);
                }
            });
        
        // 右侧面板 - 员工列表
        SidePanel::right("staff_list_panel")
            .resizable(true)
            .min_width(200.0)
            .default_width(250.0)
            .show(ctx, |ui| {
                if let Some(staff_idx) = self.staff_list.ui(ui) {
                    self.edit_staff(staff_idx);
                }
            });
        
        // 中央面板
        CentralPanel::default().show(ctx, |ui| {
            // 选项卡
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, ActiveTab::TeamDetails, "球队详情");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Visualization, "数据可视化");
                ui.selectable_value(&mut self.active_tab, ActiveTab::SponsorEditor, "杂项编辑器");
                });
                
            ui.separator();
                
            // 根据当前选项卡显示不同内容
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
                        });
                    },
                    ActiveTab::Visualization => {
                        widgets::rounded_frame(ui, |ui| {
                            // 数据可视化
                            self.visualization.ui(ui);
                        });
                },
                ActiveTab::SponsorEditor => {
                    widgets::rounded_frame(ui, |ui| {
                        // 显示提示信息，而不是实际的赞助商编辑器
                        ui.vertical_centered(|ui| {
                            ui.add_space(50.0);
                            ui.heading("杂项编辑器功能暂时不可用");
                            ui.add_space(20.0);
                            ui.label("该功能正在维护中，请稍后再试。");
                            ui.add_space(50.0);
                        });
                    });
                }
                }
            });
    }
} 