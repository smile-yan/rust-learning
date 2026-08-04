---
title: 'Q5: 为什么 Rust 没有 null？'
hint: Option 强制显式处理无值情况。可以使用 unwrap_or、map、and_then、? 等方法简化代码。
---

# Q5: 为什么 Rust 没有 null？ 🚫

Rust 的设计者 Tony Hoare 曾称 null 引用为「十亿美元错误」，因为它导致了无数的空指针异常。Rust 选择用 `Option<T>` 来彻底解决这个问题。

## 🧭 引入与类比

想象你去取快递 🎁：

- 在支持 null 的语言里，快递柜可能给你一个空盒子，你打开才会发现「哎呀，没有东西」，然后程序崩溃。
- 在 Rust 里，快递柜只会给你 `Some(包裹)` 或 `None`，你必须先确认是哪一种，才能继续处理。

## 💡 核心概念图解

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```
        Option<T>
        /        \
   Some(T)      None
      |           |
   有值，可解包   无值，必须处理
```

## 🔧 深入讲解

### 好处

1. **编译器强制处理空值** ✅：`match` 或 `if let` 必须覆盖 `None` 分支。
2. **没有运行时空指针异常** 🛡️：所有可能为空的情况都显式化。
3. **代码意图更清晰** 📖：`Option<i32>` 明确告诉读者「这里可能没有值」。

### 常用方法

- `unwrap_or(default)`：提供默认值。
- `map`：转换 `Some` 中的值，保持 `None` 不变。
- `and_then`：链式处理可能为 `None` 的操作。
- `?` 操作符：在返回 `Option` 的函数中传播 `None`。

```rust
let name = find_user(99).unwrap_or(String::from("访客"));
let upper = find_user(1).map(|n| n.to_uppercase());
```

## ⚠️ 常见误区

- **误区 1**：`Option` 只是 null 的另一种写法。
  - ✅ 正解：`Option` 把「可能为空」的信息编码进类型系统，编译器会强制你处理两种情况。
- **误区 2**：用 `unwrap` 处理 `Option` 是最佳实践。
  - ✅ 正解：`unwrap` 适合确定不会为 `None` 的场景，否则应使用 `match`、`if let` 或 `?`。
- **误区 3**：`Option<T>` 有运行时开销。
  - ✅ 正解：现代编译器通常会对 `Option` 做零成本优化，与裸指针效率相当。

## 📝 一句话总结

Rust 用 `Option<T>` 代替 null，把「可能没有值」这件事写进类型系统，让空指针异常在编译期就销声匿迹。

