use std::collections::HashMap;

fn main() {
    /*
    向量 vector
    特点
    相同类型的元素集合
    长度可变
    使用索引查找
    添加元素到队尾
    向量内存在堆上 长度可动态变化
     */

    let mut v = Vec::new();

    v.push("Keigo");
    v.push("UG");

    println!("{:?}",v);

    v.remove(0);
    println!("{:?}",v);

    if v.contains(&"Keigo") {
        println!("找到了!")
    }else {
        println!("没找到!")
    }

    /*
    HashMap KV K不能重复 V可以重复
     */

    let mut person = HashMap::new();
    person.insert("Keigo",21);
    person.insert("UG",20);

    println!("{:?}", person);


}
