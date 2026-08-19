---
title: "更多集合类型 HashMap / HashSet / BTreeMap"
module: "中等应用"
order: 14
code: |
  use std::collections::{HashMap, HashSet, BTreeMap};

  fn main() {
      // HashMap：词频统计
      let text = "hello rust hello world";
      let mut map = HashMap::new();
      for word in text.split_whitespace() {
          *map.entry(word).or_insert(0) += 1;
      }
      println!("词频: {:?}", map);

      // HashSet：去重
      let nums = vec![1, 2, 2, 3, 3, 3];
      let unique: HashSet<_> = nums.iter().collect();
      println!("去重: {:?}", unique);

      // BTreeMap：按键排序
      let mut tree = BTreeMap::new();
      tree.insert("zebra", 3);
      tree.insert("apple", 1);
      tree.insert("mango", 2);
      println!("有序: {:?}", tree);
  }
hint: "HashMap 平均 O(1) 查找，BTreeMap 有序但 O(log n)。按场景选择。"
exercises:
  - title: "统计字符频率"
    description: "用 HashMap 统计 \"abracadabra\" 中每个字符的频率。"
    code_template: |
      fn main() {
          let s = "abracadabra";
          let mut freq = std::collections::HashMap::new();
          // for c in s.chars() { ... }
      }
  - title: "集合交集"
    description: "给定两个 HashSet，打印它们的交集。"
    code_template: |
      fn main() {
          let a: HashSet<_> = [1,2,3].iter().collect();
          let b: HashSet<_> = [2,3,4].iter().collect();
          // let inter: Vec<_> = a.intersection(&b).collect();
      }
---

# 更多集合类型 HashMap / HashSet / BTreeMap 🦀

`Vec` 是最常用的集合，但真实场景中还有许多其他数据结构各司其职。Rust 标准库提供了 `HashMap`、`HashSet`、`BTreeMap`、`BTreeSet`、`BinaryHeap` 等集合，覆盖了去重、检索、排序、优先级队列等常见需求。

## 从生活类比开始 💡

Vec 是一排按顺序排队的储物柜，HashMap 是电话簿（按名字找号码），HashSet 是点名册（只关心谁来了），BTreeMap 是按字母顺序排列的单词表。

![集合类型对比](/images/module1-more-collections.svg)

## 深入讲解

### HashMap\<K, V>

- 键值对映射，查找平均 O(1)
- 键必须实现 `Eq` + `Hash`
- 遍历顺序不保证固定

### HashSet\<T>

- 基于 HashMap，只存键不存值
- 快速去重、成员检查

### BTreeMap\<K, V> / BTreeSet\<T>

- 基于 B 树，按键排序
- 查找 O(log n)
- 稳定迭代顺序

## 一句话总结 🦀

> 集合是数据结构的宝库：Vec 管顺序，HashMap 管查找，HashSet 管去重，BTreeMap 管有序。

<RustPlayground />
