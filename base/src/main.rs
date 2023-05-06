fn main() {
    println!("Hello Rust !");

    // 常量定义
    const MAX_AGE: i32 = 23;
    println!("常量==={}", MAX_AGE); //23

    // 变量定义（变量默认不可变）
    let num = 100;
    println!("变量num==={}", num); //变量num===100
                                   // mut关键字 定义可变的变量
    let mut num_two = 50;
    num_two = 80;
    println!("可变的变量num2==={}", num_two); //可变的变量num2===50

    if num_two > 50 {
        println!("{}", num_two);
    } else {
        println!("111");
    }

    // if else
    // if MAX_AGE > a{
    //     println!("{}",MAX_AGE); //23
    // }else{
    //     println!("{}",)
    // }
}
