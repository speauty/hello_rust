
// 引用与借用
// 借用: 获取变量的引用
fn main() {

    // 借用规则
    // 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    // 引用必须总是有效的

    // 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用

    // let mut s = String::from("hello");
    // // 在新的编译器中，该代码将顺利通过，因为 引用作用域的结束位置从花括号变成最后一次使用的位置
    // let r1 = &s;
    // let r2 = &s;
    // println!("{} and {}", r1, r2);
    // let r3 = &mut s;
    // println!("{}", r3);

    // let mut s1 = String::from("hello");
    // let len = calc_length(&s1); // 直接传入引入即可, 不涉及所有权转移
    // println!("the length of '{}' is {}", s1, len);
    // // 可变引用同时只能存在一个
    // // 同一作用域，特定数据只能有一个可变引用
    // // 可变引用与不可变引用不能同时存在
    // // 数据竞争可由以下行为造成
    // // 两个或更多的指针同时访问同一数据
    // // 至少有一个指针被用来写入数据
    // // 没有同步数据访问的机制
    // change(&mut s1);
    // println!("the value of s1 is: {}", s1);

    // // 常规引用就是一个指针类型, 指向了对象存储的内存地址
    // let x = 5;
    // // & 引用  * 解引
    // let y = &x;
    // // *y = 6; // 报错: `y` is a `&` reference, so the data it refers to cannot be written
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
}

fn _change(s: &mut String) { // 同一作用域，特定数据只能有一个可变引用
    s.push_str(", world")
}

fn _calc_length(s: &String) -> usize {
    s.len()
}
