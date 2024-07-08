fn main() {
    /*
    for 臨時變量 in 左區間..右區間{
        // 執行業務邏輯
    }
    左閉右開
     */
    for num in 1..5 {
        print!("{}", num);
    }
    println!();
    for num in 1..=5 {
        print!("{}", num);
    }
    println!();
    let study_list = vec![
        "Rust",
        "React",
        "Go",
    ];

    let mut study_list2 = vec![
        "Java",
        "Vue",
    ];

    // iter() 每次迭代是借用集合中的一個元素 元素本身不會被改變 循環之後還可以使用
    for name in study_list.iter() {
        match name {
            &"Rust" => println!("{} is a programming language", name),
            &"React" => println!("{} is a framework", name),
            &"Go" => println!("{} is a programming language", name),
            _ => println!("{} is a language", name),
        }
    }

    // into_iter() 每次迭代是擁有集合中的一個元素 元素本身會被改變 循環之後不能使用
    for study in study_list.into_iter() {
        match study {
            "Rust" => println!("{} is a programming language", study),
            "React" => println!("{} is a framework", study),
            "Go" => println!("{} is a programming language", study),
            _ => println!("{} is a language", study),
        }
    }

    for study in study_list2.iter_mut() {
        *study = match study {
            &mut "Java" => { "Keigo" }
            &mut "Vue" => { "UG" }
            _ => { "Ori" }
        }
    }


    for study in study_list2.iter() {
        println!("{}", study);
    }

    let mut num: i32 = 5;

    while num < 10 {
        println!("{} is less than 10", num);
        num += 1;
    }

    loop {
        if num > 20 {
            break;
        }
        println!("{} is less than 20", num);
        num += 1;
    }
}




