use num::complex::Complex;
// 引入库, 首次编译有点慢, 需要下载和编译, 二次运行就正常了

fn main() {
    // 复数 实数+虚数
    let a = Complex{re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
