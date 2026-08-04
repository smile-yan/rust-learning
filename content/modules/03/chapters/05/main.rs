use std::fs;

// 用 Result 作为返回类型，强迫调用方显式处理失败，而不是抛异常
fn read_file(path: &str) -> Result<String, String> {
    // ? 显式传播错误
    let content = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    Ok(content)
}

// Option 显式表达「可能没有结果」，不存在静默的 null
fn double_if_positive(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n * 2)
    } else {
        None
    }
}

fn main() {
    // 可变变量必须显式声明
    let mut count = 0;
    count += 1;
    println!("count: {}", count);

    // 显式错误处理
    match read_file("/etc/hosts") {
        Ok(content) => println!("读取了 {} 字节", content.len()),
        Err(e) => println!("读取失败: {}", e),
    }

    // Option 也必须显式处理
    match double_if_positive(5) {
        Some(v) => println!("结果: {}", v),
        None => println!("输入不合法"),
    }
}
