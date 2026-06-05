// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// 日本語版
// メアリーはリンゴを買っています。リンゴの値段は以下のように計算されます。
// - 1個のリンゴは2ラストバックスです。
// - しかし、メアリーが40個以上のリンゴを買うと、注文全体のリンゴの値段が1ラストバックスに減ります！

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

// TODO: 与えられたリンゴの数量に基づいて、リンゴの注文の価格を計算する関数を書いてください。

fn main() {
    // You can optionally experiment here.
}

fn calculate_price_of_apples(quantity: u32) -> u32 {
    const PRICE_PER_APPLE: u32 = 2;
    const DISCOUNTED_PRICE_PER_APPLE: u32 = 1;

    if quantity <= 40 {
        quantity * PRICE_PER_APPLE
    } else {
        quantity * DISCOUNTED_PRICE_PER_APPLE
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
