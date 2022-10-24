
struct User {
    username: String,
    password: String,
    admin: bool,
    profile: UserProfile
}

struct UserProfile {
    age: u32,
}

struct Newsfeed {
    posts: Vec<u32>
}

#[derive(PartialEq)]
enum Roles {
    OWNER,
    EDITOR,
    VIEWER,
}

fn login(user: &User) -> Newsfeed {
    println!("Logging in user:{}", user.username);
    println!("Logged in username: {}, password: {}, age: {} ", user.username, user.password, user.profile.age);
    return Newsfeed{ posts: vec![1, 2, 3]};
}

fn main(){
    let user = User{
        username: String::from("njenius"),
        password: "1234".parse().unwrap(),
        admin: true,
        profile: UserProfile{
            age: 29
        }
    };
    println!("Newsfeed of user:{} is {:?}", user.username, login(&user).posts);

    let owner_role = Roles::OWNER;
    let editor_role = Roles::EDITOR;

    assert!(owner_role == editor_role);

}


