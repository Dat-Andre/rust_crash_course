// Tuples group together values that can be of different types
// Max 12 elements

pub fn run(){

    let person: (&str, &str, i8) = ("André", "Portugal", 38);

    println!("{} is from {} and is {}", person.0, person.1, person.2);



}