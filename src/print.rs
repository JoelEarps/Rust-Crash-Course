pub fn run(){
    //Print to console
    println!("Hello from the print rs file");
    //Basic formatting
    println!("This is a number { }", 1);
    //Basic formatting
    println!("{ } is from { }", "Joel", "Notts");
    //Positional Arguements
    println!("{0} is from {1} and {0} likes {2}", "Joel", "Notts", "Coding");
    //Named Arguments
    println!("{name} likes to play {activity}", name = "Joel", activity = "football");
    //traits or placeholder traits
    println!("Binary {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    //Placeholder for debug trait
    println!("{:?}", (12, true,"hello")); // also called a tuple
    //Basic math
    println!("10 + 10 ={}", 10+10)
}