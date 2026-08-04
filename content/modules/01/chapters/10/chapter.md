---
title: 正则表达式 regex
hint: Regex::new 编译后应复用。捕获组 caps[0] 是整个匹配，caps[1] 开始是分组。
---

# 正则表达式 regex 🦀

正则表达式是文本处理的瑞士军刀。Rust 的 `regex` crate 提供了基于**有限自动机**的高性能正则引擎，不需要回溯，因此天然免疫 ReDoS（正则拒绝服务攻击），同时保持 API 简洁易用。

## 从生活类比开始 💡

正则表达式就像在图书馆的书架上按索引找书：你不需要逐本翻阅（逐字符比较），只需要告诉管理员「找包含 'rust' 且以数字结尾的书」（模式），管理员会按规则快速定位。

![正则表达式工作流程](images/module1-regex-basics.svg)

## 深入讲解

### 基本用法

```rust
let re = Regex::new(r"\d+").unwrap();
```

### 常用方法

- `is_match`：判断字符串是否包含匹配
- `find`：返回第一个匹配及位置
- `captures`：提取捕获组
- `replace` / `replace_all`：替换匹配内容
- `split`：按分隔符切分字符串

### 捕获组

```rust
let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
for caps in re.captures_iter(text) {
    println!("用户: {}, 域名: {}, 后缀: {}", &caps[1], &caps[2], &caps[3]);
}
```

### 性能特点

`regex` crate 使用 **DFA（确定有限自动机）** 实现，保证线性时间复杂度和无回溯特性。

## 常见误区 ⚠️

- 匹配中文字符时忘记使用 `\p{Script=CJK}` 或 Unicode 模式
- 捕获组索引从 1 开始，0 是整个匹配
- 正则表达式不要过度复杂化

## 一句话总结 🦀

> `regex` 用有限自动机提供安全、线性时间、易用的文本匹配能力，是 Rust 文本处理的标配。

