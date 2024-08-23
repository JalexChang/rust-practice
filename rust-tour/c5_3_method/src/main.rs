#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // new an instance. It's a static method because it doesn't take &self as a parameter.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // use &self to borrow the instance to avoid taking ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // use &mut self to borrow the instance mutably
    fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

fn main() {
    let mut rect1 = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    rect1.update(60, 60);
    println!(
        "After updating, the area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
