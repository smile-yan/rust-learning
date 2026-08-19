---
title: "组合优于继承：设计哲学的选择"
module: "Rust 哲学"
order: 4
code: |
  // 组合：Car 由 Engine 和 Wheels 组成
  struct Engine;
  struct Wheels;

  impl Engine {
      fn start(&self) {
          println!("发动机启动");
      }
  }

  impl Wheels {
      fn roll(&self) {
          println!("车轮转动");
      }
  }

  struct Car {
      engine: Engine,
      wheels: Wheels,
  }

  impl Car {
      // 组合的关键：Car 不继承任何行为，而是把具体工作委托给内部组件
      fn start(&self) {
          self.engine.start();
          self.wheels.roll();
          println!("汽车开始行驶");
      }
  }

  // trait：定义飞行的能力
  trait Flyable {
      fn fly(&self);
  }

  // Bird 只需实现 Flyable，就能获得被 let_it_fly 使用的能力
  struct Bird;
  impl Flyable for Bird {
      fn fly(&self) {
          println!("鸟在飞翔");
      }
  }

  // &dyn Flyable 是 trait 对象：运行时动态分发，
  // 接受任何实现了 Flyable 的类型
  fn let_it_fly(f: &dyn Flyable) {
      f.fly();
  }

  fn main() {
      let car = Car { engine: Engine, wheels: Wheels };
      car.start();

      let bird = Bird;
      let_it_fly(&bird);
  }
hint: "当你想写继承时，先考虑能否用组合 + trait 实现。Rust 的组合方式通常更清晰、更安全。"
exercises:
  - title: "组合实现汽车"
    description: "用 Engine 和 Wheels 组合成 Car，并实现 start 方法。"
    code_template: |
      struct Engine;
      struct Wheels;

      impl Engine { fn start(&self) { println!("engine"); } }
      impl Wheels { fn roll(&self) { println!("wheels"); } }

      struct Car { engine: Engine, wheels: Wheels }

      impl Car {
          fn start(&self) {
              self.engine.start();
              self.wheels.roll();
          }
      }

      fn main() { Car { engine: Engine, wheels: Wheels }.start(); }
---

# 组合优于继承：设计哲学的选择 🧩

传统面向对象语言常常通过类继承复用代码，但继承层次过深会导致「脆弱的基类」问题：子类被迫依赖父类的实现细节，一改父类就可能破坏所有子类。

## Rust 的选择

Rust 没有类继承。它用两种更灵活的方式实现代码复用：

1. **组合（Composition）**：一个结构体包含另一个结构体，通过委托使用其能力。
2. **Trait**：定义行为接口，不同类型可以实现同一个 trait，也可以有默认实现。

## 组合 vs 继承

| 维度 | 继承 | 组合 + Trait |
|------|------|--------------|
| 耦合度 | 高，子类依赖父类 | 低，依赖接口和包含对象 |
| 灵活性 | 单继承，受限 | 可实现多个 trait |
| 可测试性 | 需要 Mock 父类 | 依赖接口更容易 Mock |
| 意图清晰度 | 隐藏实现细节 | 显式列出依赖 |


![继承树 vs 组合积木](/images/module3-inheritance-vs-composition.svg)

## 多态依然存在

Rust 通过 `&dyn Trait` 和泛型 trait bound 实现多态。你仍然可以写出高度抽象的代码，只是不依赖类层次结构。

```
trait Flyable {
    fn fly(&self);
}

fn let_it_fly(f: &dyn Flyable) {
    f.fly();
}
```


![trait 契约：实现即获得能力](/images/module3-trait-contract.svg)

## 为什么这样更好？

组合让你只引入需要的能力，不会被不需要的父类方法绑架。trait 则让你定义清晰的契约，而不是隐含的血缘关系。

## 一句话总结 ✅

Rust 的设计哲学是：用组合拼装能力，用 trait 定义契约，避免继承带来的紧耦合和脆弱基类。

<RustPlayground />
