# CFS球队编辑器 - Rust版本

这是使用Rust语言重构的CFS球队编辑器，原始版本是使用Python和PySide6开发的。

## 功能特性

- 加载和编辑CFS数据库中的球队信息
- 编辑球队基本信息（名称、财富、成立年份等）
- 查看和编辑球队员工信息
- 更换球队Logo
- 导出球队列表到CSV文件

## 技术栈

- Rust 编程语言
- egui/eframe 用于GUI界面
- rusqlite 用于SQLite数据库操作
- image 用于图像处理
- serde/serde_json 用于JSON处理

## 构建与运行

### 依赖项

- Rust 和 Cargo (推荐使用 [rustup](https://rustup.rs/) 安装)

### 编译与运行

```bash
# 克隆仓库
git clone https://github.com/yourusername/rust-cfs.git
cd rust-cfs

# 编译
cargo build --release

# 运行
cargo run --release
```

## 使用方法

1. 点击"加载数据库"按钮选择CFS数据库文件（.db）
2. 在左侧列表中选择要编辑的球队
3. 在右侧面板中编辑球队信息
4. 点击保存按钮保存修改
5. 双击员工记录可以编辑员工信息
6. 点击Logo区域可以更换球队Logo

## 作者

- 原作者: 卡尔纳斯
- Rust版本: 卡尔纳斯

## 许可证

本项目采用 MIT 许可证 