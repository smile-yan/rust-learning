---
title: 迭代器 iter
hint: 迭代器是惰性的，只有调用消费方法（如 collect、sum）时才会真正执行。filter 中的闭包接收的是 &&i32，需要根据 iter() 的类型正确解引用。
---

# 迭代器 iter

迭代器（iterator）是 Rust 中遍历集合元素的标准方式。它抽象了“逐个产生值”的过程，支持链式调用 `map`、`filter`、`collect` 等高阶方法，代码既简洁又高效。Rust 的迭代器还经常能被编译器优化到与手写循环一样快。

## 引入：自动售货机 🥤

迭代器就像一台自动售货机：你按一下按钮，它吐出一个商品；按完所有商品后，它就停止了。你不需要关心里面具体怎么存放，只需要知道它会按顺序给你东西。

![迭代器示意图](images/module0-iterators.svg)

## 概念图解 💡

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter()
    .map(|x| x * 2)
    .collect();
```

## 深入讲解

### 创建迭代器

- `.iter()`：对元素不可变引用进行迭代，不转移所有权
- `.iter_mut()`：对元素可变引用进行迭代
- `.into_iter()`：获取元素所有权进行迭代

### 常用适配器

- `map`：对每个元素进行转换
- `filter`：按条件过滤元素
- `fold`：累积计算
- `collect`：把迭代结果收集成集合
- `sum` / `count` / `max` / `min`：聚合操作

```rust
let sum: i32 = (1..=100).sum();
let evens: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
```

### 惰性求值

迭代器适配器是**惰性**的：只有遇到消费型方法（如 `collect`、`sum`、`for_each`）时，才会真正执行。这种特性让链式操作非常高效。

### 自定义迭代器

实现 `Iterator` trait 就可以创建自己的迭代器，只需提供 `next` 方法：

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}
```

### 迭代器 vs 索引循环

Rust 官方推荐优先使用迭代器，因为它更安全、更简洁，而且通常性能更好。

## 常见误区 ⚠️

- 只调用适配器不消费，结果没有执行。
- 在 `iter()` 迭代时修改集合，导致借用冲突。
- 混淆 `iter()`、`iter_mut()` 和 `into_iter()` 的所有权语义。
- 认为迭代器比索引循环慢：Rust 迭代器通常会被零成本抽象优化。

## 一句话总结 ✅

Rust 迭代器惰性求值、链式组合；理解 `.iter()`、`.iter_mut()`、`.into_iter()` 和消费型方法，是写出优雅、高效 Rust 代码的关键。

