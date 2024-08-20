struct _Struct {
    e: i32
}

fn main() {
    // 序列, 适用于数字或字符类型: 字符和数字值是Rust中仅有的可以用于判断是否为空的类型
    // .. 不含尾数
    // ..= 含尾数
    for i in 1..5 {
        println!("{}", i);
    }

    // let a = 2i32;
    // let b = 3i32;
    // println!("a = {}, b = {}", a, b);
    // println!("(a & b) = {}", a & b);
    // println!("(a | b) = {}", a | b);
    // println!("(a ^ b) = {}", a ^ b);
    // println!("(!b) = {}", !b);
    // println!("(a << b) = {}", a << b);
    // println!("(a >> b) = {}", a >> b);
    // let mut a = a;
    // a <<= b;
    // println!("a << b = {}", a);

    // 所有跟NaN交互的操作，都会返回一个NaN
    // let x = (-42.0_f32).sqrt();
    // // assert_eq!(x, x);
    // if x.is_nan() { // 判断数值是否NAN
    //     println!("未定义的数学行为");
    // }

    // let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    // println!("abc (f32)");
    // println!("0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    // println!("0.3: {:x}", (abc.2).to_bits());

    // println!("xyz (f64)");
    // println!("0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("0.3: {:x}", (xyz.2).to_bits());

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // 浮点数: 默认为f64(双精度), 另外支持f32(单精度) IEEE-754
    // let _x = 2.0; // f64
    // let _y: f32 = 2.0; // f32
    // assert!(0.1 + 0.2  == 0.3); // assertion failed: 0.1 + 0.2 == 0.3

    // 整型默认为i32类型
    // 补码循环溢出规则
    // let a: u8 = 255; // 0 ~ 256
    // let b = a.wrapping_add(20); // 256 + 19 => 0 + 19
    // println!("{}, {}", b, u8::MAX); // 19, 255

    // 变量遮蔽: 允许申明相同的变量名, 在后面声明的变量会遮蔽前面的声明
    // 可以直接重复, 避免命名困难
    // let x = 5;
    // let x = x + 1; // 6的新变量
    // { // 局部作用域
    //     let x = x * 2; // 6 * x的新变量
    //     println!("the value if x in the inner scope is: {}", x);
    // } // 作用域解除, 恢复为最近一次声明的变量
    // println!("the value of x is: {}", x);

    // 常量使用const声明, 且必须标注类型, 不可使用mut修饰, 始终不可变
    // const MAX_POINTS: u32 = 100_000;

    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct {e, ..} = Struct {e: 5};
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 解构操作
    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // 申明的变量默认不可变, 需要加mut关键字为可变
    // 前缀_可忽略未使用的变量错误
    // let x = 5; // 添加mut关键字即可运行
    // println!("the value of x is: {}", x);
    // x = 6; // 如果声明的变量没加关键词直接报错: cannot assign twice to immutable variable
    // println!("the value of x is: {}", x);
}
