---
title: "异步编程 async / await"
module: "高级应用"
order: 3
code: |
  use tokio::time::{sleep, Duration};

  // async fn 返回一个 Future：调用时不立即执行，被 await 才真正驱动
  async fn say_hello(name: &str, delay: u64) {
      // .await 挂起当前任务等待完成，期间不阻塞所在线程
      sleep(Duration::from_millis(delay)).await;
      println!("你好, {}!", name);
  }

  // #[tokio::main] 把 async main 包装成同步入口，并启动 tokio 运行时
  #[tokio::main]
  async fn main() {
      // join! 同时等待多个 Future
      let (r1, r2) = tokio::join!(
          say_hello("Alice", 100),
          say_hello("Bob", 50),
      );
      let _ = (r1, r2);

      // spawn 创建新任务
      let handle = tokio::spawn(async {
          say_hello("Charlie", 30).await;
      });
      // await JoinHandle 等待任务结束；任务 panic 时这里返回 Err
      handle.await.unwrap();

      println!("所有任务完成");
  }
hint: "async 函数返回 Future，需要 .await 或 executor 来驱动执行。本示例依赖 tokio 运行时，请复制到本地 cargo 项目（cargo add tokio --features full）中运行。"
exercises:
  - title: "async 函数与 await"
    description: "定义 async fn hello()，在 tokio main 中 await 它。"
    code_template: |
      #[tokio::main]
      async fn main() {
          hello().await;
      }

      async fn hello() {
          println!("Hello, async!");
      }
---

# ⏳ 异步编程 async / await

现代服务常常要同时处理成千上万的网络连接。为每个连接开一个操作系统线程成本太高，而异步编程让**单个线程能够并发推进多个任务**，是高并发 I/O 密集型应用的核心技术。Rust 的 `async/await` 在提供零成本抽象的同时，还保留了内存安全保证。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 🎯 类比：餐厅服务员与厨师

同步编程像厨师一次只做一道菜，做完才接下一单；异步编程像一名优秀的服务员，点完菜后不等菜做好，立刻去服务下一桌，等菜好了再回来上菜。线程还是那个线程，但利用率大幅提高。

## 🦀 Future 与 async/await

`async fn` 返回一个实现了 `Future` trait 的值。Future 代表一个**尚未完成的异步计算**，它不会立即执行，而是需要 executor（执行器）来驱动。

当你在某个 Future 上调用 `.await` 时：
- 如果已经完成，直接返回结果
- 如果还在等待，挂起当前任务，让出线程
- 当事件就绪后，executor 会唤醒任务继续执行

本质上，`async/await` 是编译器帮你把函数状态机化。

![Future 状态机](/images/module2-future-state-machine.svg)

## ⚙️ tokio 运行时

tokio 是 Rust 生态最流行的异步运行时，提供：
- 多线程任务调度（工作窃取）
- TCP/UDP 网络 I/O
- 定时器
- 通道（mpsc、oneshot、watch、broadcast）
- 文件 I/O 与阻塞任务池

![Tokio 运行时架构](/images/module2-tokio-runtime.svg)

## ✅ 并发执行手段

- `tokio::join!`：同时等待多个 Future，全部完成后一起返回
- `tokio::try_join!`：类似 join!，但任一失败即返回错误
- `tokio::spawn`：在运行时中创建新的独立任务
- `tokio::select!`：等待多个异步操作，哪个先完成就执行哪个分支

## ⚠️ 阻塞操作的危险

在异步任务中执行 CPU 密集型或阻塞操作会**卡住整个 worker 线程**，影响同线程上其他任务的调度。应使用 `tokio::task::spawn_blocking` 把阻塞工作放到独立线程池。

## 💡 一句话总结

Rust 异步编程 = `Future` 状态机 + `async/await` 语法糖 + `tokio` 运行时；善用 `join!`、`spawn`、`select!` 可以让单个线程高效并发，但务必远离阻塞操作。

<RustPlayground />
