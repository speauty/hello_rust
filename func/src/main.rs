fn main() {
    
    let x = plus_or_minus(5);
    println!("the value of x is: {}", x);

    // let x = plus_five(5);
    // println!("the value of x is: {}", x);
    
    // _another_function(5, 6.5);
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5
    } 
    x + 5
}

fn _plus_five(x: i32) -> i32 {
    x + 5 // 表达式
}

fn _another_function(x: i32, y: f32) {
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}
