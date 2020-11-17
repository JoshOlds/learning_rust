#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The struct looks like this: {:?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle (by tuple) is {} square pixels.",
        area_by_tuple(rect1)
    );

    println!(
        "The area of the rectangle (by struct) is {} square pixels.",
        area_by_struct(&rect2)
    );

    println!(
        "The area of the rectangle (by method) is {} square pixels.",
        rect2.area()
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    fn area_by_struct(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
