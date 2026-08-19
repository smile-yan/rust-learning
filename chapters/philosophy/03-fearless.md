---
title: "fearless 并发：共享状态的安全哲学"
module: "Rust 哲学"
order: 3
code: |
  use std::sync::mpsc;
  use std::thread;

  fn main() {
      // 使用 channel 在线程间传递数据所有权
      let (tx, rx) = mpsc::channel();

      // move 闭包把 tx 的所有权转移进子线程，主线程从此不再持有
      thread::spawn(move || {
          let data = String::from("来自子线程的数据");
          tx.send(data).unwrap();
          // data 的所有权已经发送出去，这里不能再使用
      });

      // recv 会阻塞当前线程，直到收到消息或所有发送端被丢弃
      let received = rx.recv().unwrap();
      println!("收到: {}", received);
  }
hint: "在 Rust 中，优先通过 channel 转移所有权，而不是直接共享可变状态。"
exercises:
  - title: "用 channel 传所有权"
    description: "在线程间通过 mpsc 发送 String 的所有权。"
    code_template: |
      use std::sync::mpsc;
      use std::thread;

      fn main() {
          let (tx, rx) = mpsc::channel();
          thread::spawn(move || {
              tx.send(String::from("hello")).unwrap();
          });
          println!("{}", rx.recv().unwrap());
      }
---

# fearless 并发：共享状态的安全哲学 🧵

并发编程一直是 bug 高发区：数据竞争、死锁、竞态条件让人头疼。Rust 的所有权和类型系统把并发安全也推进到了编译期。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 数据竞争的本质

数据竞争通常发生在：
- 多个线程同时访问同一块数据
- 至少有一个是写操作
- 没有同步机制

Rust 的借用规则天然阻止了这种情况：
- 同一时刻只能有一个 `&mut T`
- 同一时刻可以有多个 `&T`，但不能和 `&mut T` 共存


![借用规则拦截数据竞争](/images/module3-data-race.svg)

## Send 与 Sync

Rust 用两个 marker trait 描述类型的并发安全性：

- **`Send`**：可以安全地在线程间转移所有权。
- **`Sync`**：可以安全地在线程间共享不可变引用。

编译器会自动为类型推导这两个 trait。如果类型不满足条件，你就无法在线程间传递它。

## 通道 vs 共享内存

Rust 鼓励用消息传递（channel）来组织并发：每个线程拥有独立的数据，通过 channel 交换所有权。这比直接加锁共享内存更安全、更容易推理。

![消息传递：所有权随消息转移](/images/module3-channel-ownership.svg)

## 不是完全没有并发 bug

Rust 能防止数据竞争，但不能防止逻辑上的竞态条件或死锁。你仍然需要合理设计锁的粒度和顺序。

## 一句话总结 ✅

Rust 的并发哲学是：把数据竞争变成编译错误，让你敢于写多线程代码而不用担心 segmentation fault。

<RustPlayground />
