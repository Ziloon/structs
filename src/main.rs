struct User {
    name: String,
    age: u32,
    email: String,
    active:bool,
    sign_in_count: u64,
}

fn build_user(email:String, name:String) -> User {
    User {
        email,
        name,
        age: 10,
        active: true,
        sign_in_count: 100,
    }
}
fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("zioon.z@icloud.com"),
        name: String::from("Ziloong"),
        age: 18,
        active: true,
        sign_in_count: 556,
    };

    println!("{}", user1.age);
    let user2 = build_user(String::from("try@trust.edu.cn"), String::from("Chen Wenju"));
    println!("{}", user2.name);
}


