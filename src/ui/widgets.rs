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
        ui.label(label).on_hover_text(label);
        changed = ui.text_edit_singleline(value).changed();
    });
    changed
}

/// 创建只读表单行
pub fn readonly_form_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_space(5.0);
        ui.label(label).on_hover_text(label);
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
    
    // 绘制图表框架
    let (response, painter) = ui.allocate_painter(Vec2::new(width, height), egui::Sense::hover());
    let rect = response.rect;
    
    // 绘制背景
    painter.rect_filled(
        rect,
        Rounding::same(4.0),
        Color32::from_rgb(250, 250, 250)
    );
    
    // 绘制边框
    painter.rect_stroke(
        rect,
        Rounding::same(4.0),
        Stroke::new(1.0, Color32::from_rgb(200, 200, 200))
    );
    
    // 计算条形宽度和间距
    let bar_spacing = 10.0;
    let bar_width = (width - (bar_count as f32 + 1.0) * bar_spacing) / bar_count as f32;
    
    // 绘制Y轴刻度线
    let y_ticks = 5;
    for i in 0..=y_ticks {
        let y_pos = rect.min.y + rect.height() * (1.0 - i as f32 / y_ticks as f32);
        let tick_value = max_value * i / y_ticks;
        
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
    
    // 绘制条形
    for (i, &value) in values.iter().enumerate() {
        let bar_height = (value as f32 / max_value as f32) * (height - 30.0);
        let x = rect.min.x + bar_spacing + i as f32 * (bar_width + bar_spacing);
        let y = rect.max.y - 20.0 - bar_height;
        
        // 绘制条形
        painter.rect_filled(
            Rect::from_min_size(
                pos2(x, y),
                Vec2::new(bar_width, bar_height)
            ),
            Rounding::same(2.0),
            Color32::from_rgb(100, 150, 250)
        );
        
        // 绘制数值
        painter.text(
            pos2(x + bar_width / 2.0, y - 5.0),
            Align2::CENTER_BOTTOM,
            value.to_string(),
            egui::FontId::proportional(10.0),
            Color32::DARK_GRAY
        );
        
        // 绘制标签
        if i < labels.len() {
            painter.text(
                pos2(x + bar_width / 2.0, rect.max.y - 10.0),
                Align2::CENTER_BOTTOM,
                &labels[i],
                egui::FontId::proportional(10.0),
                Color32::DARK_GRAY
            );
        }
    }
}

/// 绘制简单的饼图
pub fn draw_pie_chart(ui: &mut Ui, values: &[i64], labels: &[String], title: &str, size: f32) {
    if values.is_empty() {
        return;
    }
    
    // 图表标题
    ui.heading(title);
    ui.add_space(5.0);
    
    let total: i64 = values.iter().sum();
    if total <= 0 {
        return;
    }
    
    // 分配绘图区域
    let (response, painter) = ui.allocate_painter(Vec2::new(size, size), egui::Sense::hover());
    let rect = response.rect;
    
    // 计算圆心和半径
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.4;
    
    // 定义颜色
    let colors = [
        Color32::from_rgb(100, 150, 250),
        Color32::from_rgb(250, 150, 100),
        Color32::from_rgb(100, 250, 150),
        Color32::from_rgb(250, 100, 150),
        Color32::from_rgb(150, 100, 250),
        Color32::from_rgb(150, 250, 100),
    ];
    
    // 绘制饼图
    let mut start_angle = 0.0;
    for (i, &value) in values.iter().enumerate() {
        let angle = 2.0 * std::f32::consts::PI * (value as f32 / total as f32);
        let color = colors[i % colors.len()];
        
        // 绘制扇形
        let mut points = Vec::new();
        points.push(center);
        
        // 添加弧线上的点
        let steps = 32;
        for j in 0..=steps {
            let a = start_angle + angle * (j as f32 / steps as f32);
            let x = center.x + radius * a.cos();
            let y = center.y + radius * a.sin();
            points.push(pos2(x, y));
        }
        
        // 创建路径形状
        let path = PathShape {
            points,
            closed: true,
            fill: color,
            stroke: Stroke::new(1.0, Color32::WHITE),
        };
        
        painter.add(egui::Shape::Path(path));
        
        // 绘制标签线和文本
        let mid_angle = start_angle + angle / 2.0;
        let text_distance = radius * 1.3;
        let text_pos = pos2(
            center.x + text_distance * mid_angle.cos(),
            center.y + text_distance * mid_angle.sin()
        );
        
        // 绘制连接线
        let line_end = pos2(
            center.x + radius * 1.1 * mid_angle.cos(),
            center.y + radius * 1.1 * mid_angle.sin()
        );
        painter.line_segment(
            [line_end, text_pos],
            Stroke::new(1.0, Color32::DARK_GRAY)
        );
        
        // 绘制标签和百分比
        if i < labels.len() {
            let percentage = (value as f32 / total as f32) * 100.0;
            let text = format!("{}: {:.1}%", labels[i], percentage);
            painter.text(
                text_pos,
                Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(10.0),
                Color32::DARK_GRAY
            );
        }
        
        start_angle += angle;
    }
} 