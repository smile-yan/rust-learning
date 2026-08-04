---
title: 性能优化与 Benchmark criterion
hint: 性能优化前先 benchmark。算法优化通常比语言层面的微优化收益更大。
---

# ⚡ 性能优化与 Benchmark criterion

Rust 默认已经很快，但「快」不等于「足够快」。在关键路径上，我们需要数据驱动地测量、分析、优化，而不是凭感觉改代码。

## 🎯 测量先于优化

这是性能调优的第一原则。没有 benchmark 的优化往往是负优化：你可能把大部分时间花在根本不热的代码上，或者为了微优化牺牲可读性。

## 🦀 Criterion

`criterion` 是 Rust 生态最流行的 benchmark 库，提供：
- 统计分析（均值、中位数、标准差）
- 多次采样减少噪声
- 友好的 HTML 报告
- 自动检测性能回退

使用方法：在 `benches/` 目录写 bench 文件，运行 `cargo bench`。

## ⚙️ 常见优化方向

- **减少分配**：复用 `Vec`，使用 `String::with_capacity`
- **迭代器**：通常比手写循环更快，且更易读
- **预分配**：`Vec::with_capacity` 避免多次扩容
- **算法优化**：往往比语言层面优化收益更大
- **编译配置**：调整 release profile
- **缓存友好**：减少内存跳跃，提高局部性
- **SIMD 向量化**：使用 `std::simd` 或 `packed_simd` 做批量计算
- **谨慎使用 unsafe**：在已验证的热点做极限优化

## ✅ 编译优化配置

在 `Cargo.toml` 中：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

`lto` 开启链接时优化，`codegen-units = 1` 牺牲编译时间换取更激进的优化，`panic = "abort"` 可以减少二进制体积。

## 🔥 火焰图与 Profiling

使用 `cargo flamegraph` 可以生成火焰图，直观看到时间花在哪些函数调用栈上，快速定位热点。配合 `perf`、` Instruments`、`heaptrack` 等工具，可以分析 CPU、内存、I/O 瓶颈。

## 🧪 测量与验证

优化的闭环：
1. 用 Criterion 建立基准
2. 修改代码并重新 benchmark
3. 对比统计结果，确认改进显著
4. 回退不生效的优化，保持代码简洁

## ⚠️ 不要过早优化

先写出正确、清晰的代码，再用 benchmark 找到真正的瓶颈。优化要针对测量结果，而不是想象中的慢。

## 💡 一句话总结

性能优化 = Criterion 测量 → 火焰图定位 → 算法/分配/缓存/编译优化 → 再次测量验证；没有数据支撑的优化只是玄学。

