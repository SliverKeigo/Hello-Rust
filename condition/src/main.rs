fn main() {
    let total: f64 = 150.00;

    if total > 200.00 {
        println!("You have a discount of 20%!");
    }

    if total > 100.00 {
        println!("You have a discount of 10%!");
    }else {
        println!("You have a discount of 5%!");
    }


    if total > 100.00 || total < 200.00 {
        println!("You have a discount of 10%!");
    }else if total > 200.00 {
        println!("You have a discount of 5%!");
    }


    let code = "10000";

    let choose = match code{
        "10000" => "You have a discount of 20%!",
         _ => "You have a discount of 5%!",
    };

    println!("{}", choose);


}
