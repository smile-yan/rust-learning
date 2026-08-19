---
title: "Q2: &str 和 String 有什么区别？"
module: "Q & A"
order: 2
code: |
  fn main() {
      // String 拥有数据：存于堆上，可增长、可修改
      let mut s = String::from("hello");
      s.push_str(" world");
      println!("String: {}", s);

      // &str 是借用：只读视图，不拿走所有权，s 之后仍可使用
      let slice: &str = &s;
      println!("&str: {}", slice);

      // 字符串字面量是 '&static str'
      let literal: &str = "Rust";
      println!("字面量: {}", literal);

      // &str -> String
      let owned = literal.to_string();
      println!("转换后: {}", owned);
  }
hint: "函数参数尽量使用 &str，这样调用者可以传入 String 或字符串字面量。"
exercises:
  - title: "String 与 &str 转换"
    description: "把 String 转成 &str，再把 &str 转成 String。"
    code_template: |
      fn main() {
          let s = String::from("hello");
          let slice: &str = &s;
          let owned: String = slice.to_string();
          println!("{}", owned);
      }
---

# Q2: &str 和 String 有什么区别？ 🧵

这是 Rust 新手最容易混淆的概念之一，也是面试中的高频题。搞懂它们的区别，你就跨过了 Rust 学习的第一道坎。

## 🧭 引入与类比

把字符串想象成一本书 📚：

- `String` 是你**买回家**的书，你拥有它，可以涂改、增删、送人。
- `&str` 是书里某几页的**复印件**，你不拥有原书，只是临时看看其中一段内容。

## 💡 核心概念图解

![String 与 &str 的内存布局](/images/module4-string-str-memory.svg)

## 🔧 深入讲解

### String

- 拥有堆上的 UTF-8 数据。
- 可增长、可修改（如果是可变的）。
- 生命周期由其自身决定，离开作用域自动释放。

### &str

- 字符串切片，是对 UTF-8 数据的借用引用。
- 不拥有数据，只是「借用」一段字符串。
- 可以是堆上 `String` 的切片，也可以是字符串字面量（`'static str`）。

### 转换关系

| 方向 | 方式 | 是否消耗 |
|------|------|----------|
| `String` → `&str` | 自动强制转换或 `.as_str()` | 否（借用） |
| `&str` → `String` | `.to_string()` / `.to_owned()` | 是（分配堆内存） |

## ⚠️ 常见误区

- **误区 1**：`&str` 只能指向字符串字面量。
  - ✅ 正解：`&str` 可以指向任何 UTF-8 数据，包括 `String` 的切片。
- **误区 2**：函数返回 `&str` 时不需要考虑生命周期。
  - ✅ 正解：返回引用必须确保它不会指向已经释放的数据，否则就是悬挂引用。
- **误区 3**：为了省事，所有地方都用 `String`。
  - ✅ 正解：函数参数优先用 `&str`，调用者更灵活，也避免不必要的内存分配。

## 📝 一句话总结

`String` 是拥有的、可增长的堆字符串；`&str` 是借用的、不可变的字符串切片——函数参数优先用 `&str`，需要拥有和修改时再用 `String`。

<RustPlayground />
