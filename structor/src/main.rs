struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    
    fn area(&self) -> u32 {
            self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
    }
}

fn main() {
   let rect1 = Rectangle { width: 30, height: 50 };
   let rect2 = Rectangle { width: 10, height: 40 };
   let rect3 = Rectangle { width: 60, height: 45 };
   let rect4 = Rectangle::square(20);

   // rect1にrect2ははまり込む？
   println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
   println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
   println!("Can rect1 hold rect1? {}", rect1.can_hold(&rect4));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//struct User {
//    username: String,
//    email: String,
//    sign_in_count: u64,
//    active: bool,
//}

//fn main() {
//   let user1 = User {
//        email: String::from("someone@example.com"),
//        username: String::from("someusername123"),
//        sign_in_count: 1,
//	active: true,
//	};
//    println!("{} {}", user1.email, user1.username);
//}
