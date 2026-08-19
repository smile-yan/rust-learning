---
title: "Q12: Rust 如何避免内存泄漏？"
module: "Q & A"
order: 12
code: |
  use std::rc::Rc;

  fn main() {
      // 正常情况：Rc 在离开作用域后释放
      {
          // Rc 提供共享所有权，内部维护引用计数
          let data = Rc::new(vec![1, 2, 3]);
          // clone 只把计数加一（下面打印为 2），并不复制数据
          let _clone = Rc::clone(&data);
          println!("引用计数: {}", Rc::strong_count(&data));
      } // 两个 Rc 都离开作用域，计数归零，内存释放

      println!("正常 Rc 使用不会泄漏");

      // 循环引用示例（会导致泄漏，应避免）
      // a 和 b 互相持有对方，计数永远归不了零；
      // 真要打破循环需配合 Weak
      // let a = Rc::new(RefCell::new(None));
      // let b = Rc::new(RefCell::new(Some(Rc::clone(&a))));
      // *a.borrow_mut() = Some(Rc::clone(&b));

      println!("Rust 的借用规则帮助我们避免大多数循环引用。");
  }
hint: "Rust 的编译器能防止大多数内存泄漏，但 Rc/Arc 的循环引用仍然需要开发者注意。"
exercises:
  - title: "使用 Rc 并观察计数"
    description: "创建 Rc，clone 后打印 strong_count，离开作用域后再打印。"
    code_template: |
      use std::rc::Rc;

      fn main() {
          let r = Rc::new(1);
          {
              let _r2 = Rc::clone(&r);
              println!("count = {}", Rc::strong_count(&r));
          }
          println!("count = {}", Rc::strong_count(&r));
      }
---

# Q12: Rust 如何避免内存泄漏？ 🛡️

Rust 的设计从多个层面减少了内存泄漏的风险，但需要明确的是：**Rust 不能 100% 保证没有泄漏**。它通过编译期规则和智能指针设计，把泄漏场景压缩到极少数。

## 🧭 引入与类比

想象你住在一间公寓 🏠：

- Rust 的规则就像「每个人只能有一把钥匙，搬走的时候必须交出钥匙」。这样大多数情况下不会有人赖着不走。
- 但如果你和朋友互相交换了备用钥匙（循环引用），即使你们都搬走了，门也锁不上，资源就被困住了。

![Rc 循环引用导致内存泄漏](/images/module3-memory-leak.svg)

## 💡 核心概念图解

### 编译期保证

- **所有权规则**：每个值有且只有一个所有者，所有者离开作用域时释放资源。
- **借用检查**：引用不能比被引用的数据活得更久。
- **RAII**：资源与对象生命周期绑定，构造函数获取资源，析构函数释放资源。

## 🔧 深入讲解

### 引用计数与泄漏

`Rc<T>` 和 `Arc<T>` 使用引用计数。如果形成循环引用，引用计数永远不会归零，会导致内存泄漏。Rust 通过借用规则在很大程度上避免了循环引用，但 `Rc<RefCell<T>>` 组合仍然可能人为制造循环。

```rust
// 循环引用示例（会导致泄漏，应避免）
let a = Rc::new(RefCell::new(None));
let b = Rc::new(RefCell::new(Some(Rc::clone(&a))));
*a.borrow_mut() = Some(Rc::clone(&b));
```

### 可能的泄漏场景

- **循环引用**：使用 `Rc<RefCell<T>>` 时。
- **无限增长集合**：长时间运行的程序中忘记清理的缓存或日志队列。
- **`std::mem::forget`**：显式跳过 `drop`，通常只在特殊 FFI 场景使用。

### 最佳实践

- 优先使用借用而不是引用计数。
- 需要共享时考虑 `Arc<Mutex<T>>` 等组合。
- 注意集合的清理，避免无限增长。
- 必要时使用 `Weak<T>` 打破 `Rc`/`Arc` 循环。

## ⚠️ 常见误区

- **误区 1**：Rust 绝对不会内存泄漏。
  - ✅ 正解：Rust 能防止大多数泄漏，但循环引用和人为 forget 仍可能泄漏。
- **误区 2**：只要用 `Rc` 就会泄漏。
  - ✅ 正解：正常使用 `Rc` 不会泄漏，只有形成循环引用才会。
- **误区 3**：泄漏只和内存有关。
  - ✅ 正解：泄漏的资源还包括文件句柄、锁、网络连接等。

## 📝 一句话总结

Rust 通过所有权、借用和 RAII 在编译期防止大多数内存泄漏，但开发者仍需警惕 `Rc`/`Arc` 循环引用和无限增长集合等运行时泄漏场景。

<RustPlayground />
