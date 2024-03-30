struct User {
    name: String,
    age: u32,
    email: String,
    active:bool,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("zioon.z@icloud.com"),
        name: String::from("Ziloong"),
        age: 18,
        active: true,
        sign_in_count: 556,
    };

    println!("{}", user1.age);
}


