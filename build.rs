#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/favicon.ico"); // 使用已存在的图标文件
    res.set_language(0x0004); // 中文简体
    
    // 设置版本信息
    res.set("FileVersion", "1.0.0.0");
    res.set("ProductVersion", "1.0.0.0");
    res.set("FileDescription", "CFS球队编辑器");
    res.set("ProductName", "CFS球队编辑器");
    res.set("CompanyName", "卡尔纳斯");
    
    // 编译资源
    if let Err(e) = res.compile() {
        eprintln!("无法编译资源: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {
    // 非Windows平台不需要做任何事情
} 