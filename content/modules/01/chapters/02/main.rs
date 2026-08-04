// trait 定义一组共享行为：summarize_author 是没有默认体的必需方法
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 带默认实现的方法：实现者可直接复用，也可以覆盖
    fn summarize(&self) -> String {
        format!(
            "(阅读更多来自 {} 的内容...)",
            self.summarize_author()
        )
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

// 为 NewsArticle 实现 trait，并覆盖了默认的 summarize
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!(
            "{} - {} (作者: {})",
            self.headline,
            self.location,
            self.author
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Tweet 只实现必需方法，summarize 沿用 trait 的默认实现
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl Trait 作参数：接受任何实现了 Summary 的类型，
// 是「泛型 + trait 约束」的语法糖
fn notify(item: &impl Summary) {
    println!("突发新闻! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.80 发布"),
        location: String::from("全球"),
        author: String::from("Rust 团队"),
    };
    // 这里调用的是覆盖后的 summarize
    notify(&article);

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust 越来越好用了！"),
    };
    // 这里调用的是 trait 默认实现的 summarize
    notify(&tweet);
}
