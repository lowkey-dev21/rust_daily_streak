fn main() {
    println!("Hello, world!");

    another_function(90);
    multiply_together(8,9);



}

fn five() -> i32 {
   5
}

fn another_function(x : i32){
    println!("This is another function {x}")
}


fn multiply_together(x : i32 ,y : i32){
    let z = five();
    println!("{}", x*y);
    println!("{}",z)
}