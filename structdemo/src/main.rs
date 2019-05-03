fn main() {
    let user1 = build_user(String::from("mike@gmail.com"),String::from("mike"));
    
    let user2 = User{email:String::from("jabnetty@gmail.com"), username:String::from("jake"), ..user1};//dot dot syntax autopopulates fields
    print_user(&user1);
    print_user(&user2);
    build_tuplestruct();
}

fn print_user(s:&User){
        println!("Email:{} Username:{}, Active:{}, Sign in count: {}", 
        s.email, s.username, s.active, s.sign_in_count);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_tuplestruct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,244,24);
    let origin = Point(0, 35, 63);
    println!("Colour:{},{},{}",black.0, black.1, black.2);
    println!("Point:{},{},{}",origin.0, origin.1, origin.2);
}

fn build_user(email:String, username:String) -> User{
    User{
         email,
        username,
        active: true,
        sign_in_count: 1
    }
}