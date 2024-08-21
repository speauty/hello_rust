// use std::io;

// array 为数组，Vector 为动态数组
// 多个类型相同的元素依次组合在一起，就是一个数组
// 长度固定 元素必须有相同的类型 依次线性排列
// array-栈  vector-堆
fn main() {
    // let a = [1, 2, 3, 4, 5]; // 默认
    // // [类型;长度]---中间是分号
    // let b: [i32; 5] = [1, 2, 3, 4, 5]; // 带类型声明的
    // // [固定值; 次数]
    // let c = [3; 5]; // 固定值出现N的数组

    // 访问元素, 下标从0开始, 越界会报错
    // let a = [9, 8, 7, 5, 6];
    // let first = a[0];
    // let second = a[1];

    // let a = [1, 2, 3, 4, 5];
    // println!("请输入一个数组索引: ");
    
    // let mut idx = String::new();
    // io::stdin().read_line(&mut idx).expect("读取异常");
    // let idx: usize = idx.trim().parse().expect("非法数字");

    // let ele = a[idx];
    // // 越界报错-运行时: index out of bounds: the len is 5 but the index is 34
    // println!("the value of the elelemt at index {} is: {}", idx, ele);
    // 报错: the trait `Copy` is not implemented for `String`
    // 复杂类型不支持深度拷贝
    // let strArr = [String::from("rust"); 8];
    // let str_arr = [String::from("rust"), String::from("rust"), String::from("rust"), String::from("rust")];
    // let str_arr: [String; 8] = std::array::from_fn(|_i|String::from("rust"));
    // println!("{:#?}", str_arr);

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let slice: &[i32] = &a[1..3];
    // assert_eq!(slice, &[2, 3]);

    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let black1 = [0; 3];
    let black2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, black1, black2];

    for item in &arrays { // 外层循环
        print!("{:?}", item);

        for num in item.iter() { // 迭代内层数组
            print!("\t{} + 10 = {}", num, num + 10);
        }

        let mut sum = 0;
        for i in 0..item.len() {
            sum += item[i];
        }
        println!("\t({:?} = {})", item, sum);
    }
}
