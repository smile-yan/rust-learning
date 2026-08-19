---
title: "Q15: Rust 需要手动 free 吗？"
module: "Q & A"
order: 15
code: |
  fn main() {
      // 完全不需要手动 free
      let s = String::from("Rust 自动管理内存");
      println!("{}", s);
  } // s 在这里自动释放

  // 与 C 交互时才可能需要手动管理（不安全代码）
  unsafe fn manual_memory_example() {
      // into_raw 把 Box 转为裸指针，放弃自动释放的责任转移给调用者
      let ptr = Box::into_raw(Box::new(42));
      println!("裸指针: {:?}", ptr);
      // 必须手动释放：from_raw 还原为 Box 再由 drop 释放，漏掉则泄漏
      drop(Box::from_raw(ptr));
  }

  fn main2() {
      unsafe {
          manual_memory_example();
      }
      println!("手动管理内存只用于 unsafe 或 FFI 场景");
  }
hint: "日常 Rust 代码完全不需要手动 free。只有在 unsafe 代码或与 C 交互时才可能需要手动管理内存。"
exercises:
  - title: "不需要 free"
    description: "创建 String，使用它，然后让 Rust 自动释放。"
    code_template: |
      fn main() {
          let s = String::from("auto");
          println!("{}", s);
      } // 自动 drop
---

# Q15: Rust 需要手动 free 吗？ 🚫

**不需要**。Rust 通过所有权系统自动管理内存，开发者不需要像 C/C++ 那样手动调用 free/delete。这是 Rust 相比系统编程语言的一大吸引力。

## 🧭 引入与类比

在 C/C++ 里，你租了一辆车，用完必须自己开回租车点还车 🚗。如果忘了，车就一直占着资源（内存泄漏）；如果还了两次，就会出问题（double-free）。

在 Rust 里，你租的车有「自动归还」功能：当你离开停车场（作用域）时，它会自己开回去。

## 💡 核心概念图解

![不需要手动 free](/images/module4-scope-drop.svg)

## 🔧 深入讲解

### 自动释放机制

当堆上分配的值离开作用域时，Rust 会自动调用 `drop` 函数释放内存。这个过程是确定性的，不需要运行时垃圾回收器。

```rust
fn main() {
    let s = String::from("Rust 自动管理内存");
    println!("{}", s);
} // s 在这里自动释放
```

### 什么时候需要手动干预？

- 使用 `Box::into_raw` 将 `Box` 转换为裸指针后，需要手动释放。
- 与 C 代码交互时，可能需要管理 C 分配的内存。
- 使用 `std::mem::forget` 显式跳过 `drop`（不推荐，会导致资源泄漏）。

### 智能指针的作用

`Box<T>`、`Rc<T>`、`Arc<T>` 等智能指针封装了堆内存管理，让代码更安全、更简洁。

## ⚠️ 常见误区

- **误区 1**：Rust 和 C 一样需要手动 free。
  - ✅ 正解：日常 Rust 代码完全不需要手动 free，所有权系统会自动处理。
- **误区 2**：自动管理意味着有 GC。
  - ✅ 正解：Rust 没有 GC，释放时机由所有权和作用域决定，完全确定。
- **误区 3**：unsafe 代码里也不需要 free。
  - ✅ 正解：与裸指针或 C FFI 交互时，仍然需要遵循对应内存规则。

## 📝 一句话总结

在纯 Rust 代码中，99% 的情况下不需要手动 free；所有权和借用规则会在编译期确保内存安全、自动释放。

<RustPlayground />
