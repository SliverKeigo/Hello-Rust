fn main() {
    /*
    定義規範

    1. 指向資料的起始位置

    2. 資料的長度

    3. 資料型態

    */

    let mut v = Vec::new();

    v.push(1);

    v.push(2);

    v.push(3);

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    let v1 = &v[1..3];
     // [..] 獲取全部元素
    // [起始位置..結束位置] 獲取指定範圍元素
    // [起始位置..] 獲取從指定位置到最後的元素
    // [..結束位置] 獲取從最後到指定位置的元素
    println!("{:?}", v1);

    show_slice(v1);

    modify_slice(&mut v[1..3]);

    show_slice(&v[1..3]);

}

fn modify_slice(s: &mut [i32]){
    for i in 0..s.len(){
        s[i] = 2 * s[i];
    }
}

fn show_slice(s: &[i32]){
    println!("{:?}", s);
}