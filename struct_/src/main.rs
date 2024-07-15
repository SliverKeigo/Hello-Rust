#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: i32
}

fn main() {
    let user = User {
        email: String::from("keigo@keigo.cc"),
        username: String::from("Keigo"),
        active: true,
        sign_in_count: 1,
    };

    show_user(&user);

    let user2 = get_instance(String::from("Keigo2"), String::from("keigo2@keigo.cc"));

    show_user(&user2);

     let user3 = User::new(String::from("Keigo3"));

    show_user(&user3);

    println!("User3 active: {}", user3.active());

    let pair = ("keigo@keigo.cc", "Keigo");

    let (email, username) = pair;
    println!("email: {}, username: {}", email, username)

}


fn show_user(user: &User) {

    println!("User: {:#?}", user);
    println!("Name: {}", user.username);
}

fn get_instance(name:String, email:String) -> User{

    User {
        email,
        username: name,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn new(username: String) -> User {
        User {
            email: String::from(""),
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn active(&self) -> bool {
         self.active
    }
}