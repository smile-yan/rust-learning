use regex::Regex;

fn main() {
    let re = Regex::new(r"\d{3}-\d{4}").unwrap();
    let text = "客服: 010-1234, 电话: 021-5678";

    println!("包含号码: {}", re.is_match(text));

    if let Some(m) = re.find(text) {
        println!("第一个号码: {} (位置 {}-{})", m.as_str(), m.start(), m.end());
    }

    let email_re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
    if let Some(caps) = email_re.captures("请联系 alice@example.com") {
        println!("用户名: {}, 域名: {}, 后缀: {}", &caps[1], &caps[2], &caps[3]);
    }

    let hidden = re.replace_all(text, "***-****");
    println!("脱敏后: {}", hidden);
}
