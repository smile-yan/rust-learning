---
title: 进程管理 std::process::Command
hint: Command::new + .arg() 链式配置。spawn 异步，output 同步等待完成。
---

# 进程管理 std::process::Command 🦀

很多工具需要调用外部程序：Git、Python 脚本、系统命令。Rust 标准库的 `std::process` 模块提供了安全、灵活的进程管理能力，无需任何外部依赖。

## 从生活类比开始 💡

`Command` 就像你给助理的指令清单：你想让助理帮你执行一个程序、传参数、指定环境变量，然后选择是盯着做完（output）还是先忙别的（spawn）。

![Command 调用外部进程](images/module1-process-command.svg)

## 深入讲解

### Command::new

设置要执行的程序名称，然后链式 `.arg()` 添加参数、`.env()` 设置环境变量。

### spawn vs output

- `spawn()`：异步启动子进程，立即返回 `Child`
- `output()`：等待子进程完成，一次性返回 `Output`

### 管道

父进程可以用 `stdin()` 向子进程写入数据，子进程的 stdout 可被父进程读取。

## 一句话总结 🦀

> std::process::Command 让 Rust 安全地创建、管理和与子进程交互，是构建 CLI 工具和工作流的基础。

