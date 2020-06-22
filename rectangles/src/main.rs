// fn main() {
//     let width = 500;
//     let height = 300;

//     println!(
//         "The area of the rectangle is {} px.",
//         calc_area(width, height)
//     );
// }

// fn calc_area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn main() {
//     let rect = (50, 30);

//     println!(
//         "The area of the rectangle is {} px.",
//         calc_area(rect)
//     );
// }

// fn calc_area(dimensions: (u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// fn main() {
//     let rect = Rectangle {
//         width: 50,
//         height: 30
//     };

//     println!(
//         "The area of the rectangle is {} px.",
//         calc_area(&rect)
//     );
// }

// fn calc_area(rectangle: &Rectangle) -> u32 {
//     println!("{:#?}", rectangle);
//     return rectangle.width * rectangle.height;
// }

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size
        };
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.width > rect.width && self.height > rect.height;
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 30
    };

    let rect2 = Rectangle {
        width: 50,
        height: 20
    };

    // println!(
    //     "The area of the rectangle is {} px.",
    //     rect.area()
    // );

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(50);

    println!("Square height {}", square.height);
}