---
title: "定时器与时间处理"
module: "中等应用"
order: 19
code: |
  use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

  fn main() {
      let start = Instant::now();
      for _ in 0..1_000_000 { let _ = 1 + 1; }
      println!("耗时: {:?}", start.elapsed());

      let now = SystemTime::now();
      let ts = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
      println!("当前时间戳: {}", ts);

      std::thread::sleep(Duration::from_millis(500));
      println!("已睡眠 500ms");
  }
hint: "测量耗时用 Instant，获取当前时间用 SystemTime。异步环境用 tokio::time::sleep。"
exercises:
  - title: "计时循环"
    description: "用 Instant 测量 1..=1_000_000 循环的耗时并打印。"
    code_template: |
      use std::time::Instant;

      fn main() {
          let start = Instant::now();
          let mut sum = 0u64;
          for i in 1..=1_000_000 { sum += i; }
          println!("sum={} time={:?}", sum, start.elapsed());
      }
---

# 定时器与时间处理 🦀

处理时间戳、计算耗时、延迟执行、定时任务——这些需求在 CLI 工具、Web 服务和测试中无处不在。Rust 标准库的 `std::time` 已经覆盖了大部分场景，tokio 等异步运行时提供了更丰富的定时能力。

## 从生活类比开始 💡

时间就像河里的秒表：你可以问「现在几点」（SystemTime），可以按「过去多久」（Duration），可以在「X 毫秒后响铃」（sleep / timeout）。

![时间 API 速览](/images/module1-timer-time.svg)

## 深入讲解

### std::time::Instant

高精度单调时钟，适合测量耗时：
```rust
let start = Instant::now();
let elapsed = start.elapsed();
```

### std::time::SystemTime

获取当前系统时间，可转换为 `UNIX_EPOCH` 时间戳。

### Duration

表示时间段，支持 `as_secs()`、`as_millis()`、`as_nanos()` 等转换。

### sleep

阻塞当前线程：`std::thread::sleep(Duration::from_secs(1));`

### tokio::time::sleep

异步非阻塞：`tokio::time::sleep(Duration::from_secs(1)).await;`

## 一句话总结 🦀

> std::time 提供耗时测量和系统时间，async 场景交给 tokio::time 做非阻塞定时和超时控制。

<RustPlayground />
