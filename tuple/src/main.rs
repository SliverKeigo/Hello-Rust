fn main() {
    /*
    let tup 變量名稱:(數據類型1,數據類型2,數據類型3) = (值1,值2,值3);
    let tup = (數據值1, 數據值2, 數據值3);
     */

    let name = ("Keigo","Ori");
    println!("{} {}",name.0,name.1);

    show_name(name);

    // 解構
    let (x,y) = name;
    println!("{} {}",x,y);

}
fn show_name(t:(&str,&str)){
    println!("{} {}",t.0,t.1);
}