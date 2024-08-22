
// 方法
fn main() {
    let rect: Reactangle = Reactangle {width: 30, height: 50};
    println!("width: {}", rect.width());
    println!("the area of the rectangle is {}", rect.area());
}

#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    // 允许方法名跟结构体的字段名相同
    // 判断长度是否有效
    fn width(&self) -> bool {
        self.width > 0
    }
    // 矩形面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle {
//     // 自定义创建函数
//     // 定义在 impl 中且没有 self 的函数被称之为关联函数
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle {x: x, y: y, radius: radius}
//     }
//     fn area(&self) -> f64 { // 计算圆面积的方法
//         std::f64::consts::PI * (self.radius * self.radius)
//     }
// }