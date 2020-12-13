pub fn run(){
    let mut hello = String::from("Hello ");

   

    //Get length

    println!("Length: {}", hello.len());
    // push a char
    hello.push('W');
    //push a string 
    hello.push_str("orld");
    //capacity in bytes
    println!("Capacity:{}", hello.capacity());
    //is empty
    println!("Is Empty:{}", hello.is_empty());
    // conatins certain sub string
    println!("Conatins World:{}", hello.contains("World"));
    //replace part fo the string
    println!("Replace: {}", hello.replace("World", "There"));
    //loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create strign with capcity

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('s');

    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("{}", s);


}