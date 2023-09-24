use std::{fmt::{Debug, Display}};

fn main() {

    let human:Human = Human{
        name: String::from("Carlos"),
    };

    let cat:Cat = Cat::default();

    println!("{}", human.say_hello());
    println!("{}", cat.say_hello());

    let option: Option<i32> = Some(20);

    println!("is adult? {}", option.is_adult());

    println!("{:#?}", human); //implemented debug display trait or derive debug
    println!("{:#?}", cat);
    println!("{}", cat);
}

#[derive(Debug)]
struct Human{
    name:String,
}

//Custom debug format
impl Debug for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This is an Cat called {}", self.name)
    }
}

impl Display for Cat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Beautiful display of {}", self.name)
    }
}

impl Default for Cat {
    fn default() -> Self {
        Self { name: String::from("Chucky") }
    }
}
struct Cat  {
    name:String
}

trait Speak {
    fn say_hello(&self) -> String;
}

impl Speak for Human {
    fn say_hello(&self) -> String{
        return String::from("hello");
    }
}

impl Speak for Cat {
    fn say_hello(&self) -> String{
        return String::from("mew");
    }
}

trait DrivingLicense{
    fn is_adult(&self) -> bool;
}

impl DrivingLicense for Option<i32> {
    fn is_adult(&self) -> bool {
       return self.unwrap_or_default() > 18;
    }
}