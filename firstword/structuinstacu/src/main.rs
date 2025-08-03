fn main() {
    let mut user1= User {
        active : true ,
        name : String::from("Aksshay"),
        email:String::from("aksshay123@gmail.com"),
        sign_in_count:1,
    };
    user1.name=String::from("aswin");
    let mut user2=User{
        active:user1.active,
        name:user1.name,
        email:String::from("example@gmail.com"),
        sign_in_count:user1.sign_in_count,
    };
}

struct User {
    active : bool,
    name : String , 
    email: String , 
    sign_in_count: u64 ,
}
