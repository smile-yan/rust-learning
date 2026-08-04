// pub 使函数对 crate 外部可见，构成库的公共 API
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 用 Result 显式返回错误，而不是 panic，调用方可以优雅处理
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("除数不能为零")
    } else {
        Ok(a / b)
    }
}

// 文档注释（///）中 ``` 包裹的示例代码，
// 会被 cargo test 作为文档测试执行
/// 返回一个问候语
///
/// # Examples
///
/// ```
/// assert_eq!(rust_projects::greet("Rust"), "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("Rust"));
    println!("2 + 3 = {}", add(2, 3));
    println!("10 / 2 = {:?}", divide(10, 2));
    println!("10 / 0 = {:?}", divide(10, 0));
}

// #[cfg(test)] 标记的模块只在运行测试时编译，不进入发布产物
#[cfg(test)]
mod tests {
    // use super::* 把外层模块的项引入测试作用域
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_divide_ok() {
        assert_eq!(divide(10, 2).unwrap(), 5);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
