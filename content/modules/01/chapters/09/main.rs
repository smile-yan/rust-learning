use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() {
    // 创建临时目录和文件
    // PathBuf 是可变的自有路径类型，跨平台处理路径分隔符
    let dir = PathBuf::from("tmp_data");
    // create_dir_all 递归创建目录；
    // unwrap_or_default 忽略「目录已存在」这类错误
    fs::create_dir_all(&dir).unwrap_or_default();

    // join 拼接路径，比字符串拼接更安全且跨平台
    let file_path = dir.join("example.txt");
    fs::write(&file_path, "Hello, Rust!\nLine 2").unwrap();

    // 一次性读取
    let content = fs::read_to_string(&file_path).unwrap();
    println!("文件内容:\n{}", content);

    // 带缓冲逐行读取
    // BufReader 减少系统调用次数，适合逐行处理大文件
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("读取行: {}", line.unwrap());
    }

    // 路径操作
    // set_extension 是替换（而非追加）扩展名
    let mut new_path = file_path.clone();
    new_path.set_extension("log");
    println!("新路径: {:?}", new_path);

    // 清理
    fs::remove_file(&file_path).unwrap_or_default();
    fs::remove_dir(&dir).unwrap_or_default();
}
