#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 在发布模式下隐藏控制台窗口

mod app;
mod data;
mod error;
mod ui;
mod utils;

use app::TeamEditorApp;
use log::info;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // 配置日志
    env_logger::init();
    info!("CFS球队编辑器启动");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 800.0])
            .with_min_inner_size([800.0, 500.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "CFS球队编辑器 BY.卡尔纳斯",
        options,
        Box::new(|cc| {
            // 加载中文字体
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(TeamEditorApp::new(cc))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // 创建字体
    let mut fonts = egui::FontDefinitions::default();
    
    // 尝试从系统加载中文字体
    let font_paths = [
        // Windows 系统字体
        "C:/Windows/Fonts/msyh.ttc",      // 微软雅黑
        "C:/Windows/Fonts/simhei.ttf",    // 黑体
        "C:/Windows/Fonts/simsun.ttc",    // 宋体
        // Linux 系统字体
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        // macOS 系统字体
        "/System/Library/Fonts/PingFang.ttc",
    ];
    
    // 尝试加载字体
    for path in &font_paths {
        let path = PathBuf::from(path);
        if path.exists() {
            match std::fs::read(&path) {
                Ok(font_data) => {
                    // 添加字体
                    fonts.font_data.insert(
                        "chinese_font".to_owned(),
                        egui::FontData::from_owned(font_data),
                    );
                    
                    // 将中文字体设置为首选字体
                    fonts.families
                        .get_mut(&egui::FontFamily::Proportional)
                        .unwrap()
                        .insert(0, "chinese_font".to_owned());
                    
                    // 同时也支持等宽字体
                    fonts.families
                        .get_mut(&egui::FontFamily::Monospace)
                        .unwrap()
                        .push("chinese_font".to_owned());
                    
                    info!("已加载中文字体: {}", path.display());
                    break;
                }
                Err(err) => {
                    eprintln!("无法读取字体文件 {}: {}", path.display(), err);
                }
            }
        }
    }
    
    // 应用字体
    ctx.set_fonts(fonts);
}
