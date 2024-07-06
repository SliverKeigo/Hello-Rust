use std::char::REPLACEMENT_CHARACTER;

fn main() {
    // &str 是模块std:str , 字符串切片
    let name = "sliver";

    println!("Hello, {}!", name);

    // String 是模块std::string, 字符串对象
    // String::new() 创建一个空的字符串对象
    // String::from() 从具体的字符串字面量创建一个字符串对象

    let mut name = String::new();
    // 添加字符串
    name.push_str("SliverKeigo");
    name.push('!');
    println!("{}", name);

    // String 是可变的
    let name = String::from("Keigo");
    println!("Hello, {}!", name);
    // 替换字符串
    let name = name.replace("Keigo", "SliverKeigo");

    // 获取字符串长度
    let len = name.len();

    println!("len: {}", len);

    // 字符串切片
    let slice = &name[0..6];

    println!("slice: {}", slice);

    // show my name
    show_my_name(&name);

    // 字符串修剪
    let name = "\t SliverKeigo \t";
    println!("name: {}, len: {}", name, name.len());


    let name = name.trim();
    println!("name: {}, len: {}", name, name.len());

    for char in name.chars() {
        println!("char: {}", char);
    }

    let sliver = "Sliver";
    let keigo = "Keigo";
    let name = sliver.to_string() + keigo;

    println!("Hello, {}!", name);

    show_my_name(name.as_str());
}

fn show_my_name(name: &str) {
    println!("Hello, my name is {}!", name)
}
