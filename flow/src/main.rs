
// 流程控制
// if for while loop
// continue break
fn main() {
    // 普通版本-if-else
    // let cond = true;
    // let num = if cond { 5 } else { 6 };
    // println!("the value of number is: {}", num);

    // if-else if-else-多分支结构
    // 有多个分支能匹配，也只有第一个匹配的分支会被执行
    // let n = 6;
    // if n % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if n % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if n % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3 or 2");
    // }

    // 循环-for
    // for idx in 1..=5 {
    //     print!("{}\t", idx);
    // }
    
    // 输出索引
    // let a = [4, 3, 2, 1];
    // for (idx, val) in a.iter().enumerate() {
    //     println!("第{}个元素: {}", idx + 1, val);
    // }

    // continue跳过当次循环
    // for i in 1..4 {
    //     if i == 2 {
    //         continue;
    //     }
    //     println!("{}", i);
    // }
    
    // break 退出循环
    // for i in 1..4 {
    //     if i == 2 {
    //         break;
    //     }
    //     println!("{}", i);
    // }

    // let mut n = 0;
    // while n <= 5 {
    //     println!("{}", n);
    //     n += 1;
    // }
    // println!("循环结束");

    // loop 无条件循环
    // break可以单独使用，也可以带一个返回值
    // loop一个表达式，因此可以返回一个值
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };

    println!("the result if {}", result);
}
