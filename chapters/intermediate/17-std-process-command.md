---
title: "进程管理 std::process::Command"
module: "中等应用"
order: 17
code: |
  use std::process::Command;

  fn main() {
      let output = Command::new("rustc")
          .arg("--version")
          .output()
          .expect("找不到 rustc");

      if output.status.success() {
          let version = String::from_utf8_lossy(&output.stdout);
          println!("Rust 版本: {}", version.trim());
      } else {
          let err = String::from_utf8_lossy(&output.stderr);
          println!("错误: {}", err);
      }

      let mut child = Command::new("sleep")
          .arg("2")
          .spawn()
          .expect("spawn 失败");

      println!("子进程已启动，PID: {:?}", child.id());
      let status = child.wait().expect("wait 失败");
      println!("子进程退出: {}", status);
  }
hint: "Command::new + .arg() 链式配置。spawn 异步，output 同步等待完成。"
exercises:
  - title: "获取当前工作目录"
    description: "用 Command::new(\"pwd\") 执行命令并打印输出。"
    code_template: |
      fn main() {
          let output = Command::new("pwd").output().unwrap();
          println!("{}", String::from_utf8_lossy(&output.stdout));
      }
---

# 进程管理 std::process::Command 🦀

很多工具需要调用外部程序：Git、Python 脚本、系统命令。Rust 标准库的 `std::process` 模块提供了安全、灵活的进程管理能力，无需任何外部依赖。

## 从生活类比开始 💡

`Command` 就像你给助理的指令清单：你想让助理帮你执行一个程序、传参数、指定环境变量，然后选择是盯着做完（output）还是先忙别的（spawn）。

![Command 调用外部进程](/images/module1-process-command.svg)

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

<RustPlayground />
