fn main() {

    // 字符串是由字符组成的连续集合
    // Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间
    // 但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)

    // 从&str转成String: String::from to_string()
    // 从String转成&str: 取引用

    //  Rust 不允许去索引字符串

    // 切片: 允许你引用集合中部分连续的元素序列，而不是引用整个集合
    // 对于字符串而言，切片就是对 String 类型中某一部分的引用
    // [开始索引..终止索引]
    // 左开右闭: 开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置
    // 在切片数据结构内部会保存开始的位置和切片的长度，其中长度是通过 终止索引 - 开始索引 的方式计算得来的
    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界
    // 字符串切片的类型标识是 &str
    // 字符串字面量是切片
    // let s = String::from("hello world");
    // let hello = &s[0..5]; // 同..5一样, 从0开始
    // let world = &s[6..11];
    // println!("the value of hello is: {}", hello);
    // println!("the value of world is: {}", world);

    // let name = "Pascal";
    // greet(name.to_string()); // 手动转为String类型即可
}

fn _greet(name: String) {
    println!("hello, {}!", name);
}
