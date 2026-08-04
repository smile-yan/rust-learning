---
title: 错误处理 Result / panic!
hint: unwrap() 在 Err 时会导致程序崩溃，实际代码中建议用 match 或 ?。? 操作符只能在返回 Result/Option 的函数中使用。
---

# 错误处理 Result / panic!

Rust 没有异常机制，而是使用 `Result<T, E>` 和 `Option<T>` 这两个枚举来显式处理可能失败或可能缺失的情况。这种方式让错误处理路径变得清晰可见，是 Rust 可靠性的重要来源。

## 引入：失败是常态，但要坦然面对 🛡️

文件可能打不开、网络可能中断、用户输入可能无效。Rust 要求你把这些可能性体现在类型里，而不是用隐藏的全局状态或异常跳转。

![错误处理示意图](images/module0-error-handling.svg)

## 概念图解 💡

![Result 与 Option：把结果包起来](images/module0-result-option.svg)

## 深入讲解

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

用于可能失败的操作，如文件打开、网络请求：

```rust
let f = File::open("hello.txt");
match f {
    Ok(file) => file,
    Err(error) => panic!("{:?}", error),
}
```

### Option

用于可能为空的值：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### ? 运算符

`?` 运算符可以简化错误传播：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

如果 `?` 前面的表达式是 `Err`，函数会立即返回该错误；如果是 `Ok`，则取出值继续执行。

### unwrap / expect

- `unwrap()`：成功就返回值，失败就 panic。适合示例代码或确定不会失败的场景。
- `expect("msg")`：类似 unwrap，但允许自定义 panic 信息。

## 常见误区 ⚠️

- 在生产代码中滥用 `unwrap`，导致程序 panic。
- 忽略 `Result` 返回值：Rust 编译器会警告未使用的 `Result`。
- 混淆 `Option` 和 `Result` 的使用场景。
- 在 `main` 函数中随意使用 `?`：`main` 默认返回 `()`，需要使用 `Result<(), E>` 签名。

## 一句话总结 ✅

Rust 用 `Result` 和 `Option` 把错误/null 显式化，配合 `match` 和 `?` 写出可靠、可预测的错误处理代码。

