---
title: "网络编程基础 TcpListener / TcpStream"
module: "中等应用"
order: 16
code: |
  use std::io::{Read, Write};
  use std::net::{TcpListener, TcpStream};

  fn main() {
      let _listener = TcpListener::bind("127.0.0.1:8080");
      println!("TCP 监听已准备");

      match TcpStream::connect("127.0.0.1:8080") {
          Ok(mut stream) => {
              let msg = "Hello, TCP!";
              stream.write_all(msg.as_bytes()).unwrap();
              let mut buf = [0u8; 1024];
              let n = stream.read(&mut buf).unwrap();
              println!("收到 {} 字节", n);
          }
          Err(e) => {
              println!("连接失败: {}（服务器可能未启动）", e);
          }
      }
  }
hint: "TcpListener::bind 和 TcpStream::connect 都返回 Result，需要错误处理。"
exercises:
  - title: "TCP Echo 服务端"
    description: "编写一个简单的 echo server，读取客户端发来的数据并原样返回。"
    code_template: |
      use std::io::{Read, Write};
      use std::net::TcpListener;

      fn main() {
          let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
          for stream in listener.incoming() {
              // let mut stream = stream.unwrap();
          }
      }
---

# 网络编程基础 TcpListener / TcpStream 🦀

网络编程是理解 Web 后端、微服务、分布式系统的基础。Rust 标准库的 `std::net` 模块提供了零依赖、零成本抽象的 TCP 通信能力，是迈向 tokio/axum 异步网络的第一步。

## 从生活类比开始 💡

网络通信就像打电话：一方拨号（connect），一方等铃响（bind + listen）。接通后，双方轮流说话（read / write），说完挂断（drop）。

![TCP 通信流程](/images/module1-networking-tcp.svg)

## 深入讲解

### TcpListener

绑定地址并监听连接：
```rust
let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
```

### TcpStream

- 客户端：`TcpStream::connect("127.0.0.1:8080")`
- 服务端：`listener.accept()` 返回 `(TcpStream, SocketAddr)`

### 读写数据

- `stream.read(&mut buf)` / `stream.write(&buf)`
- 配合 `BufReader` / `BufWriter` 提高效率

## 一句话总结 🦀

> std::net 提供零成本抽象的 TCP 通信，是理解异步网络框架（tokio/axum）的基石。

<RustPlayground />
