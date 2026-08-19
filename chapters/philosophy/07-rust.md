---
title: "大模型时代下的 Rust 机遇"
module: "Rust 哲学"
order: 7
code: |
  // 用纯 std 实现一个简化的向量相似度计算，
  // 这是向量数据库和语义检索中最基础的操作之一。

  fn dot_product(a: &[f64], b: &[f64]) -> f64 {
      // zip 把两个向量逐项配对，对应维度相乘后求和
      a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
  }

  // 向量模长（L2 范数）：各分量平方和开根号
  fn magnitude(v: &[f64]) -> f64 {
      v.iter().map(|x| x * x).sum::<f64>().sqrt()
  }

  // 余弦相似度：夹角越小值越接近 1，表示语义越相关
  fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
      let dot = dot_product(a, b);
      let mag_a = magnitude(a);
      let mag_b = magnitude(b);
      // 零向量没有方向，约定返回 0 以避免除零
      if mag_a == 0.0 || mag_b == 0.0 {
          0.0
      } else {
          dot / (mag_a * mag_b)
      }
  }

  fn main() {
      // 模拟两个文本片段的 embedding 向量（简化版）
      let query = vec![0.8, 0.2, 0.1, 0.4];
      let doc1 = vec![0.7, 0.3, 0.0, 0.5];
      let doc2 = vec![0.0, 0.9, 0.8, 0.1];

      let sim1 = cosine_similarity(&query, &doc1);
      let sim2 = cosine_similarity(&query, &doc2);

      println!("query 与 doc1 的余弦相似度: {:.4}", sim1);
      println!("query 与 doc2 的余弦相似度: {:.4}", sim2);

      if sim1 > sim2 {
          println!("doc1 与 query 更相关");
      } else {
          println!("doc2 与 query 更相关");
      }
  }
hint: "大模型基础设施对性能和内存安全要求极高，Rust 在推理、向量数据库、数据管道和边缘 AI 中都有很好的切入点。"
exercises:
  - title: "计算余弦相似度"
    description: "实现 dot_product 和 cosine_similarity，计算两个简单向量的相似度。"
    code_template: |
      fn dot_product(a: &[f64], b: &[f64]) -> f64 {
          a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
      }

      fn main() {
          let a = [1.0, 0.0];
          let b = [0.0, 1.0];
          println!("{}", dot_product(&a, &b));
      }
---

# 大模型时代下的 Rust 机遇 🤖

大模型（LLM）正在重塑软件行业。当模型参数量达到千亿级、上下文窗口达到百万级时，**性能、并发、内存安全和部署成本** 成了基础设施团队的核心痛点。而这正是 Rust 的主场。

## 为什么大模型需要 Rust？

- **性能**：推理引擎、向量数据库、数据预处理都依赖极致的计算与内存效率。
- **并发**：模型服务需要同时处理成千上万的长文本请求，Rust 的无数据竞争并发模型天然适合。
- **安全**：AI 基础设施一旦崩溃，影响的是整个业务线。Rust 在编译期消灭空指针、悬垂引用和数据竞争。
- **部署**：单文件二进制、小体积、可静态链接，非常适合容器化和边缘部署。

## 已经在发生的 Rust 机遇

| 场景 | Rust 生态/项目 | 价值 |
|------|---------------|------|
| 模型推理 | `candle`、`tract` | 轻量级、可嵌入的推理运行时 |
| 向量数据库 | `Qdrant`（Rust 核心）、`tantivy` | 低延迟语义检索 |
| 分词器 | `tokenizers`（Rust 实现） | 与 Hugging Face 生态兼容的高性能分词 |
| 数据管道 | `Polars`、`DataFusion` | 高性能特征工程与 ETL |
| WebAssembly | `wasm-bindgen` | 把推理逻辑安全地跑在浏览器/边缘节点 |
| Agent 基础设施 | `tokio`、`axum` | 高并发模型网关、函数调用代理 |


![Rust 在 AI 基础设施中的位置](/images/module3-ai-infra-map.svg)

## Rust 在 AI 领域的独特优势

1. **推理侧性能** 🚀：Rust 的零成本抽象让张量运算、KV Cache 管理、批量解码可以接近 C/C++ 的性能，同时避免内存安全漏洞。
2. **边缘与浏览器** 🌐：通过 WASM，Rust 可以把分词、向量计算甚至小型模型推理带到浏览器和 IoT 设备。
3. **数据基础设施** 📊：Polars/DataFusion 用 Rust 实现了比 pandas 更快的 DataFrame 引擎，适合处理训练语料和检索数据。
4. **AI Agent 工具链** 🛠️：Rust 的类型系统能严格约束函数调用的参数结构，减少 LLM Agent 调用外部工具时的类型错误。


![四大优势 × AI 场景](/images/module3-ai-four-strengths.svg)

## 一句话总结 ✅

大模型时代不仅需要更大的 GPU，也需要更高效的 CPU 侧基础设施；Rust 以性能、安全和并发优势，正在成为 AI 基础设施的关键语言。

<RustPlayground />
