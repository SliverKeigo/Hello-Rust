fn main() {
    /*
       fn  function縮寫
       fn 函數名稱([參數名稱: 參數型態]) -> 回傳型態 {
           // 執行邏輯代碼
       }

       無明確回傳值時，可以省略 -> 回傳型態
     */
    let sum = add(1, 2);
    println!("Sum: {}", sum);

    let name = get_name();
    println!("Name: {}", name);

    let name = change_name(String::from("K"));
    println!("Name: {}", name);

    let mut price = 100;
    println!("Price: {}", price);
    double_price(price);
    println!("Price: {}", price);
    double_price2(&mut price);
    println!("Price: {}", price);

    let name = String::from("Keigo");
    show_my_name(name);
    println!("My name is {}", name);

}

fn show_my_name(name:String) {
    println!("Name is {}", name);
}

// 值傳遞 函數內部和外部的變數是獨立的，函數內部對變數的修改不會影響外部變數
fn double_price(mut price: i32) {
    price = price * 2;
    println!("Double1 Price: {}", price)
}
// 引用傳遞 函數內部對變數的修改會影響外部變數
fn double_price2(price: &mut i32) {
    *price = *price * 2;
    println!("Double2 Price: {}", price)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn get_name() -> String {
    String::from("Keigo")
}

fn change_name(name: String) -> String {
    if name == "Keigo" {
        return String::from("K");
    }
    let name = String::from("Ori");
    name
}

