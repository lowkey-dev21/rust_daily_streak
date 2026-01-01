use std::io;

fn main() {
    println!("Hello, world!");

    let tup : (u32,f32) = (5,0.9);

    let (x,y) = tup;

    println!("{x} and {y}");
    println!("{}", tup.0);

    let total_number : [i32;6] = [1,2,3,4,5,6];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    
    let index: usize = match index.trim().parse() {
        Ok(index) => index,
        Err(_) => {
            println!("You did not enter a valid index");
            println!("Defaulting to the first one");
            0
        }
    };


    println!("{}", total_number[index]);
}
