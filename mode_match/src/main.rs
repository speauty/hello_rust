
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// 模式匹配 match
// match 的匹配必须要穷举出所有可能，_来代表未列出的所有可能性
// 每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
// X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可

// enum Action {
//     Say(String),
//     MoveTo(i32, i32),
//     ChangeColorRGB(u16, u16, u16),
// }

fn main() {
    // let dire: Direction = Direction::South;
    // match dire {
    //     Direction::East => println!("East"),
    //     Direction::North | Direction::South => {
    //         println!("South or North");
    //     },
    //     _ => println!("West"),
    // };

    // let actions = [
    //     Action::Say("hello rust".to_string()),
    //     Action::MoveTo(1, 2),
    //     Action::ChangeColorRGB(255, 255, 0),
    // ];

    // for action in actions {
    //     match action {
    //         Action::Say(s) => println!("{}", s),
    //         Action::MoveTo(x, y) => println!("point from (0, 0) move to ({}, {})", x, y),
    //         Action::ChangeColorRGB(r, g, _) => println!("change color into '(r:{}, g:{}, b:0)', b has been ignored", r, g),
    //     }
    // }

    // 只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match
}
