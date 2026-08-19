---
title: "文件 I/O 与路径处理 fs / Path"
module: "中等应用"
order: 9
code: |
  use std::fs::{self, File};
  use std::io::{BufRead, BufReader, Write};
  use std::path::PathBuf;

  fn main() {
      // 创建临时目录和文件
      // PathBuf 是可变的自有路径类型，跨平台处理路径分隔符
      let dir = PathBuf::from("tmp_data");
      // create_dir_all 递归创建目录；
      // unwrap_or_default 忽略「目录已存在」这类错误
      fs::create_dir_all(&dir).unwrap_or_default();

      // join 拼接路径，比字符串拼接更安全且跨平台
      let file_path = dir.join("example.txt");
      fs::write(&file_path, "Hello, Rust!\nLine 2").unwrap();

      // 一次性读取
      let content = fs::read_to_string(&file_path).unwrap();
      println!("文件内容:\n{}", content);

      // 带缓冲逐行读取
      // BufReader 减少系统调用次数，适合逐行处理大文件
      let file = File::open(&file_path).unwrap();
      let reader = BufReader::new(file);
      for line in reader.lines() {
          println!("读取行: {}", line.unwrap());
      }

      // 路径操作
      // set_extension 是替换（而非追加）扩展名
      let mut new_path = file_path.clone();
      new_path.set_extension("log");
      println!("新路径: {:?}", new_path);

      // 清理
      fs::remove_file(&file_path).unwrap_or_default();
      fs::remove_dir(&dir).unwrap_or_default();
  }
hint: "文件 I/O 操作容易失败，记得处理 Result。PathBuf 提供安全、跨平台的路径操作。"
exercises:
  - title: "读取文件内容"
    description: "用 fs::read_to_string 读取一个文本文件并打印其长度。"
    code_template: |
      use std::fs;

      fn main() {
          // let content = fs::read_to_string("hello.txt").unwrap();
          // println!("{} bytes", content.len());
      }
---

# 文件 I/O 与路径处理 fs / Path 🦀

文件操作是几乎所有程序都需要的能力。Rust 的标准库提供了安全、跨平台的文件 I/O API，同时通过 `Result` 强迫你处理可能发生的错误。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 从生活类比开始 💡

读写文件就像寄信：你需要知道地址（路径）、信件内容（数据），还要处理可能寄丢或地址错误的情况（错误处理）。Rust 不允许你忽略这些“可能出错”的环节。

## 深入讲解

### 读写文件

- `std::fs::read_to_string`：一次性读取整个文件为字符串。
- `std::fs::write`：一次性写入字符串到文件。
- `File::open` / `File::create`：打开或创建文件，返回 `Result`。
- `BufReader`：带缓冲的逐行读取，适合大文件。

### 路径处理

`PathBuf` 是可拥有的路径类型，`Path` 是借用类型。它们提供了跨平台的路径拼接、扩展名修改等方法：

```rust
let mut p = PathBuf::from("data");
p.push("example.txt");
p.set_extension("log");
```

### 错误处理

文件 I/O 操作容易失败，因此大多数 API 返回 `Result`。推荐使用 `?` 传播错误，或在示例中使用 `unwrap()` 并理解其风险。

### 缓冲与性能

对于大文件，使用 `BufReader` 和 `BufWriter` 可以显著减少系统调用次数，提高 I/O 性能。

### 目录操作

```rust
std::fs::create_dir_all("data/logs")?;
for entry in std::fs::read_dir("data")? {
    let entry = entry?;
    println!("{:?}", entry.path());
}
```

`read_dir` 返回目录中的条目迭代器，适合批量处理文件。

### 临时文件与清理

在测试或示例中，建议创建临时目录并在结束后清理，避免污染工作目录。`std::fs::remove_dir_all` 可以递归删除目录。

## 常见误区 ⚠️

- **误区 1**：`unwrap()` 适合所有文件操作。  
  ✅ 正解：生产代码应使用 `?` 或 `match` 处理 `Result`。
- **误区 2**：路径字符串拼接可以跨平台。  
  ✅ 正解：应使用 `PathBuf` 和 `join`，避免手动处理分隔符。
- **误区 3**：`read_to_string` 适合所有文件。  
  ✅ 正解：大文件应使用 `BufReader` 逐行或分块读取，避免内存爆炸。

## 一句话总结 🦀

> Rust 的文件 I/O 用类型系统把“可能出错”变成“必须处理”，再配合 PathBuf 让路径操作既安全又跨平台。

<RustPlayground />
