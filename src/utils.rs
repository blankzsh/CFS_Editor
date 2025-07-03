use std::path::{Path, PathBuf};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use log::error;

use crate::error::Result;

/// 加载图像并调整大小
pub fn load_and_resize_image(path: &Path, width: u32, height: u32) -> Result<DynamicImage> {
    let img = image::open(path)?;
    let resized = img.resize(width, height, image::imageops::FilterType::Triangle);
    Ok(resized)
}

/// 保存图像到文件
pub fn save_image(img: &DynamicImage, path: &Path) -> Result<()> {
    img.save(path)?;
    Ok(())
}

/// 将图像转换为RGBA8格式的字节数组，用于egui显示
pub fn image_to_rgba8_bytes(img: &DynamicImage) -> Vec<u8> {
    let rgba8 = img.to_rgba8();
    rgba8.into_raw()
}

/// 验证数字输入
pub fn validate_number(input: &str) -> Result<i64> {
    match input.trim().parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(crate::error::AppError::InvalidInput(
            format!("无效的数字输入: {}", input)
        )),
    }
}

/// 创建球队Logo的文件路径
pub fn create_logo_path(db_dir: &Path, team_id: i64) -> PathBuf {
    db_dir.join(format!("L{}.png", team_id))
}

/// 检查文件是否存在
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// 将图像保存为PNG格式，并调整大小
pub fn save_image_as_png(src_path: &Path, dst_path: &Path, width: u32, height: u32) -> Result<()> {
    // 加载图像
    let img = image::open(src_path)?;
    
    // 调整大小
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    
    // 保存为PNG
    resized.save(dst_path)?;
    
    Ok(())
} 