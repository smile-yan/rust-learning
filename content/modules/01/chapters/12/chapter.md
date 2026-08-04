---
title: 日志系统 log / tracing
hint: log 是 facade，需要配合 env_logger 等 subscriber 输出。生产环境推荐 tracing 生态。
---

# 日志系统 log / tracing 🦀

任何有生命周期的应用都需要日志：排查 bug、追踪请求、监控异常。Rust 提供了分层、可插拔的日志生态：`log` 是轻量级 facade，`tracing` 是结构化可观测方案。

## 从生活类比开始 💡

日志就像飞机的黑匣子：正常时你不觉得它重要，出问题时它能告诉你每一秒发生了什么。

![日志分层架构](images/module1-log-tracing.svg)

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

