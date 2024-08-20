fn _add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 1;
    x + y
}

fn main() {
    // 能返回值，它就是表达式, 表达式不能包含分号
    // 表达式如果不返回任何值, 会隐式地返回一个()
    let y = {
        let x = 3;
        x + 1 // 这个是表达式
    };
    println!("the value of y is : {}", y);

    assert_eq!(ret_unit_type(), ());
}

fn ret_unit_type() {
    let x = 1;
    let _y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}
