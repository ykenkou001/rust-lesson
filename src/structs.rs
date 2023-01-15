// Debug traitを付与することで、構造体の中身を表示できるようになる
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 構造体のメソッド
impl Rectangle {
    fn create(width: u32, height:u32) -> Self {
        Self {width, height}
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
} 
pub fn run() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("user1 is {:#?}", user1);
    let user2 = build_user(String::from("user2@example.com"), 
    String::from("user2"));
    println!("user2 is {:#?}", user2);
        
    let rect1 = Rectangle {width: 30, height: 50};
    println!("rect1 is {:#?}", rect1);
    rect1.area();
}
// email, usernameを引数にとる関数
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
