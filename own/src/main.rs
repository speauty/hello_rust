fn main() {
    let s = String::from("hello");
    takes_ownership(s); // 已经移动, 调用后无效, 如果后续需要使用, 可以直接返回出来
    // println!("the value of s is: {}", s); // 报错: value borrowed here after move
    let x = 5;
    makes_copy(x); // 复制

    // 克隆: 深拷贝
    // Rust 永远也不会自动创建数据的 “深拷贝”
    // 任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小
    // 如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用，也就是赋值的过程即是拷贝的过程
    //  任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以Copy的
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // 对于基本类型（存储在栈上），Rust 会自动拷贝，但是 String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝

    // Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    // 栈和堆的核心目标就是为程序在运行时提供可供使用的内存空间
    // 栈按照顺序存储值并以相反顺序取出值，这也被称作后进先出 增加数据叫做进栈，移出数据则叫做出栈
    // 操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针, 该过程被称为在堆上分配内存
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
