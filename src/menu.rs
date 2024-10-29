use std::num::ParseIntError;

use crate::helpers;

pub fn show_menu(show_welcome: bool) -> [String; 5] {
    if show_welcome {
        println!("Welcome to the CLI To Do App.");
    }
    println!("Here's the menu, reply with the number of the operation you want to perform:");
    let menu_array: [String; 5] = [
        String::from("View All Tasks"),
        String::from("Add New Task"),
        String::from("Update A Task"),
        String::from("Delete A Task"),
        String::from("Exit Program"),
    ];
    let mut counter: i32 = 1;
    for item in &menu_array {
        println!("{counter}. {item}");
        counter += 1;
    }
    menu_array
}

pub fn get_menu_selection() -> Result<usize, ParseIntError> {
    let trimmed_input = helpers::get_input_from_cli("Please enter a menu number: ");
    println!("You entered: {}", trimmed_input);
    let menu_input: Result<usize, ParseIntError> = trimmed_input.parse::<usize>();
    menu_input
}
