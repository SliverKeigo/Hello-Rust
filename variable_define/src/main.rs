fn main() {
    // 变量定义
    // let 变量名 = 变量值;  不指定變量類型
    // let 变量名: 变量类型 = 变量值;  指定變量類型
    // 變量就是給某一個內存地址起名字
    // 变量名必须以字母或_开头，后面可以是任意字母、数字或下划线
    // 变量名是大小写敏感的
    // 可變變量
    // mut 關鍵字 mutable 的縮寫
    // let mut 變量名 = 值
    // let mut 變量名:數據類型 = 值

    let a: i32 = 5;
    println!("a = {}", a);

    // let a2: i32 = 10;
    //
    // a2 = 20;  // 错误，变量a2是不可变的     |     ^^^^^^^ cannot assign twice to immutable variable

    let mut a2: i32 = 10;
    println!("a2 = {}", a2);
    a2 = 20;
    println!("a2 = {}", a2);

}