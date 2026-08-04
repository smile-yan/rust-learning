---
title: 'Q17: Rust 如何实现继承？'
hint: Rust 没有继承，但组合 + trait 默认实现可以覆盖大多数继承的使用场景，而且通常更安全。
---

# Q17: Rust 如何实现继承？ 🔧

Rust **没有类继承**语法，但这并不意味着你不能实现类似继承的代码复用。Rust 提供了多种更安全的替代方案。

## 🧭 引入与类比

传统继承就像 DNA 遗传 🧬：孩子自动获得父母的所有特征，但也可能被不想要的特征绑架。Rust 则更像是「招聘制」：你需要什么能力，就组合什么组件，或者让类型遵守某个协议（trait）。

## 💡 核心概念图解

### 方案对比

![没有 class 继承，Rust 怎么做](images/module4-inheritance-alternatives.svg)

## 🔧 深入讲解

### 方案一：组合（Composition）

在一个结构体中包含另一个结构体，通过委托调用其方法。这是 Rust 中首选的复用方式。

```rust
struct Car {
    engine: Engine,
}

impl Car {
    fn start(&self) {
        self.engine.start();
    }
}
```

### 方案二：Trait 默认实现

Trait 可以提供默认方法实现，多个类型实现同一个 trait 时可以复用默认逻辑，也可以覆盖。

```rust
trait Greetable {
    fn name(&self) -> &str;
    fn greet(&self) {
        println!("你好，我是 {}", self.name());
    }
}
```

### 方案三：Trait Bound

通过 trait bound 限制泛型参数，实现类似「基类约束」的效果。

```rust
fn process<T: Animal>(animal: T) { ... }
```

### 方案四：Deref 委托

实现 `Deref` trait 可以让一个类型自动调用另一个类型的方法，简化组合的使用。

### 为什么不支持继承？

- 继承会破坏封装，子类可能依赖父类实现细节。
- 继承层次过深会导致代码难以理解和维护。
- Rust 更推崇「组合优于继承」的设计哲学。

## ⚠️ 常见误区

- **误区 1**：Rust 缺少继承是个重大缺陷。
  - ✅ 正解：继承带来的紧耦合往往弊大于利，Rust 的组合方案通常更清晰。
- **误区 2**：组合会让代码变长。
  - ✅ 正解：确实需要多写一点，但换来了更明确的依赖关系和更高的可维护性。
- **误区 3**：Deref 可以完全替代继承。
  - ✅ 正解：Deref 是语法糖，不能滥用；过度使用会让代码意图变得模糊。

## 📝 一句话总结

Rust 没有类继承，但组合、trait 默认实现和 trait bound 能更安全、更灵活地实现代码复用；当你想继承时，先问自己能不能用组合 + trait 解决。

