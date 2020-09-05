#[derive(Debug)]
struct Color2 (u8, u8, u8);

#[derive(Debug)]
struct Color {
  rgb: (u8, u8, u8),
}

#[derive(Debug)]
struct Person {
  name: &'static str,
  dob: u8,
  fav_color: Color,
  color: Color2
}

impl Person {
  // describw methods for Person struct
  fn say_hello(&self) -> String {
    return format!("Hi! My name is {}", &self.name);
  }

  fn beverage(&self) -> String {
    if self.dob < 21 {
      "Have some milk.".to_string()
    } else {
      "Have a cocktail.".to_string()
    }
  }
}

fn main() {
  /*
    let age = 120.0; // unsigned integer
    // let age: i8 = 230000; // signed integer
    let dec: f32 = 1.05;
    let name = "Bolaji";
    println!("Hello world");

    println!("My age is {}, and my name is {}.", age, name);

    let thing = age * dec;
    dbg!(thing);
  */

  let rgb: (u8, u8, u8) = (255, 127, 0);
  dbg!(rgb.1); // access element at index 1

  // let person: (&str, u8) = ("Boolaji", 16);
  let person = Person{
    name: "Boolaji",
    dob: 114,
    fav_color: Color{ rgb: (255, 127, 0) },
    color: Color2(255, 127, 0)
  };
  let greeting = person.say_hello();
  let beverage = person.beverage();

  dbg!(person);
  // println!("{:?}", person)
  println!("{}. {}", greeting, beverage);
}