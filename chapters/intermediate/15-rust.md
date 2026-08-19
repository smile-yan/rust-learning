---
title: "常用设计模式（Rust 风格）"
module: "中等应用"
order: 15
code: |
  // 策略模式：Box<dyn Trait> 实现运行时多态
  trait Payment {
      fn pay(&self, amount: u64);
  }

  struct Alipay;
  struct WechatPay;

  impl Payment for Alipay {
      fn pay(&self, amount: u64) { println!("支付宝支付 {}", amount); }
  }
  impl Payment for WechatPay {
      fn pay(&self, amount: u64) { println!("微信支付 {}", amount); }
  }

  struct ShoppingCart {
      payment: Box<dyn Payment>,
  }

  impl ShoppingCart {
      fn checkout(&self, amount: u64) {
          self.payment.pay(amount);
      }
  }

  fn main() {
      let cart = ShoppingCart { payment: Box::new(Alipay) };
      cart.checkout(100);
  }
hint: "Rust 的设计模式强调组合优于继承。策略用 trait + dyn，状态机用 enum + match。"
exercises:
  - title: "实现策略模式"
    description: "定义 Payment trait，为 Alipay 和 WechatPay 实现，在 checkout 中调用。"
    code_template: |
      trait Payment { fn pay(&self, amount: u64); }
      struct Alipay;
      struct WechatPay;

      fn main() {
          // impl Payment for ...
      }
---

# 常用设计模式（Rust 风格）🦀

设计模式是解决特定问题的可复用方案。Rust 没有类继承，因此经典 GoF 模式在这里被重新诠释：策略模式用 trait 表达，状态机用 enum + match 表达，观察者用闭包表达，构建者用方法链表达。

## 从生活类比开始 💡

设计模式就像乐高积木的标准接口：你不需要 reinvent the wheel，只需选合适的积木块拼装。Rust 的设计模式强调组合优于继承、枚举穷尽、闭包灵活。

![Rust 风格设计模式](/images/module1-design-patterns.svg)

## 深入讲解

### 策略模式 Strategy

用 trait 定义算法契约，运行时用 `Box<dyn Strategy>` 替换具体实现。

### 状态机 State Machine

用 enum 表示所有状态，用 match 穷尽处理。编译期保证状态不可随意组合。

### 观察者 Observer

用 `Vec<Box<dyn Fn()>>` 维护回调列表，事件触发时调用所有观察者。

### 构建者 Builder

Rust 的标准库广泛使用构建者模式：`std::process::Command`、`http::Request::builder()`。

## 一句话总结 🦀

> Rust 用 trait、枚举和闭包重新诠释经典模式：策略可插拔、状态机编译期穷尽、观察者用闭包、构建者链式组装。

<RustPlayground />
