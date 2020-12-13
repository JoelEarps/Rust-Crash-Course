pub fn run(){
    //Default this is i32
    let x = 1;
    //Default this is going to be f64
    let y = 2.5;
    //Add explicit type
    let z:i64 = 4544454545;

    //find max size of int
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    // Get noolean by expression
    let is_greater: bool = 10> 5;
    

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, is_greater,a1, face));

}