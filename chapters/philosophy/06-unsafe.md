---
title: "实用主义：unsafe 的边界与责任"
module: "Rust 哲学"
order: 6
code: |
  // 安全包装层：内部使用 unsafe，但对外提供安全接口
  fn split_at_mut(
      slice: &mut [i32],
      mid: usize,
  ) -> (&mut [i32], &mut [i32]) {
      let len = slice.len();
      assert!(mid <= len, "mid 不能超过切片长度");

      // 返回两个不重叠的可变切片
      // 安全代码无法同时借出两个可变切片，
      // 这个安全抽象只能借助裸指针实现
      let ptr = slice.as_mut_ptr();
      // unsafe 块圈出编译器无法验证的操作；
      // 上面的 assert 保证两个切片不重叠
      unsafe {
          (
              std::slice::from_raw_parts_mut(ptr, mid),
              std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
          )
      }
  }

  fn main() {
      let mut numbers = [1, 2, 3, 4, 5];
      // 调用方完全处于安全代码中：unsafe 被封装在函数内部
      let (left, right) = split_at_mut(&mut numbers, 2);

      left[0] = 10;
      right[0] = 20;

      println!("left: {:?}", left);
      println!("right: {:?}", right);
      println!("原始数组: {:?}", numbers);
  }
hint: "unsafe 代码应该尽量封装在 safe API 内部，并清楚文档化其不变量。"
exercises:
  - title: "包装 unsafe 函数"
    description: "写一个 safe 包装函数，内部用 unsafe 调用 strlen。"
    code_template: |
      use std::os::raw::c_char;

      extern "C" {
          fn strlen(s: *const c_char) -> usize;
      }

      fn safe_len(s: &std::ffi::CStr) -> usize {
          unsafe { strlen(s.as_ptr()) }
      }

      fn main() {
          let s = std::ffi::CString::new("hello").unwrap();
          println!("{}", safe_len(&s));
      }
---

# 实用主义：unsafe 的边界与责任 ⚠️

Rust 的安全规则非常强大，但现实世界并不总是完美的。有时你必须与 C 代码交互、操作裸指针，或者做一些编译器无法验证的优化。Rust 没有逃避这个问题，而是提供了 `unsafe` 块，让你明确标出「这里我承担了安全责任」。

## unsafe 不是关闭所有检查

`unsafe` 只关闭了少数几项检查：
- 解引用裸指针
- 调用 unsafe 函数或方法
- 访问或修改可变静态变量
- 实现 unsafe trait
- 读写 union 的字段

它**不会**关闭：
- 借用检查
- 生命周期检查
- 类型检查


![unsafe 关闭了什么，没关什么](/images/module3-unsafe-scope.svg)

## 边界与责任

使用 `unsafe` 时，你必须手动保证：
- 裸指针有效
- 不创建数据竞争
- 不破坏不变量
- 调用约定正确

理想情况下，`unsafe` 代码应该被封装成 safe API，让调用者无需关心内部风险。


![unsafe：被围起来的责任区](/images/module3-unsafe-boundary.svg)

## 什么时候用 unsafe？

- 与 C/FFI 交互
- 实现某些数据结构（如自定义链表、环形缓冲区）
- 对热点代码做极限优化

## 不是越多越好

Rust 哲学鼓励把 `unsafe` 压缩到最小范围，并且用 safe 抽象包裹它。大多数 Rust 代码一辈子都不需要写 `unsafe`。

## 一句话总结 ✅

`unsafe` 是 Rust 的逃生舱，不是任意破坏规则的许可证。它的使用必须被明确标记、最小化范围，并封装成安全的 API。

<RustPlayground />
