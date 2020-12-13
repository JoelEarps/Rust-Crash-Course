pub fn run(){
    let name = "Joel";
    let mut age = 26;
    println!("My name is {} and I'm {}", name, age);
    age = 27;
    println!("My name is {} and I'm {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_age, my_name) = (26, "Joel");
    println!("{} is {}", my_name, my_age);
}