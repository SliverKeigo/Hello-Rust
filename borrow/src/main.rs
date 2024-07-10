fn main() {

    // 借用 &变量名
    let name = String::from("Keigo");
    let name2 = name;
    show(&name2);
    println!("{}", name2);

    let mut name3 = vec!["Keigo"];
    println!("{}", name3[0]);
    show2(&mut name3);
    println!("{}", name3[0]);

}

fn show(name: &String) {
    println!("name is {}.", name);
}

fn show2(mut name: &mut Vec<&str>) {
    name[0] = "Ori";

    println!("{}", name[0]);
}