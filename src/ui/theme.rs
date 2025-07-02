use egui::{
    Color32, Context, FontFamily, FontId, Rounding, Stroke, TextStyle, Visuals,
    style::{Selection, Widgets, WidgetVisuals},
};

/// Mac风格的UI主题
pub fn setup_mac_theme(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    
    // 字体设置
    style.text_styles = [
        (TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(14.0, FontFamily::Proportional)),
    ].into();
    
    // 间距设置
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.menu_margin = egui::Margin::same(8.0);
    style.spacing.indent = 20.0;
    
    // 视觉效果
    let mut visuals = Visuals::light();
    
    // 背景色
    visuals.override_text_color = Some(Color32::from_rgb(50, 50, 50));
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(240, 240, 240);
    visuals.extreme_bg_color = Color32::from_rgb(240, 240, 240);
    visuals.faint_bg_color = Color32::from_rgb(230, 230, 230);
    
    // 窗口圆角
    visuals.window_rounding = Rounding::same(8.0);
    visuals.menu_rounding = Rounding::same(6.0);
    
    // 按钮样式
    let button_visuals = WidgetVisuals {
        bg_fill: Color32::from_rgb(230, 230, 230),
        weak_bg_fill: Color32::from_rgb(220, 220, 220),
        bg_stroke: Stroke::new(1.0, Color32::from_rgb(200, 200, 200)),
        rounding: Rounding::same(6.0),
        fg_stroke: Stroke::new(1.0, Color32::from_rgb(50, 50, 50)),
        expansion: 0.0,
    };
    
    // 激活按钮样式
    let active_button_visuals = WidgetVisuals {
        bg_fill: Color32::from_rgb(80, 145, 245),
        weak_bg_fill: Color32::from_rgb(100, 160, 255),
        bg_stroke: Stroke::new(1.0, Color32::from_rgb(70, 130, 220)),
        rounding: Rounding::same(6.0),
        fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
        expansion: 1.0,
    };
    
    // 应用按钮样式
    visuals.widgets.inactive = button_visuals.clone();
    visuals.widgets.hovered = button_visuals.clone();
    visuals.widgets.active = active_button_visuals;
    visuals.widgets.open = button_visuals;
    
    // 选择样式
    visuals.selection = Selection {
        bg_fill: Color32::from_rgb(180, 200, 255),
        stroke: Stroke::new(1.0, Color32::from_rgb(80, 145, 245)),
    };
    
    // 应用主题
    style.visuals = visuals;
    ctx.set_style(style);
} 