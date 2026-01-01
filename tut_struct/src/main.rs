#[derive(Debug)]
struct  Rectangle {
    width: u32,
    height : u32
}

fn main() {
    println!("Hello, world!");

  let rect1  = Rectangle { width: 89, height: 90 };
   let result =  area(&rect1);
   println!(" the result of the area is {result}");
   println!("this is the struct values {rect1:#?}");
}


fn area(rectangle : &Rectangle) -> u32{
 
 rectangle.width * rectangle.height
}