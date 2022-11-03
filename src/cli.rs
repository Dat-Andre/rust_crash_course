use std::env;

pub fn run() {

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "André";
    let status = "99%";

    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?",name);
    }else if command == "status"{
        println!("Status is {}?", status);
    }else {
        println!("Dat is not a valid command!");
    }
}