fn basic() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // create User instance
    // 인스턴스는 반드시 mutable 해야 한다.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 구조체 갱신 문법
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusernames567"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
