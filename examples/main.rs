use std::{ io::{ self, Write }, thread, time::Duration };

use windows_awake::PowerManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let power_manager = PowerManager::new();

    println!("Windows power manager v{}", env!("CARGO_PKG_VERSION"));
    println!("------------------------");
    println!("\n Options:");
    println!("1. 无限保持系统唤醒");
    println!("2. 定时后进入睡眠");
    println!("3. 恢复系统默认设置");
    println!("4. 立即进入睡眠");
    println!("5. 立即进入休眠");
    println!("6. 强制进入睡眠");
    println!("7. 退出程序");

    loop {
        print!("\n请选择操作 (1-7): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!("正在设置系统为持续唤醒状态...");
                power_manager.keep_awake_indefinite()?;
                println!("✓ 系统已设置为持续唤醒状态");
            }
            "2" => {
                print!("请输入多少分钟后进入睡眠: ");
                io::stdout().flush()?;
                let mut duration = String::new();
                io::stdin().read_line(&mut duration)?;

                if let Ok(minutes) = duration.trim().parse::<u32>() {
                    println!("正在设置系统 {} 分钟后进入睡眠...", minutes);
                    power_manager.keep_awake_for_minutes(minutes)?;
                    println!("✓ 系统将在 {} 分钟后进入睡眠", minutes);

                    thread::spawn(move || {
                        let mut remaining = minutes;
                        while remaining > 0 {
                            thread::sleep(Duration::from_secs(60));
                            remaining -= 1;
                            println!("距离睡眠还剩: {} 分钟", remaining);
                        }
                    });
                } else {
                    println!("❌ 输入无效，请输入有效的分钟数");
                }
            }
            "3" => {
                println!("正在恢复系统默认电源设置...");
                power_manager.restore_default()?;
                println!("✓ 系统已恢复默认电源设置");
            }
            "4" => {
                println!("正在使系统进入睡眠状态...");
                if let Err(e) = power_manager.force_sleep(false, false, false) {
                    println!("❌ 进入睡眠失败: {}", e);
                }
            }
            "5" => {
                println!("正在使系统进入休眠状态...");
                if let Err(e) = power_manager.force_sleep(true, false, false) {
                    println!("❌ 进入休眠失败: {}", e);
                }
            }
            "6" => {
                println!("正在强制使系统进入睡眠状态...");
                if let Err(e) = power_manager.force_sleep(false, true, true) {
                    println!("❌ 强制睡眠失败: {}", e);
                }
            }
            "7" => {
                println!("正在退出程序...");
                power_manager.restore_default()?;
                println!("✓ 已恢复默认电源设置");
                break;
            }
            _ => println!("❌ 无效的选择，请重试"),
        }
    }

    Ok(())
}
