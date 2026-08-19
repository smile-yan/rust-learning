---
title: "智能指针 Box / Rc / RefCell"
module: "中等应用"
order: 5
code: |
  use std::rc::Rc;
  use std::cell::RefCell;

  fn main() {
      // Box 在堆上分配
      let b = Box::new(5);
      println!("b = {}", b);

      // Rc 共享所有权
      // Rc<RefCell<T>> 是单线程下「共享 + 可变」的经典组合：
      // Rc 提供多所有权，RefCell 提供内部可变性
      let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
      println!("引用计数: {}", Rc::strong_count(&shared));

      {
          // Rc::clone 只增加引用计数，不做深拷贝，开销很小
          let shared2 = Rc::clone(&shared);
          println!("clone 后引用计数: {}", Rc::strong_count(&shared));

          // borrow_mut 在运行时检查借用规则；
          // shared2 离开作用域后计数自动减一
          shared2.borrow_mut().push(4);
      }

      shared.borrow_mut().push(5);
      println!("共享数据: {:?}", shared.borrow());
      println!("最终引用计数: {}", Rc::strong_count(&shared));
  }
hint: "Rc 用于不可变共享，RefCell 提供内部可变性。Rc<T> 不是线程安全的，多线程请使用 Arc<T>。"
exercises:
  - title: "用 Box 装箱整数"
    description: "创建一个 Box<i32> 并打印其中值。"
    code_template: |
      fn main() {
          let b = Box::new(42);
          println!("{}", b);
      }
  - title: "Rc 共享 Vec"
    description: "用 Rc::new 共享一个 Vec，clone 两次并打印引用计数。"
    code_template: |
      use std::rc::Rc;

      fn main() {
          let v = Rc::new(vec![1, 2, 3]);
          let _v2 = Rc::clone(&v);
          let _v3 = Rc::clone(&v);
          println!("{}", Rc::strong_count(&v));
      }
---

# 智能指针 Box / Rc / RefCell 🦀

智能指针（Smart Pointer）是具有额外能力的结构体，通常实现了 `Deref` 和 `Drop` trait。它们的行为类似指针，但拥有更复杂的功能，如自动释放、共享所有权、运行时借用检查等。

## 从生活类比开始 💡

普通指针就像一把简单的钥匙，只能开门。智能指针则像一把带计数、自动收回、还能记录使用状态的“智能钥匙”——更安全、更省心。

## 概念图解

![Box<T> 堆内存单所有权](/images/module1-smartpointer-box.svg)

`Box<T>` 在堆上分配数据，栈上只保存指向堆的指针，离开作用域时自动释放堆内存。

![Rc<T> 多所有者共享](/images/module1-smartpointer-rc.svg)

`Rc<T>` 通过引用计数允许多个所有者共享同一份数据，当计数归零时自动释放。

![RefCell<T> 运行时借用规则](/images/module1-smartpointer-refcell.svg)

`RefCell<T>` 在运行时检查借用规则，提供“内部可变性”。

## 深入讲解

### Box\<T>

在堆上分配数据，适合递归类型或大型数据。`Box<T>` 拥有堆上的数据，离开作用域时自动释放。

### Rc\<T>

引用计数智能指针，允许多个所有者共享数据。`Rc<T>` 只能用于单线程场景。

### RefCell\<T>

运行时借用检查，提供内部可变性。与 `Rc<T>` 搭配可以实现共享可变状态。

### 线程安全版本

- `Arc<T>`：`Rc<T>` 的线程安全版本。
- `Mutex<T>` / `RwLock<T>`：线程安全的内部可变性。

## 常见误区 ⚠️

- **误区 1**：`Box<T>` 会让数据变慢。  
  ✅ 正解：`Box` 只是堆分配，访问速度与引用相同；不必要的堆分配才会带来开销。
- **误区 2**：`Rc<T>` 可以跨线程使用。  
  ✅ 正解：`Rc<T>` 不是线程安全的，跨线程请用 `Arc<T>`。
- **误区 3**：`RefCell<T>` 绕过了借用检查。  
  ✅ 正解：`RefCell` 把检查从编译期移到了运行时，违反规则会 panic。

## 一句话总结 🦀

> 智能指针在普通指针之上增加了所有权语义，让复杂的内存模式既安全又易于表达。

<RustPlayground />
