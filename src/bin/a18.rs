// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: String::from(name),
                age,
            })
        } else {
            Err("Must be at least 21 years of age.")
        }
    }
}



// #[derive(Debug)]
// enum MenuChoice {
//     MainMenu,
//     Start,
//     Quit,
// }

// fn get_choice(input: &str) -> Result<MenuChoice, String> {
//     match input {
//         "mainmenu" => Ok(MenuChoice::MainMenu),
//         "start" => Ok(MenuChoice::Start),
//         "quit" => Ok(MenuChoice::Quit),
//         _ => Err(String::from("Menu choice not found.")),
//     }
// }

// fn print_choice(choice: &MenuChoice) {
//     println!("Choice = {:?}", choice);
// }

// fn pick_choice(input: &str) -> Result<(), String> {
//     let choice: MenuChoice = get_choice(input)?;
//     print_choice(&choice);
//     Ok(())
// }


fn main() {
    let adult = Adult::new("Anthony", 30);

    let child = Adult::new("Sariah", 8);

    match adult {
        Ok(adult) => println!("{:?} is {:?} years old.", adult.name, adult.age),
        Err(e) => println!("{}", e),
    }
    match child {
        Ok(child) => println!("{:?} is {:?} years old.", child.name, child.age),
        Err(e) => println!("{}", e),
    }
    // let choice = pick_choice("start");
    // println!("Choice Result value = {:?}", choice);

    // let choice: Result<MenuChoice, _> = get_choice("leave");
    // match choice {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(e) => println!("Error = {:?}", e),
    // }

    // println!("Choice = {:?}", choice);


}
