//traditional struct
// struct Colour{
//     red: u8,
//     green:u8,
//     blue: u8
// }

//tuple struct
// struct Colour(u8,u8,u8);

struct Person {
    firstname: String,
    lastname: String
}

impl Person{
    //Construct a person
    fn new(first: &str, last: &str) -> Person{
        Person {
            firstname: first.to_string(),
            lastname: last.to_string()
        }
    }
    fn full_name(&self) -> String{
        format!("{} {}", self.firstname, self.lastname)
    }

    //set last name
    fn set_last_name(&mut self, last: &str){
        self.lastname = last.to_string();
    }

    //name to tuple 
    fn to_tuple(self) -> (String, String){
        (self.firstname, self.lastname)
    }

}


pub fn run(){

//     //used to create custom data types
//     let mut c = Colour{
//         red: 255,
//         green: 0,
//         blue: 0
//     };

//     c.red = 200;

//   println!("Colour: {} {} {}", c.red, c.green, c.blue);

// let mut c = Colour(255,0,0);
// c.0 = 200;
// println!("Colour: {} {} {}", c.0, c.1, c.2);

let mut p = Person::new("Mary", "Doe");
println!("Person {}", p.full_name());
p.set_last_name("Williams");
println!("Person {}", p.full_name());
println!("Person Tuple {:?}", p.to_tuple());


}