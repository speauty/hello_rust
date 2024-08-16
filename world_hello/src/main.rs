fn greet_world() { // 命名风格全蛇皮吗?
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "Hello, World";
    let regions: [&str; 3] = [southern_germany, chinese, english]; // [] 数组
    for region in regions { // 也可加.iter()获取迭代器?
        // 不带换行的: print, {} 自动解析需要打印输出的类型
        // ! 宏操作符
        println!("{}", region)
    }
}

fn strings_parser() {
    let data = "\
    名称,身高(cm)
    张三,165
    李四,176
    麻子,155
    无效,数据
    ";
    let records = data.lines(); // 以换行符切割字符串 返回切片, 就是数组?

    // enumerate: 可获取当前元素索引
    // iter: 只是单纯的循环, 无索引
    for (idx, record) in records.enumerate() { // enumerate 创建yield迭代
        if idx == 0 || record.trim().len() == 0 { // 过滤空值
            continue; // 跳过本次循环, 同其他语言一致
        }
        // Vec 向量, 堆数据, _ 自动推导类型
        // @todo 待解析
        let fields: Vec<_> = record.split(',').map(|field|field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} => {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(hgih) = fields[1].parse::<i16>() {
            println!("{}, {}cm", name, hgih)
        }
    }
}

fn main() {
    println!("Hello, world!");
    greet_world();
    strings_parser();
}
