use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("找不到 rustc");

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("Rust 版本: {}", version.trim());
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("错误: {}", err);
    }

    let mut child = Command::new("sleep")
        .arg("2")
        .spawn()
        .expect("spawn 失败");

    println!("子进程已启动，PID: {:?}", child.id());
    let status = child.wait().expect("wait 失败");
    println!("子进程退出: {}", status);
}
