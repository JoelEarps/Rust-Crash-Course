pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //reassign values
    numbers[2] = 20;

    println!("{:?}", numbers);
    //get single val
    println!("Single Value: {}", numbers[0]);
    //array length
    println!("Array Length: {}", numbers.len());
    //arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

}