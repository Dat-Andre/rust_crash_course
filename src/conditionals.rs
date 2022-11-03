// Conditionals - Used to check the condition of something an act...

pub fn run(){

    let age = 22;
    let check_id = true;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What do you like to drink?");
    }else if age< 21 && check_id {
        println!("Bartender: sorry you have to leave.");
    }else {
        println!("Bartender: I'll need to see your ID.");
    }

    // shorthand if
     let is_of_age = if age>21 {true} else {false};
     println!("Is of age: {}", is_of_age);

}