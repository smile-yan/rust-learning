use thiserror::Error;
use anyhow::{Context, Result};

#[derive(Error, Debug)]
enum DataError {
    #[error("字段缺失: {0}")]
    MissingField(String),
    #[error("类型不匹配")]
    TypeMismatch,
}

fn parse_age(data: &str) -> Result<u32> {
    let age = data
        .parse::<u32>()
        .context("age 不是有效数字")?;
    Ok(age)
}

fn main() -> Result<()> {
    let content = std::fs::read_to_string("user.json")
        .context("无法打开 user.json")?;
    let age = parse_age(&content)?;
    println!("年龄: {}", age);
    Ok(())
}
