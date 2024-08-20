
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 结构体: struct 结构体名 {字段...}
// 初始化实例时，每个字段都需要进行初始化
// 初始化时的字段顺序不需要和结构体定义时的顺序一致
// 通过 . 操作符即可访问结构体实例内部的字段值，也可以修改它们

// 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段

// 使用 #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出

// dbg! 宏，它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回
fn main() {
    // 元组结构体: 匿名字段的结构体
    // struct Color(i32, i32, i32);

    // 单元结构体: 无字段和属性
    // 如果定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体

    // let mut use1 = User {
    //     username: String::from("someuser111"),
    //     email: String::from("someuser@example.com"),
    //     active: true,
    //     sign_in_count: 2,
    // };
    // user1.email = "just@example.com";
}
