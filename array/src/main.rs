

fn main() {
    // 数组的声明和初始化

    let a = [1, 2, 3, 4, 5];

    let mut b = [3; 5]; // [3, 3, 3, 3, 3]

    println!("{:?}", a);
    // 查看数组的长度
    println!("{} ", a.len());
    // 遍历
    for i in &a {
        print!("{}", i);
    }
    println!();


    println!("{:?}", b);
    for i in b.iter() {
        print!("{} ", i);
    }

    // 修改元素
    b[0] = 1;
    println!("\n{:?}", b);
    show_array(b);
    println!("\n{:?}", b);

    modify_array(&mut b);
    println!("\n{:?}", b);

}

fn modify_array(a: &mut [i32; 5]) {
      for i in 0..a.len() {
           a[i] = 0
      }
}

fn show_array(mut a:  [i32; 5]) {
    for i in 0..a.len() {
        if i == 1 {
            a[i] = 10;
        }
        print!("{} ",  a[i]);
    }
}