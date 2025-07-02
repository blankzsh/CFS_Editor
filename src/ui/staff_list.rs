use egui::{Color32, RichText, Ui, Stroke, Rounding};
use egui_extras::{Column, TableBuilder};

use crate::data::staff::Staff;
use crate::ui::widgets;

pub struct StaffListView {
    pub all_staff: Vec<Staff>,
    pub team_staff: Vec<Staff>,
    pub selected_index: Option<usize>,
}

impl StaffListView {
    pub fn new() -> Self {
        StaffListView {
            all_staff: Vec::new(),
            team_staff: Vec::new(),
            selected_index: None,
        }
    }

    pub fn set_all_staff(&mut self, staff: Vec<Staff>) {
        self.all_staff = staff;
    }

    pub fn update_team_staff(&mut self, team_id: i64) {
        self.team_staff = self.all_staff
            .iter()
            .filter(|s| s.team_id == team_id)
            .cloned()
            .collect();
        self.selected_index = None;
    }

    pub fn get_selected_staff(&self) -> Option<&Staff> {
        self.selected_index.and_then(|idx| self.team_staff.get(idx))
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<usize> {
        let mut selected_staff_idx = None;

        widgets::titled_frame("员工信息", ui, |ui| {
            if self.team_staff.is_empty() {
                ui.add_space(10.0);
                ui.label("该球队没有员工");
                ui.add_space(10.0);
            } else {
                // Mac风格的表格容器
                egui::Frame::none()
                    .fill(Color32::from_rgb(255, 255, 255))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
                    .rounding(Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(8.0))
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto().at_least(50.0))  // ID
                            .column(Column::remainder().at_least(120.0))  // 姓名
                            .column(Column::auto().at_least(60.0))  // 能力值
                            .column(Column::auto().at_least(60.0))  // 知名度
                            .header(24.0, |mut header| {
                                header.col(|ui| { 
                                    ui.strong("ID"); 
                                });
                                header.col(|ui| { 
                                    ui.strong("姓名"); 
                                });
                                header.col(|ui| { 
                                    ui.strong("能力值"); 
                                });
                                header.col(|ui| { 
                                    ui.strong("知名度"); 
                                });
                            })
                            .body(|mut body| {
                                for (idx, staff) in self.team_staff.iter().enumerate() {
                                    let is_selected = Some(idx) == self.selected_index;
                                    let row_height = 28.0;
                                    
                                    body.row(row_height, |mut row| {
                                        let ability = match staff.get_ability() {
                                            Ok(a) => a.to_string(),
                                            Err(_) => "错误".to_string(),
                                        };
                                        
                                        row.col(|ui| {
                                            ui.label(staff.id.to_string());
                                        });
                                        row.col(|ui| {
                                            let mut text = RichText::new(&staff.name);
                                            
                                            if is_selected {
                                                text = text.strong().color(Color32::from_rgb(50, 100, 200));
                                            }
                                            
                                            if ui.selectable_label(is_selected, text).clicked() {
                                                self.selected_index = Some(idx);
                                                selected_staff_idx = Some(idx);
                                            }
                                        });
                                        row.col(|ui| {
                                            ui.label(ability);
                                        });
                                        row.col(|ui| {
                                            ui.label(staff.fame.to_string());
                                        });
                                    });
                                }
                            });
                    });

                ui.add_space(8.0);
                ui.small("双击员工记录可编辑");
            }
        });

        selected_staff_idx
    }
} 