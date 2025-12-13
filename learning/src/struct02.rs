#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn squar(size: u32) -> Rectangle {
        Self { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::squar(40);
    if rect1.width() {
        println!("The area of the rectangle is {} square pixels.", 
        rect1.area());    
    }
    
}