use std::string;
#[derive(Debug)]
// use std::string;

struct Person {
    name: String,
    age: u32,
}
impl Person{
    // fn pp(name:String){
    //     println!("{}",name);
    // }
}
fn main() {
    // 必须 全大写 下划线分开
    const MAX_AGE: u32 = 10000;
    println!("常量==={}", MAX_AGE);

    println!("Hello, Rust!");
    // let a = go(1,2);
    println!("a+b=={}", go(1, 2));

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{},{},{}", tup.0, tup.1, tup.2); //500,6.4,1

    // 数组 [类型, 长度]
    let arr = [3, 5, 8, 4]; //let b = ["January", "February", "March"];
    println!("{}", arr[2]); // 8

    // 结构体实例化
    let person = Person {
        name: String::from("GJH"),
        age: 18,
    };
    println!("{:?}", person.name); //  "GJH"
    // println!("{:?}",person.pp("GJH"));
    println!("{:?}", person); // Person { name: "GJH", age: 18 }
    


}

// -> 指定返回值的类型
fn go(a: i32, b: i32) -> i32 {
    println!("go....");
    // println!("a+b=={}",a+b);
    // a+b+1
    return a + b;
}

