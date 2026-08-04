// 策略模式：Box<dyn Trait> 实现运行时多态
trait Payment {
    fn pay(&self, amount: u64);
}

struct Alipay;
struct WechatPay;

impl Payment for Alipay {
    fn pay(&self, amount: u64) { println!("支付宝支付 {}", amount); }
}
impl Payment for WechatPay {
    fn pay(&self, amount: u64) { println!("微信支付 {}", amount); }
}

struct ShoppingCart {
    payment: Box<dyn Payment>,
}

impl ShoppingCart {
    fn checkout(&self, amount: u64) {
        self.payment.pay(amount);
    }
}

fn main() {
    let cart = ShoppingCart { payment: Box::new(Alipay) };
    cart.checkout(100);
}
