
// 元组是由多种类型组合到一起形成的，元组的长度是固定的，元组中元素的顺序也是固定的
fn main() {
    let tup = (500, 5.4, 1);
    // 解构：用同样的形式把一个复杂对象中的值匹配出来
    let (x, y, z) = tup; // 模式匹配解构元组
    println!("the value of (x, y, z) is: ({}, {}, {})", x, y, z);
    // 元组的索引从0开始, 用.访问元组里面的元素
    println!("the value of (0, 1, 2) is: ({}, {}, {})", tup.0, tup.1, tup.2);

    let s1 = String::from("hello");
    let (s2, len) = calc_length(s1); // 之后 s1已经无效, 不可访问
    println!("the length of '{}' is {}", s2, len);
}

fn calc_length(s: String) -> (String, usize) {
    // (s, s.len()) // 报错: value borrowed here after move
    let len = s.len();
    (s, len)
}
