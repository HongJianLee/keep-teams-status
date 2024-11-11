use chrono::Local;
use ctrlc;
use inputbot::KeybdKey;
use std::thread;
use std::time::Duration;

// 定义一个函数来模拟两次按下和释放大写锁定键
fn toggle_caps_lock(key: &KeybdKey) {
    key.press();
    key.release();
    key.press();
    key.release();
}

fn main() {
    let caps_lock_key = KeybdKey::CapsLockKey;

    // 定义信号处理器
    let signal_handler = move || {
        toggle_caps_lock(&caps_lock_key);
        std::process::exit(0);
    };

    // 注册信号处理器
    ctrlc::set_handler(signal_handler).expect("Error setting Ctrl-C handler");

    // 主循环
    loop {
        // 打印当前时间
        println!("{:?}", Local::now());

        // 模拟两次按下和释放大写锁定键
        toggle_caps_lock(&caps_lock_key);

        // 线程睡眠60秒
        thread::sleep(Duration::from_secs(60));
    }
}
