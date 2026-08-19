---
title: "Q14: 什么是 RAII？"
module: "Q & A"
order: 14
code: |
  // RAII：资源获取即初始化，把资源的生命周期绑定到对象上
  struct FileGuard {
      name: String,
  }

  impl FileGuard {
      fn new(name: &str) -> FileGuard {
          // 构造即"打开"资源
          println!("打开文件: {}", name);
          FileGuard {
              name: name.to_string(),
          }
      }
  }

  // 实现 Drop：值离开作用域时自动执行清理，无需手动调用
  impl Drop for FileGuard {
      fn drop(&mut self) {
          println!("关闭文件: {}", self.name);
      }
  }

  fn main() {
      {
          let file = FileGuard::new("data.txt");
          println!("使用文件: {}", file.name);
      } // file 离开作用域，drop 被调用

      println!("文件已自动关闭");

      // 也可以显式 drop
      let file2 = FileGuard::new("temp.txt");
      // std::mem::drop 立即释放资源，而不是等作用域结束
      drop(file2);
      println!("temp.txt 已提前关闭");
  }
hint: "RAII 让资源管理与对象生命周期绑定，无需手动释放。MutexGuard 就是典型的 RAII 应用。"
exercises:
  - title: "实现简单的 Drop"
    description: "为一个 Guard 结构体实现 Drop，观察它离开作用域时的输出。"
    code_template: |
      struct Guard(&'static str);

      impl Drop for Guard {
          fn drop(&mut self) {
              println!("drop {}", self.0);
          }
      }

      fn main() {
          let _g = Guard("A");
      }
---

# Q14: 什么是 RAII？ 🔒

RAII 是 **Resource Acquisition Is Initialization** 的缩写，即「资源获取即初始化」。这是 Rust 资源管理的核心思想，也是它能不用 GC 就保证安全的关键机制之一。

## 🧭 引入与类比

想象你入住酒店 🏨：

- 办理入住时拿到房卡（资源获取即初始化）。
- 你在房间里使用各种设施（使用资源）。
- 退房时归还房卡，房间被清理（离开作用域，自动释放资源）。

你不需要手动打电话给前台说「我要退房」，系统在你离开时就自动处理了。

![RAII 资源生命周期](/images/module3-raii.svg)

## 💡 核心概念图解

### 核心思想

- 资源的生命周期与对象的生命周期绑定。
- 对象创建时获取资源。
- 对象销毁时释放资源。

## 🔧 深入讲解

### 在 Rust 中的体现

Rust 通过实现 `Drop` trait 来自定义资源释放逻辑。当值离开作用域时，会自动调用 `drop` 方法。

```rust
struct FileGuard {
    name: String,
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("关闭文件: {}", self.name);
    }
}
```

### 示例资源

- **内存**：`String`、`Vec`、`Box` 等离开作用域时释放堆内存。
- **文件**：文件句柄自动关闭。
- **锁**：`MutexGuard` 离开作用域时自动释放锁。
- **网络连接**：连接断开。

### 与 GC 的对比

RAII 提供了**确定性的资源释放**，而 GC 只能保证内存最终会被回收，其他资源（如文件、锁）通常需要额外处理。

## ⚠️ 常见误区

- **误区 1**：RAII 只和内存有关。
  - ✅ 正解：RAII 管理一切资源，包括文件、锁、网络连接、数据库句柄等。
- **误区 2**：`drop` 必须手动调用。
  - ✅ 正解：大多数情况下 Rust 会自动调用 `drop`；只有需要提前释放时才显式 `drop(value)`。
- **误区 3**：RAII 是 Rust 独有的。
  - ✅ 正解：RAII 起源于 C++，但 Rust 的所有权系统让它在编译期就更安全、更不易出错。

## 📝 一句话总结

RAII 让资源的生命周期与对象绑定，创建时获取、离开作用域时自动释放，无需手动 free/close/unlock。

<RustPlayground />
