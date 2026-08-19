---
title: "Q16: Rust 是面向对象语言吗？"
module: "Q & A"
order: 16
code: |
  // Rust 用结构体 + impl 实现封装
  struct BankAccount {
      // 字段默认私有，外部不能直接读写 balance
      owner: String,
      balance: f64,
  }

  impl BankAccount {
      fn new(owner: &str, balance: f64) -> Self {
          BankAccount {
              owner: owner.to_string(),
              balance,
          }
      }

      // 修改必须经过方法，非法金额被挡在门外
      fn deposit(&mut self, amount: f64) {
          if amount > 0.0 {
              self.balance += amount;
          }
      }

      // 只读访问通过 getter 暴露
      fn balance(&self) -> f64 {
          self.balance
      }
  }

  // Rust 用 trait 实现多态
  trait Animal {
      fn speak(&self);
  }

  struct Dog;
  struct Cat;

  impl Animal for Dog {
      fn speak(&self) {
          println!("汪汪");
      }
  }

  impl Animal for Cat {
      fn speak(&self) {
          println!("喵喵");
      }
  }

  // &dyn Animal 是 trait 对象：运行时动态分发，
  // 可接受任何实现了 Animal 的类型
  fn make_speak(animal: &dyn Animal) {
      animal.speak();
  }

  fn main() {
      let mut account = BankAccount::new("Alice", 100.0);
      account.deposit(50.0);
      println!("{} 的余额: {}", account.owner, account.balance());

      let dog = Dog;
      let cat = Cat;
      // 同一接口、不同实现各自响应，这就是多态
      make_speak(&dog);
      make_speak(&cat);
  }
hint: "Rust 没有继承，但用组合 + trait 可以实现面向对象的核心特性。&dyn Trait 是动态分发的一种方式。"
exercises:
  - title: "封装与多态"
    description: "定义 BankAccount 封装 balance，并用 trait 实现多态。"
    code_template: |
      struct BankAccount { balance: f64 }

      impl BankAccount {
          fn deposit(&mut self, amount: f64) { self.balance += amount; }
          fn balance(&self) -> f64 { self.balance }
      }

      fn main() {
          let mut acc = BankAccount { balance: 0.0 };
          acc.deposit(100.0);
          println!("{}", acc.balance());
      }
---

# Q16: Rust 是面向对象语言吗？ 🐕

Rust **不是传统意义上的面向对象语言**，但它支持很多面向对象的编程特性。Rust 更接近多范式语言，融合了命令式、函数式和面向对象的思想。

## 🧭 引入与类比

传统面向对象语言像是一套严格的家族继承制度 👨‍👩‍👧‍👦：儿子继承父亲的财产和方法。Rust 则像是一个团队合作模式：每个人都可以有自己的特长（结构体 + impl），并通过共同遵守的协议（trait）一起工作。

## 💡 核心概念图解

### 面向对象的三个经典特征

根据《设计模式》一书的定义，面向对象语言通常有三个特征：

1. **对象**：包含数据和行为。
2. **封装**：隐藏内部实现细节。
3. **继承**：通过父类复用代码。

### Rust 支持什么？

- **封装**：通过 `pub` 控制可见性，结构体和枚举可以隐藏内部字段。
- **多态**：通过 trait 实现，类似于接口。
- **对象**：结构体 + impl 块可以封装数据和行为。

### Rust 不支持什么？

- **继承**：Rust 没有类继承机制。代码复用主要通过组合和 trait 实现。
- **虚函数表**：没有传统意义的类层次结构，但 `&dyn Trait` 提供动态分发。

## 🔧 深入讲解

### 为什么不用继承？

Rust 的设计者认为继承会带来紧耦合和脆弱的基类问题。组合（composition）更灵活、更安全：

```rust
struct Car {
    engine: Engine,  // 组合
}
```

### 多态实现

```rust
trait Animal {
    fn speak(&self);
}

fn make_speak(animal: &dyn Animal) {
    animal.speak();
}
```

## ⚠️ 常见误区

- **误区 1**：Rust 完全不是面向对象语言。
  - ✅ 正解：Rust 支持封装和多态，只是不支持继承。
- **误区 2**：没有继承就无法复用代码。
  - ✅ 正解：组合 + trait 默认实现可以覆盖大多数继承场景。
- **误区 3**：`&dyn Trait` 就是传统类的多态。
  - ✅ 正解：`&dyn Trait` 是 trait 对象的动态分发，没有类层次结构。

## 📝 一句话总结

Rust 提供了面向对象的核心优点（封装、多态），同时避免了继承的缺点；你可以用面向对象风格写 Rust，也可以用函数式风格。

<RustPlayground />
