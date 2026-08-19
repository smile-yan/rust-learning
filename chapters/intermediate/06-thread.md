---
title: "并发基础 thread"
module: "中等应用"
order: 6
code: |
  use std::sync::{mpsc, Arc, Mutex};
  use std::thread;
  use std::time::Duration;

  fn main() {
      // 消息传递
      // mpsc 通道：多生产者单消费者，tx 是发送端，rx 是接收端
      let (tx, rx) = mpsc::channel();

      // move 闭包把 tx 的所有权移入新线程，
      // 保证同一时刻只有一个线程持有它
      thread::spawn(move || {
          let vals = vec!["hi", "from", "the", "thread"];
          for val in vals {
              tx.send(val).unwrap();
              thread::sleep(Duration::from_millis(100));
          }
      });

      // rx 可当迭代器用：阻塞等待消息，所有发送端 drop 后循环自动结束
      for received in rx {
          println!("收到: {}", received);
      }

      // 共享状态
      // Arc 是线程安全版的 Rc（原子引用计数），
      // 配合 Mutex 实现跨线程共享可变数据
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..5 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              // lock 返回锁守卫，离开作用域时自动释放锁（RAII）
              let mut num = counter.lock().unwrap();
              *num += 1;
          });
          handles.push(handle);
      }

      // join 等待所有线程结束，确保计数完成后再读结果
      for handle in handles {
          handle.join().unwrap();
      }

      println!("计数器结果: {}", *counter.lock().unwrap());
  }
hint: "Arc 是线程安全的 Rc，Mutex 提供锁保护的内部可变性。MutexGuard 离开作用域时自动释放锁。"
exercises:
  - title: "spawn 打印数字"
    description: "用 thread::spawn 启动一个线程打印 1 到 3，主线程等待其结束。"
    code_template: |
      use std::thread;

      fn main() {
          let handle = thread::spawn(|| {
              for i in 1..=3 {
                  println!("{}", i);
              }
          });
          handle.join().unwrap();
      }
  - title: "mpsc 发送消息"
    description: "用 channel 从子线程向主线程发送字符串。"
    code_template: |
      use std::sync::mpsc;
      use std::thread;

      fn main() {
          let (tx, rx) = mpsc::channel();
          thread::spawn(move || {
              tx.send("hi".to_string()).unwrap();
          });
          println!("{}", rx.recv().unwrap());
      }
---

# 并发基础 thread 🦀

Rust 的所有权和类型系统使得并发编程既安全又高效。编译器在编译期就能捕获很多并发错误，例如数据竞争，让多线程程序的错误率大大降低。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 从生活类比开始 💡

想象一个厨房里有多位厨师同时工作：如果大家都随意拿同一把刀，就会出事故。Rust 的并发模型像是给每把刀都加了“使用规则”——谁可以用、什么时候用、用完后怎么归还，编译器都帮你检查好。

## 概念图解

![创建线程](/images/module1-concurrency-threads.svg)

主线程可以通过 `std::thread::spawn` 创建多个工作线程，Rust 在线程创建时就会检查数据是否安全传递。

![mpsc 消息传递](/images/module1-concurrency-channel.svg)

`mpsc` 通道让线程之间通过发送消息来通信，数据的所有权随消息转移。

![Arc + Mutex 共享状态](/images/module1-concurrency-arc-mutex.svg)

`Arc<Mutex<T>>` 是 Rust 中常见的共享可变状态模式：`Arc` 负责线程安全的引用计数，`Mutex` 负责互斥访问。

## 深入讲解

### 线程

使用 `std::thread::spawn` 创建新线程。新线程可能需要 `'static` 生命周期的数据。

### 消息传递

`mpsc`（multi-producer, single-consumer）通道用于线程间通信。发送者使用 `tx.send()`，接收者使用 `rx.recv()` 或 `for received in rx`。

### 共享状态

- `Arc<T>`：原子引用计数，线程安全的共享所有权。
- `Mutex<T>`：互斥锁，保证同一时间只有一个线程能访问数据。
- `RwLock<T>`：读写锁，允许多个读或一个写。

### Send 与 Sync

- `Send`：类型可以安全地在线程间转移所有权。
- `Sync`：类型可以安全地在线程间共享引用。

绝大多数类型自动实现这两个 trait，违反规则会在编译期报错。

## 常见误区 ⚠️

- **误区 1**：Rust 并发代码不需要考虑数据竞争。  
  ✅ 正解：Rust 在编译期阻止数据竞争，但仍需正确设计锁粒度和消息协议。
- **误区 2**：`Mutex` 可以保护整个程序状态。  
  ✅ 正解：锁粒度要尽可能小，避免性能瓶颈和死锁。
- **误区 3**：闭包中可以直接使用外部变量。  
  ✅ 正解：必须使用 `move` 转移所有权，或使用 `Arc`/`Mutex` 共享。

## 一句话总结 🦀

> Rust 用所有权把并发错误消灭在编译期：要么通过消息传递转移所有权，要么通过锁安全共享。

<RustPlayground />
