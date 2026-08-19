---
title: "闭包与迭代器 closure"
module: "中等应用"
order: 4
code: |
  fn main() {
      // 闭包：参数和返回类型可省略，类型由第一次调用推断并固定
      let add_one = |x| x + 1;
      println!("5 + 1 = {}", add_one(5));

      let offset = 10;
      let add_offset = |x| x + offset;  // 捕获环境变量
      println!("5 + offset = {}", add_offset(5));

      let v = vec![1, 2, 3, 4, 5];

      // 迭代器适配器是惰性的：map/filter 只是组装，
      // sum() 才真正消费求值
      let sum_of_squares: i32 = v
          .iter()
          .map(|x| x * x)
          .filter(|x| *x > 5)
          .sum();
      println!("大于 5 的平方和: {}", sum_of_squares);

      let words = vec!["hello", "rust", "world"];
      // collect 把迭代器收集成集合，目标类型由 Vec<String> 标注决定
      let upper: Vec<String> = words
          .iter()
          .map(|w| w.to_uppercase())
          .collect();
      println!("{:?}", upper);

      // fold 手动折叠迭代器：acc 是累积值，这里用来求最大值
      let max = v.iter()
          .fold(0, |acc, x| if *x > acc { *x } else { acc });
      println!("最大值: {}", max);

      // 自定义简单的迭代器
      let mut counter = Counter::new();
      println!("计数器: {:?}", counter.next());
      println!("计数器: {:?}", counter.next());
  }

  struct Counter {
      count: u32,
  }

  impl Counter {
      fn new() -> Counter {
          Counter { count: 0 }
      }
  }

  // 只要实现 Iterator trait，自定义类型就能接入整个迭代器生态
  impl Iterator for Counter {
      // 关联类型：声明每次迭代产出的元素类型
      type Item = u32;

      // next 返回 Some 表示还有值，返回 None 时迭代结束
      fn next(&mut self) -> Option<Self::Item> {
          if self.count < 5 {
              self.count += 1;
              Some(self.count)
          } else {
              None
          }
      }
  }
hint: "闭包可以根据使用情况自动推断参数和返回类型。自定义迭代器只需实现 Iterator trait 的 next 方法。"
exercises:
  - title: "用闭包翻倍列表"
    description: "用 iter().map(...).collect() 把 Vec<i32> 中每个元素翻倍。"
    code_template: |
      fn main() {
          let v = vec![1, 2, 3];
          // let doubled: Vec<i32> = ...
          println!("{:?}", doubled);
      }
  - title: "捕获环境变量"
    description: "定义外部变量 factor，用闭包把 Vec 中每个元素乘以 factor。"
    code_template: |
      fn main() {
          let factor = 10;
          let nums = vec![1, 2, 3];
          // let scaled: Vec<i32> = nums.iter().map(|x| x * factor).collect();
          println!("{:?}", scaled);
      }
---

# 闭包与迭代器 closure 🦀

闭包（Closure）是可以捕获其所在环境变量的匿名函数。迭代器（Iterator）提供了一种惰性处理集合元素的方式。两者结合，构成了 Rust 函数式编程风格的核心。

## 从生活类比开始 💡

想象你正在做一个三明治：你可以提前切好所有食材（传统循环），也可以只在需要时一片一片地准备（迭代器）。闭包则像是一个可以随身携带食材的“移动小厨房”，走到哪里都能用。

## 概念图解

![闭包捕获环境变量](/images/module1-closure-capture.svg)

闭包 `add_offset` 捕获了外部的 `offset` 变量，调用时可以直接使用环境中的数据。

![迭代器惰性求值链](/images/module1-iterator-lazy.svg)

迭代器链 `iter -> map -> filter -> sum` 只有在最终消费（如 `sum()`）时才会真正执行，中间步骤不会分配新的集合。

## 深入讲解

### 闭包

```rust
let add_one = |x| x + 1;
```

闭包可以根据使用情况自动推断参数和返回类型。闭包捕获环境变量的方式有三种：

- `Fn`：不可变借用捕获 ✅
- `FnMut`：可变借用捕获 🔧
- `FnOnce`：获取所有权 ⚠️

### 迭代器

迭代器是惰性的，只有在调用 `next()` 或消费方法（如 `sum`、`collect`）时才会真正计算。

常用适配器：

- `map`：对每个元素转换
- `filter`：按条件过滤
- `fold`：聚合
- `collect`：收集到新集合

### 自定义迭代器

只需实现 `Iterator` trait：

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

## 常见误区 ⚠️

- **误区 1**：闭包和普通函数完全一样。  
  ✅ 正解：闭包可以捕获环境，函数不能。
- **误区 2**：迭代器链会创建很多中间集合。  
  ✅ 正解：迭代器是惰性的，通常不会分配中间集合。
- **误区 3**：所有闭包都实现 `Fn`、`FnMut`、`FnOnce`。  
  ✅ 正解：根据捕获方式，闭包至少实现其中一个。

## 一句话总结 🦀

> 闭包把环境“打包”带走，迭代器把计算“延迟”到需要时——两者让 Rust 代码既简洁又高效。

<RustPlayground />
