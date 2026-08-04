use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let start = Instant::now();
    for _ in 0..1_000_000 { let _ = 1 + 1; }
    println!("耗时: {:?}", start.elapsed());

    let now = SystemTime::now();
    let ts = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("当前时间戳: {}", ts);

    std::thread::sleep(Duration::from_millis(500));
    println!("已睡眠 500ms");
}
