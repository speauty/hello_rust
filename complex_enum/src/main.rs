

// 枚举: 关键字 enum
// 包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn main() {
    let c1 = PokerCard {suit: PokerSuit::Clubs, value: 1};
    let c2 = PokerCard {suit: PokerSuit::Diamonds, value: 12};

    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;
    //
    // print_suit(heart);
    // print_suit(diamond);
}

// fn print_suit(card: PokerSuit) {
//     println!("{:?}", card);
// }