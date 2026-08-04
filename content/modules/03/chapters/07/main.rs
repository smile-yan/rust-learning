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
