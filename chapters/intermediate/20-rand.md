---
title: "随机数与哈希 rand / 哈希函数"
module: "中等应用"
order: 20
code: |
  use rand::Rng;

  fn main() {
      let mut rng = rand::thread_rng();
      println!("随机数: {}", rng.gen_range(1..100));
      println!("布尔: {}", rng.gen_bool(0.5));

      let chars: String = std::iter::repeat(|| {
          rng.sample(rand::distributions::Alphabetic)
      }).take(8).collect();
      println!("随机字符串: {}", chars);
  }
hint: "thread_rng 是线程本地的，性能好。测试时用 SeedableRng 保证结果可复现。"
exercises:
  - title: "生成随机密码"
    description: "生成一个 8 位的随机字母数字字符串。"
    code_template: |
      fn main() {
          let mut rng = rand::thread_rng();
          // let password: String = ...;
      }
---

# 随机数与哈希 rand / 哈希函数 🦀

随机数和哈希函数在游戏中、安全中和数据结构中都有广泛应用。Rust 的 `rand` crate 提供了安全、高质量的随机数生成，标准库的 `std::hash` 提供了通用哈希 trait，第三方的 `blake3` / `sha2` 则覆盖密码学场景。

## 从生活类比开始 💡

掷骰子就是「真随机」：每次结果不可预测。哈希函数则像指纹机：无论输入多大，输出固定长度，而且输入微变输出大变。

![随机数生成与哈希](/images/module1-random-hash.svg)

## 深入讲解

### rand crate

- `thread_rng()`：线程本地随机数生成器
- `gen()`：生成指定范围的随机值
- `SeedableRng`：固定种子的可重复生成器（适合测试）

### 哈希函数

- `std::hash::Hash`：通用哈希 trait
- `std::collections::hash_map::DefaultHasher`：快速非加密哈希
- `sha2` / `blake3`：密码学/高速哈希

### HashMap 的随机性

Rust 的 HashMap 使用 `SipHash`（密钥哈希），随机化种子可防御哈希碰撞攻击。

## 一句话总结 🦀

> rand 提供安全随机数，std::hash 提供通用哈希 trait，密码学场景用 blake3 / sha2。

<RustPlayground />
