# Windows 电源管理器

一个用于管理 Windows 系统电源状态的 Rust 库，支持睡眠、休眠和防止系统休眠等功能。

## 主要特性

- 🔒 防止系统睡眠和显示器超时
- ⏰ 定时系统睡眠（带倒计时）
- 💤 强制系统睡眠或休眠
- 🔄 恢复默认电源设置
- ⚡ 集成底层 Windows 电源管理 API

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
windows-awake = "0.1.0"
```

## 快速开始

```rust
use windows_awake::PowerManager;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let power_manager = PowerManager::new();

    // 保持系统唤醒 30 分钟
    power_manager.keep_awake_for_minutes(30)?;

    // 或者无限期保持系统唤醒
    power_manager.keep_awake_indefinite()?;

    // 立即使系统进入睡眠状态
    power_manager.force_sleep(false, false, false)?;

    Ok(())
}
```

## 使用示例

### 保持系统唤醒

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// 无限期防止系统睡眠
power_manager.keep_awake_indefinite()?;

// 在指定时间内保持系统唤醒（以分钟为单位）
power_manager.keep_awake_for_minutes(60)?; // 保持唤醒1小时
```

### 控制系统睡眠状态

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// 普通睡眠模式
power_manager.force_sleep(false, false, false)?;

// 休眠模式
power_manager.force_sleep(true, false, false)?;

// 强制睡眠（忽略其他应用程序的阻止）
power_manager.force_sleep(false, true, true)?;
```

### 恢复默认设置

```rust
use windows_awake::PowerManager;

let power_manager = PowerManager::new();

// 恢复系统默认电源设置
power_manager.restore_default()?;
```

## 交互式演示

库包含一个交互式演示应用程序，展示了所有功能。使用以下命令运行：

```bash
cargo run --example power_management_demo
```

## API 参考

### PowerManager

- `new()` - 创建新的 PowerManager 实例
- `keep_awake_indefinite()` - 无限期防止系统睡眠
- `keep_awake_for_minutes(minutes: u32)` - 在指定时间内防止睡眠
- `force_sleep(hibernate: bool, force: bool, disable_wake_events: bool)` - 控制系统睡眠状态
- `restore_default()` - 恢复默认电源设置

## 错误处理

库使用自定义的 `PowerError` 枚举进行错误处理：

```rust
pub enum PowerError {
    WindowsError(windows::core::Error),
    InvalidDuration(String),
    SleepError(String),
}
```

## 系统要求

- Windows 操作系统
- Rust 1.56 或更高版本
- 某些操作可能需要管理员权限

## 常见问题及解决方案

1. **睡眠/休眠无法正常工作**
   - 确保系统已启用休眠功能
   - 以管理员权限运行应用程序
   - 检查是否有其他应用程序阻止睡眠状态

2. **权限错误**
   - 以管理员身份运行应用程序
   - 检查 Windows 电源设置

3. **防止睡眠功能不起作用**
   - 验证没有其他应用程序修改电源设置
   - 检查系统电源策略设置

## 贡献

欢迎提供贡献！请随时提交 Pull Request。对于重大更改，请先开一个 issue 讨论您想要更改的内容。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 版本历史

- 0.1.0 (2025-02-21)
  - 初始发布
  - 基本电源管理功能
  - 交互式演示应用程序

## 作者

由 cocshank 创建于 2025-02-21

## 支持

如果遇到任何问题或需要帮助，请：
1. 查看 [Issues](https://github.com/akirco/windows-awake/issues) 页面
2. 如果您的问题尚未列出，请创建新的 issue
3. 报告问题时请包含您的 Windows 版本和 Rust 版本

## 安全注意事项

⚠️ **重要：**
- 在强制睡眠或休眠之前，请始终保存您的工作
- 使用 `force_sleep` 时请谨慎，特别是当 `force = true` 时
- 长时间防止系统睡眠时，请考虑对系统的整体影响