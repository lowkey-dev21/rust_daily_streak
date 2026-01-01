fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let name = "Muiz";
    let my_name = name;
    println!("{my_name}");

    let other_name = String::from("Akande");
    let other_name_two = other_name;
    println!("{other_name_two}");

    let mut s1 = String::from("Malik");

    let len = calculate_length(&s1);
    println!(" '{s1}' , {len}");

    let changed_string = change(&mut s1);

    println!("{changed_string}");

    let sent = String::from("This is the name");
    let first_word_sent = first_word(&sent);
    println!("{first_word_sent}");

    


}


fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(some_string : &mut String) -> &str {
   some_string.push_str(",World");
   some_string

}


fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        } 
    }
    &s[..]
}
