use egui::{Color32, Frame, Rounding, Stroke, Ui, Vec2, Rect, Align2, pos2, epaint::PathShape};

/// 创建带有标题的分组框
pub fn titled_frame(title: &str, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.group(|ui| {
        ui.heading(title);
        ui.add_space(2.0);
        ui.separator();
        ui.add_space(4.0);
        add_contents(ui);
    });
}

/// 创建带有圆角和阴影的面板（Mac风格）
pub fn rounded_frame(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(Color32::from_rgb(250, 250, 250))
        .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
        .rounding(Rounding::same(8.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_black_alpha(15),
        })
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(10.0, 10.0);
            add_contents(ui);
        });
}

/// 创建Mac风格的卡片
pub fn mac_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(Color32::from_rgb(255, 255, 255))
        .stroke(Stroke::new(1.0, Color32::from_rgb(230, 230, 230)))
        .rounding(Rounding::same(6.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 2.0,
            color: Color32::from_black_alpha(10),
        })
        .inner_margin(egui::Margin::same(10.0))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            add_contents(ui);
        });
}

/// 创建水平分隔线
pub fn horizontal_separator(ui: &mut Ui) {
    ui.add_space(2.0);
    ui.separator();
    ui.add_space(2.0);
}

/// 创建标签和值的显示行
pub fn label_value(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.strong(label);
        ui.add_space(5.0);
        ui.label(value);
    });
}

/// 创建表单行，带有标签和编辑框
pub fn form_row(ui: &mut Ui, label: &str, value: &mut String) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.add_space(5.0);
        ui.label(label);
        changed = ui.text_edit_singleline(value).changed();
    });
    changed
}

/// 创建只读表单行
pub fn readonly_form_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_space(5.0);
        ui.label(label);
        ui.add_enabled(false, egui::TextEdit::singleline(&mut value.to_string()));
    });
}

/// 创建错误消息显示
pub fn error_message(ui: &mut Ui, message: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        ui.label("⚠").on_hover_text(message);
        ui.colored_label(Color32::from_rgb(200, 0, 0), message);
    });
}

/// 创建Mac风格按钮
pub fn mac_button(ui: &mut Ui, text: &str) -> bool {
    ui.add(egui::Button::new(text)
        .min_size(Vec2::new(80.0, 24.0))
        .rounding(Rounding::same(6.0)))
    .clicked()
}

/// 创建Mac风格主按钮（蓝色）
pub fn mac_primary_button(ui: &mut Ui, text: &str) -> bool {
    let response = ui.add(egui::Button::new(text)
        .min_size(Vec2::new(80.0, 24.0))
        .rounding(Rounding::same(6.0))
        .fill(Color32::from_rgb(80, 145, 245)));
    
    response.clicked()
}

/// 创建Mac风格标题栏
pub fn mac_title_bar(ui: &mut Ui, title: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.heading(title);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
        });
    });
}

/// 绘制简单的条形图
pub fn draw_bar_chart(ui: &mut Ui, values: &[i64], labels: &[String], title: &str, max_height: f32) {
    let max_value = *values.iter().max().unwrap_or(&1);
    let width = ui.available_width();
    let height = max_height;
    let bar_count = values.len();
    
    if bar_count == 0 {
        return;
    }
    
    // 图表标题
    ui.heading(title);
    ui.add_space(5.0);
    
    // 为X轴标签预留更多空间
    let x_axis_height = if bar_count > 8 { 60.0 } else { 30.0 };
    let chart_height = height - x_axis_height;
    
    // 绘制图表框架
    let (response, painter) = ui.allocate_painter(Vec2::new(width, chart_height), egui::Sense::hover());
    let rect = response.rect;
    
    // 绘制背景
    painter.rect_filled(
        rect,
        Rounding::same(6.0),
        Color32::from_rgb(250, 250, 250)
    );
    
    // 绘制边框
    painter.rect_stroke(
        rect,
        Rounding::same(6.0),
        Stroke::new(1.0, Color32::from_rgb(220, 220, 220))
    );
    
    // 计算条形宽度和间距 - 调整间距以适应更多条形
    let bar_spacing = if bar_count > 10 { 5.0 } else { 10.0 };
    let bar_width = (width - (bar_count as f32 + 1.0) * bar_spacing) / bar_count as f32;
    
    // 计算Y轴刻度的最大值（向上取整到合适的数值）
    let max_display_value = if max_value < 10 {
        max_value + 1
    } else if max_value < 100 {
        ((max_value + 9) / 10) * 10
    } else if max_value < 1000 {
        ((max_value + 99) / 100) * 100
    } else {
        ((max_value + 999) / 1000) * 1000
    };
    
    // 绘制Y轴刻度线
    let y_ticks = 5;
    for i in 0..=y_ticks {
        let y_pos = rect.min.y + rect.height() * (1.0 - i as f32 / y_ticks as f32);
        let tick_value = max_display_value * i / y_ticks;
        
        // 绘制水平辅助线
        painter.line_segment(
            [pos2(rect.min.x, y_pos), pos2(rect.max.x, y_pos)],
            Stroke::new(0.5, Color32::from_rgb(220, 220, 220))
        );
        
        // 绘制刻度值
        painter.text(
            pos2(rect.min.x + 5.0, y_pos - 10.0),
            Align2::LEFT_CENTER,
            format!("{}", tick_value),
            egui::FontId::proportional(10.0),
            Color32::DARK_GRAY
        );
    }
    
    // 定义条形图颜色
    let colors = [
        Color32::from_rgb(100, 150, 250), // 蓝色
        Color32::from_rgb(250, 150, 100), // 橙色
        Color32::from_rgb(100, 250, 150), // 绿色
        Color32::from_rgb(250, 100, 150), // 粉色
        Color32::from_rgb(150, 100, 250), // 紫色
        Color32::from_rgb(150, 250, 100), // 黄绿色
    ];
    
    // 绘制条形
    for (i, &value) in values.iter().enumerate() {
        // 计算条形高度（相对于max_display_value而非max_value）
        let bar_height = (value as f32 / max_display_value as f32) * (rect.height() - 20.0);
        let x = rect.min.x + bar_spacing + i as f32 * (bar_width + bar_spacing);
        let y = rect.max.y - bar_height;
        
        // 选择颜色
        let color = colors[i % colors.len()];
        
        // 绘制条形
        painter.rect_filled(
            Rect::from_min_size(
                pos2(x, y),
                Vec2::new(bar_width, bar_height)
            ),
            Rounding::same(4.0),
            color
        );
        
        // 绘制条形边框
        painter.rect_stroke(
            Rect::from_min_size(
                pos2(x, y),
                Vec2::new(bar_width, bar_height)
            ),
            Rounding::same(4.0),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 100))
        );
        
        // 绘制数值（只有当值足够大时才显示）
        if bar_height > 20.0 {
            // 绘制白色背景确保文字清晰可见
            let font_id = egui::FontId::proportional(10.0);
            let text = value.to_string();
            let galley = painter.layout_no_wrap(
                text.clone(),
                font_id.clone(),
                Color32::DARK_GRAY
            );
            let text_pos = pos2(x + bar_width / 2.0, y - 5.0);
            let text_rect = Rect::from_center_size(
                text_pos,
                galley.size() + Vec2::new(6.0, 4.0)
            );
            painter.rect_filled(
                text_rect,
                Rounding::same(2.0),
                Color32::from_rgba_unmultiplied(255, 255, 255, 220)
            );
            
            // 绘制数值文本
            painter.text(
                text_pos,
                Align2::CENTER_BOTTOM,
                text,
                font_id,
                Color32::DARK_GRAY
            );
        }
    }
    
    // 在图表下方绘制X轴标签
    ui.add_space(5.0);
    
    // 为标签创建一个新的绘图区域
    let (label_response, label_painter) = ui.allocate_painter(Vec2::new(width, x_axis_height), egui::Sense::hover());
    let label_rect = label_response.rect;
    
    // 绘制标签
    for (i, label) in labels.iter().enumerate() {
        let x = rect.min.x + bar_spacing + i as f32 * (bar_width + bar_spacing) + bar_width / 2.0;
        let y = label_rect.min.y + 5.0;
        
        // 如果标签太多，需要旋转显示
        if bar_count > 8 {
            // 创建旋转标签
            let font_id = egui::FontId::proportional(9.0);
            
            // 计算旋转角度（45度）
            let angle = std::f32::consts::PI / 4.0;
            
            // 绘制旋转文本
            // 注意：egui不直接支持文本旋转，所以我们使用倾斜的方式来模拟
                
            label_painter.text(
                pos2(x, y),
                Align2::LEFT_TOP,
                label,
                font_id,
                Color32::DARK_GRAY
            );
        } else {
            // 正常显示标签
            label_painter.text(
                pos2(x, y),
                Align2::CENTER_TOP,
                label,
                egui::FontId::proportional(10.0),
                Color32::DARK_GRAY
            );
        }
    }
}

/// 绘制简单的饼图（已废弃，请使用visualization.rs中的实现）
pub fn draw_pie_chart(ui: &mut Ui, values: &[i64], labels: &[String], title: &str, size: f32) {
    ui.heading(title);
    ui.label("请使用visualization.rs中的实现");
} 