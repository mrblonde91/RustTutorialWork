//10.1 
// Generics are performant as handled at 
// compile time with monomorphisation
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = findlargest(&number_list);

    // Generic struct
    let integer = Point{x:"2.1", y:"hello world"};
    let decimal = Point{x:2, y:40.0};
    let newPoint = decimal.mixup(integer);
    println!("Value x is {},y:{}", newPoint.x, newPoint.y);
    println!("The largest number is {}", largest);
}

// fn findlargest(list : &[i32]) -> i32{
//     let mut largest = list[0];

//         for &number in list.iter() {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }

//Add copy to return 
fn findlargest<T: PartialOrd + Copy>(list : &[T]) -> T{
    let mut largest = list[0];

        for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

   largest
}

// struct Point<T>{
//     x : T,
//     y : T,
// }

// Gotta mark type after impl if generic implementation
// impl<T> Point<T>{
//     fn x(&self) -> &T{
//         &self.x
//     }
// }

// To allow different types
struct Point<T,U>{
    x : T,
    y : U,
}

//Bring two different point match ups together
impl<T,U> Point<T,U> {
    fn mixup<V, W>(self, other:Point<V, W>) -> Point<T,W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

