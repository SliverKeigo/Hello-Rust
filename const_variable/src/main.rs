fn main() {
    /*
    const 常量名称：数据类型 = 值
    常量名称通常是大写字母
    常量可以在任何地方定义 常量只是一个符号 编译时替换为具体的值
    常量不能被修改

    static 具有‘static 声明周期的 可以是可变的的变量
     */

    const PI: f64 = 3.14159;

    println!("PI = {}", PI);

    /*
        变量的隐藏
     */

    let name = "Rust";

    let name = "Keigo";

    println!("name = {}", name);

    // 隐藏变量并且改变数据类型
    let price:i32 = 199;
    let price = "299";

    println!("price = {}", price);

    // 常量不能被隐藏也不能被重复定义
    // `PI` is defined multiple times [E0428]`PI` can only be defined once in the `value` namespace of this block
    // const  PI: f64 = 3.14;


    static BOOK: &'static str = "<Keigo>";

    println!("PI = {}", BOOK);


}
