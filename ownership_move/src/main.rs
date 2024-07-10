fn main() {
    /*
 Rust 有一项名为“所有权”的特性。这套规则由 Rust 编译器在编译时检查。
 所有权规则包括：
 Rust 中的每个值都拥有一个被称为“所有者”的变量。
 每个值在同一时间只能有一个所有者。
 当所有者超出作用域时，该值将被丢弃。
  */
    let a = 5;

    println!("a: {}", a);

    let b = a; // a is copied to b

    println!("b: {}", b);


    let name = String::from("Keigo");
    let name2 = name; // name is moved to name2

    println!("name: {}", name2);
    // println!("name: {}", name); // error: use of moved value: `name`

    let v1 = vec![1, 2, 3];

    let v2 = v1; // v1 is moved to v2

    // println!("v1: {:?}", v1); // error: use of moved value: `v1`

    println!("v2: {:?}", v2);
    show(v2);
    // println!("v2: {:?}", v2); // error: use of moved value: `v2`

    let name = String::from("Keigo");
    let name2 = name; // name is moved to name2
    let v3 = show2(name2); // v2 is moved to v3

    println!("v3: {:?}", v3);



}
fn show2(v: String) -> String {

    println!("v: {:?}", v);
    return v

}

fn show(v: Vec<i32>) {
    println!("v: {:?}", v);
}