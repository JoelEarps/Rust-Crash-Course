//use to check condition of somethign then act on it, like many other languages

pub fn run(){
    let age: u8 = 23;
    let check_id: bool = true;
    let know_person_of_age: bool = true;

    //ifelse
    if age >= 21 && check_id || know_person_of_age {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("Sorry you have to leave");
    }
    else {
        println!("I'll need to check your ID");
    }

    //no turnary operator - shorthand if
    let is_of_age = if age>= 21 {true} else {false};
    println!("Is of age {}", is_of_age);
}