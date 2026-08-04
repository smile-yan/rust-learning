---
title: 更多集合类型 HashMap / HashSet / BTreeMap
hint: HashMap 平均 O(1) 查找，BTreeMap 有序但 O(log n)。按场景选择。
---

# 更多集合类型 HashMap / HashSet / BTreeMap 🦀

`Vec` 是最常用的集合，但真实场景中还有许多其他数据结构各司其职。Rust 标准库提供了 `HashMap`、`HashSet`、`BTreeMap`、`BTreeSet`、`BinaryHeap` 等集合，覆盖了去重、检索、排序、优先级队列等常见需求。

## 从生活类比开始 💡

Vec 是一排按顺序排队的储物柜，HashMap 是电话簿（按名字找号码），HashSet 是点名册（只关心谁来了），BTreeMap 是按字母顺序排列的单词表。

![集合类型对比](images/module1-more-collections.svg)

## 深入讲解

### HashMap<K, V>

- 键值对映射，查找平均 O(1)
- 键必须实现 `Eq` + `Hash`
- 遍历顺序不保证固定

### HashSet<T>

- 基于 HashMap，只存键不存值
- 快速去重、成员检查

### BTreeMap<K, V> / BTreeSet<T>

- 基于 B 树，按键排序
- 查找 O(log n)
- 稳定迭代顺序

## 一句话总结 🦀

> 集合是数据结构的宝库：Vec 管顺序，HashMap 管查找，HashSet 管去重，BTreeMap 管有序。

