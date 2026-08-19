---
title: "日志系统 log / tracing"
module: "中等应用"
order: 12
code: |
  fn main() {
      env_logger::init();
      log::info!("应用启动");
      log::warn!("磁盘空间不足");
      log::error!("数据库连接失败");

      tracing::info!(
          user_id = 42,
          action = "login",
          "用户登录成功"
      );
  }
hint: "log 是 facade，需要配合 env_logger 等 subscriber 输出。生产环境推荐 tracing 生态。"
exercises:
  - title: "添加不同级别日志"
    description: "用 log::info! / warn! / error! 输出不同级别的日志。"
    code_template: |
      fn main() {
          env_logger::init();
          // log::info!(...);
      }
---

# 日志系统 log / tracing 🦀

任何有生命周期的应用都需要日志：排查 bug、追踪请求、监控异常。Rust 提供了分层、可插拔的日志生态：`log` 是轻量级 facade，`tracing` 是结构化可观测方案。

## 从生活类比开始 💡

日志就像飞机的黑匣子：正常时你不觉得它重要，出问题时它能告诉你每一秒发生了什么。

![日志分层架构](/images/module1-log-tracing.svg)

## 深入讲解

### log facade

`log` crate 定义了一套统一的宏接口：`trace!`、`debug!`、`info!`、`warn!`、`error!`。应用代码调用宏，实际输出由 subscriber 决定。

### tracing

`tracing` 提供 `span`（一段有开始和结束的操作）和 `event`（事件点），支持结构化字段（key-value）和异步上下文传播。

### 常用组合

- `log` + `env_logger`：简单快速启用
- `tracing` + `tracing_subscriber`：生产级可观测

## 一句话总结 🦀

> 日志是把程序运行过程「录音」的能力；log 是轻量 facade，tracing 是结构化可观测。

<RustPlayground />
