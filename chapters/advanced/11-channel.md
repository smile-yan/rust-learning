---
title: "并发模式与设计 channel"
module: "高级应用"
order: 11
code: |
  use std::sync::{Arc, RwLock};
  use std::thread;

  fn main() {
      // 读写锁示例
      let data = Arc::new(RwLock::new(vec![1, 2, 3]));
      let mut handles = vec![];

      // 多个读线程
      for _ in 0..3 {
          // Arc 提供线程安全的共享所有权，clone 只增加引用计数
          let data = Arc::clone(&data);
          // move 闭包把 data 的所有权移入新线程
          let handle = thread::spawn(move || {
              let read = data.read().unwrap();
              println!("读取: {:?}", *read);
          });
          handles.push(handle);
      }

      // 一个写线程
      let data = Arc::clone(&data);
      let write_handle = thread::spawn(move || {
          // write() 获取独占写锁，会等待所有读锁释放
          let mut write = data.write().unwrap();
          write.push(4);
          println!("写入后: {:?}", *write);
      });
      handles.push(write_handle);

      // join 等待所有线程结束，避免 main 提前退出
      for handle in handles {
          handle.join().unwrap();
      }
  }
hint: "读写锁适合读多写少。复杂并发场景可以考虑 crossbeam 或 tokio 提供的高级抽象。"
exercises:
  - title: "RwLock 读多写少"
    description: "用 Arc<RwLock<Vec<i32>>> 实现多个读线程和一个写线程共享数据。"
    code_template: |
      use std::sync::{Arc, RwLock};
      use std::thread;

      fn main() {
          let data = Arc::new(RwLock::new(vec![1, 2, 3]));
          // 启动读线程和写线程
          // 记得 join
      }
---

# 🦀 并发模式与设计 channel

并发是 Rust 的强项之一。所有权、借用检查和类型系统共同构建了一套「编译期并发正确性」的防线。本章介绍几种在 Rust 中常见且被验证有效的并发设计模式。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 🎯 为什么 Rust 并发更安全

传统语言靠程序员手动加锁、小心传递指针；Rust 则用所有权把「数据竞争」变成了编译错误：同一时刻要么只有一个可变引用，要么有多个不可变引用——这个规则天然适用于并发。

## 🎭 Actor 模型

每个 actor 是独立的计算单元，不共享内存，只通过消息通信。Rust 的所有权模型与 actor 非常契合：你把数据所有权发送出去，接收方独占处理，不会有数据竞争。

典型实现：`actix`、`tokio::sync::mpsc` + 任务。

## 🏭 Pipeline 管道

把任务拆成多个阶段，阶段之间用 channel 连接，数据像流水线一样传递。每个阶段可以并发运行，适合数据流处理、ETL、日志处理等场景。

## ⚙️ 工作窃取 Work Stealing

tokio 等运行时内部使用工作窃取调度器。每个 worker 线程有自己的任务队列；当自己队列空了，就从其他线程「偷」任务执行，自动在多核间平衡负载。

![Rust 并发模式地图](/images/module2-concurrent-patterns.svg)

## 🔒 读写锁 RwLock

`RwLock<T>` 允许多个读取者同时访问，或一个写入者独占访问，适合**读多写少**的场景。相比 `Mutex`，它能显著减少读竞争。

![RwLock 状态示意](/images/module2-rwlock.svg)

## 🚀 无锁数据结构

在极端性能场景下，可以使用原子操作和精心设计的内存顺序实现无锁并发：`crossbeam` 提供了 channel、epoch 内存管理、原子栈/队列等工具。

## ✅ 背压 Backpressure

当生产者速度持续快于消费者时，内存会无限增长。背压机制让生产者在通道满时阻塞或丢弃数据。有界 channel 是最常见的背压实现。

## ⚠️ 常见误区

- `Mutex` 不是银弹，死锁仍可能发生
- 锁的粒度太大会降低并发度，太小会增加开销
- `async` 中避免使用阻塞锁，优先使用 `tokio::sync::Mutex`
- 不要为了在 channel 中共享而过度 `clone` 大数据

## 💡 一句话总结

Rust 并发设计的精髓是：用所有权避免数据竞争，用 channel 划分边界，用合适的锁或无锁结构匹配场景，用背压控制系统负载。

<RustPlayground />
