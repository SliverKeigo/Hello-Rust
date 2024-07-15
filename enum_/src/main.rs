fn main() {
    /*
    枚舉的定義規範
     */


    let direction = Direction::Down;

    match direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }


    let p = 2;
    let result = get_discount(p);
    println!("{:?}", result);

    // 帶數據類型的枚舉

    let direction = Direction::Up;

    print_direction(direction);

    // 帶數據類型的枚舉
    let user = User::Age(2);
    match user {
        User::Name(name) => println!("Name: {}", name),
        User::Age(age) => println!("Age: {}", age),
    }
}

enum User {
    Name(String),
    Age(i32),
}


fn print_direction(direction: Direction) {
    match direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_discount(price: i32) -> Option<bool> {
    if price > 100 {
        Some(true)
    } else {
        None
    }
}