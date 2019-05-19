fn main() {
    let rectangle = Rectangle{width:30, height:70};
    println!("The area of the rectangle is {} square pixels....", areatuplerefactoringstruct(&rectangle));
    let square = Rectangle::square(20);
    println!("Debug print of {:#?}", rectangle);

    println!("Square is {:?}", square);
    //Struct specific function
    println!("Result of built in functionality is {}", rectangle.area());
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn areatuplerefactoringtuple(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

 fn areatuplerefactoringstruct(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
 }

// POC of debug
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

//Rectangle specific functions, --implementation block--
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    
    fn square(size:u32) -> Rectangle{
        Rectangle{height:size, width:size}
    }
}