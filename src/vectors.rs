pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    //reassign values
    numbers[2] = 20;
    //add to vector
    numbers.push(5);
    numbers.push(6);

    //pop off last value
    numbers.pop();

    println!("{:?}", numbers);
    //get single val
    println!("Single Value: {}", numbers[0]);
    //vector length
    println!("Vector Length: {}", numbers.len());
    //vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //loop through values
    for x in numbers.iter(){
        println!("Number :{}", x);
    }

     //loop through values and mutate
     //similar to map in javascript
     for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}