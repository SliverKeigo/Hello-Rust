fn main() {

    // 有符號類型 signed 可以存儲正數或負數 無符號類型 unsigned只能存儲正數
    // 按存儲空間來說 劃分1字節 2字節 4字節 8字節 16字節  1字節 = 8位
    let price = 100;
    let price2:u32 = 200;
    let price3:i32 = -300;
    let price4:isize = 400;
    let price5:usize= 500;

    println!("price is {}",price);
    //输出 price is 100
    println!("price2 is {} and price3 is {}",price2,price3);
    //输出 price2 is -300 and price3 is 200
    println!("price4 is {} and price5 is {}",price4,price5);
    //输出 price4 is 400 and price5 is 500

    // 類型錯誤
    // let price6:u32 = -600;

    // 超出範圍
    let price7:i8 = 192;



}