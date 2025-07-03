# CFS 球队编辑器 (Rust-CFS)

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

**CFS 球队编辑器** 是一款使用 Rust 和 `egui` 构建的高性能桌面应用程序，旨在提供一个功能丰富、体验流畅的 CFS（中国足球模拟器）数据库编辑工具。本项目是原 Python 版本的重构版，在保留核心功能的基础上，带来了现代化的 UI 和更强大的编辑能力。

## ✨ 功能特性

- **数据管理**:
  - ✅ **加载与编辑**: 轻松加载 CFS 数据库文件（`.db`），并对球队信息进行实时编辑。
  - ✅ **信息修改**: 编辑球队名称、财富、成立年份、声望等核心数据。
  - ✅ **员工管理**: 查看、编辑、甚至添加和删除球队的员工信息。
  - ✅ **Logo 更换**: 支持点击更换球队的 Logo，支持多种图片格式。
  - ✅ **CSV 导出**: 将球队列表一键导出为 CSV 文件，方便在其他程序中使用。
- **现代化 UI**:
  - ✨ **Mac 风格界面**: 简洁、现代的 UI 设计，提供卓越的视觉和交互体验。
  - ✨ **三栏布局**: 优化的"球队列表 - 主编辑器 - 员工列表"三栏布局，信息结构清晰，操作高效。
  - ✨ **响应式设计**: 界面元素可根据窗口大小动态调整，确保在不同分辨率下内容都能完整显示。
- **高级功能**:
  - 🚀 **批量编辑**: 选中多个球队，一次性修改他们的共同属性（如所在联赛、财富等）。
  - 🚀 **智能搜索**: 根据球队名称、地区、联赛等多种条件快速筛选球队。
  - 🚀 **数据可视化**: 内置图表工具，直观展示球队财富、支持者数量等数据的分布情况。
  - 🚀 **自动保存**: 可选的自动保存功能，防止意外关闭导致数据丢失。

## 📸 应用截图


| 主界面 | 批量编辑 | 数据可视化 |
| :---: | :---: | :---: |
| ![主界面截图](https://pic1.imgdb.cn/item/686637a158cb8da5c88bab32.png) | ![批量编辑截图](https://pic1.imgdb.cn/item/686637d458cb8da5c88bab57.png) | ![数据可视化截图](https://pic1.imgdb.cn/item/686637fc58cb8da5c88bab75.png) |


## 🛠️ 技术栈

本项目采用以下技术构建：

- **核心语言**: [Rust](https://www.rust-lang.org/) (2021 Edition)
- **GUI 框架**: [`egui`](https://github.com/emilk/egui) / [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe)
- **数据库**: [`rusqlite`](https://github.com/rusqlite/rusqlite) (捆绑 SQLite)
- **图像处理**: [`image`](https://github.com/image-rs/image)
- **序列化**: [`serde`](https://serde.rs/) / [`serde_json`](https://github.com/serde-rs/json)
- **原生对话框**: [`native-dialog`](https://github.com/mgdm/native-dialog-rs)
- **错误处理**: [`anyhow`](https://github.com/dtolnay/anyhow) / [`thiserror`](https://github.com/dtolnay/thiserror)
- **日志**: [`log`](https://github.com/rust-lang/log) / [`env_logger`](https://github.com/rust-cli/env_logger)
- **其他**: `chrono`, `mime_guess`

## 🚀 构建与运行

在开始之前，请确保您已经安装了 [Rust 环境](https://rustup.rs/)。

```bash
# 1. 克隆仓库
git clone https://github.com/blankzsh/CFS_Editor.git
cd rust-cfs

# 2. 构建项目 (推荐使用 release 模式以获得最佳性能)
cargo build --release

# 3. 运行应用
cargo run --release
```

## 📖 使用指南

1.  启动应用程序后，点击 **"加载数据库"** 按钮，选择您的 CFS 数据库文件 (`.db`)。
2.  数据加载后，左侧面板会显示所有球队的列表。您可以使用顶部的搜索框进行快速过滤。
3.  在左侧列表中选择一个球队，中央面板将显示其详细信息供您编辑。
4.  右侧面板会展示该球队的员工列表。双击员工条目可进行编辑。
5.  在中央面板中点击 Logo 区域，可以从本地选择新的图片文件来更换球队 Logo。
6.  所有修改在输入时即时生效，您也可以通过顶部的 **"保存"** 按钮手动保存。
7.  若要进行批量编辑，点击 **"批量编辑"** 按钮，在弹出的窗口中选择多个球队并应用修改。
8.  切换到 **"数据可视化"** 标签页，可以查看基于当前数据的统计图表。

## 🤝 贡献指南

我们非常欢迎各种形式的贡献！如果您希望为本项目做出贡献，请遵循以下步骤：

1.  **Fork** 本仓库。
2.  创建一个新的分支 (`git checkout -b feature/YourAmazingFeature`)。
3.  提交您的代码修改 (`git commit -m 'Add some AmazingFeature'`)。
4.  将您的分支推送到远程仓库 (`git push origin feature/YourAmazingFeature`)。
5.  创建一个 **Pull Request**。

如果您发现了 Bug 或有功能建议，请随时提交 [Issues](https://github.com/blankzsh/CFS_Editor/issues)。

## ⚖️ 行为准则

为了营造一个开放和友好的环境，我们采纳并期望项目参与者遵守 [贡献者契约行为准则](https://www.contributor-covenant.org/zh-cn/version/2/1/code_of_conduct/)。

## 👤 作者

- **卡尔纳斯** - *Rust版本开发者*

## 📜 许可证

本项目基于 [MIT 许可证](https://choosealicense.com/licenses/mit/) 发布。详情请见 `LICENSE` 文件。 