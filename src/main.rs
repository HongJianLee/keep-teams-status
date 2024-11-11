use chrono::Local;
use ctrlc;
use inputbot::KeybdKey;
use std::thread;
use std::time::Duration;
fn main() {
    let caps_lock_key = KeybdKey::CapsLockKey;

    // 定义信号处理器
    let signal_handler = move || {
        caps_lock_key.press();
        caps_lock_key.release();
        caps_lock_key.press();
        caps_lock_key.release();
        std::process::exit(0);
    };

    // 注册信号处理器
    ctrlc::set_handler(signal_handler).expect("Error setting Ctrl-C handler");
    // 主循环
    loop {
        println!("{:?}", Local::now());
        caps_lock_key.press();
        caps_lock_key.release();
        caps_lock_key.press();
        caps_lock_key.release();
        thread::sleep(Duration::from_secs(60));
    }
}
