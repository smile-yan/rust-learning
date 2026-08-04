---
title: 并发基础 thread
hint: Arc 是线程安全的 Rc，Mutex 提供锁保护的内部可变性。MutexGuard 离开作用域时自动释放锁。
---

# 并发基础 thread 🦀

Rust 的所有权和类型系统使得并发编程既安全又高效。编译器在编译期就能捕获很多并发错误，例如数据竞争，让多线程程序的错误率大大降低。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 从生活类比开始 💡

想象一个厨房里有多位厨师同时工作：如果大家都随意拿同一把刀，就会出事故。Rust 的并发模型像是给每把刀都加了“使用规则”——谁可以用、什么时候用、用完后怎么归还，编译器都帮你检查好。

## 概念图解

![创建线程](images/module1-concurrency-threads.svg)

主线程可以通过 `std::thread::spawn` 创建多个工作线程，Rust 在线程创建时就会检查数据是否安全传递。

![mpsc 消息传递](images/module1-concurrency-channel.svg)

`mpsc` 通道让线程之间通过发送消息来通信，数据的所有权随消息转移。

![Arc + Mutex 共享状态](images/module1-concurrency-arc-mutex.svg)

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

