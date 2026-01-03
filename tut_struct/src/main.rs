struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("lowkey"),
        email: String::from("oyetolamuiz@gmail.com"),
        sign_in_count: 5,
    };

    user1.email = String::from("newemail@gmail.com");
    build_user(String::from("myemail@gmail.com"), String::from("lowkey"));

    let mut user2 = User{
        email: String::from("new@gmail.com"),
        ..user1
    };

   

}

fn build_user (email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 5 }
}