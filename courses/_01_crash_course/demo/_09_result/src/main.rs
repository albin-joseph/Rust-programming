#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
   println!("Choice = {:?}", choice);
}

fn main() {
    println!("Result");

    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    match choice {
        Ok(choice) => print_choice(&choice),
        Err(err) => println!("{}", err),
    }

    let choice1: Result<MenuChoice, _> = get_choice("mainmenu123");
    match choice1 {
        Ok(choice) => print_choice(&choice),
        Err(err) => println!("{}", err),
    }
    
}
