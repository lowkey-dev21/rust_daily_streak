fn main() {

    let number = 5;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("condition was false");
    };

    let mut counter : i32 = 0;

    let result : i32 = loop {
        counter += 1;

        if counter  == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");
}
