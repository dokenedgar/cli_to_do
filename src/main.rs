mod add_task;
mod delete_task;
mod helpers;
mod menu;
mod to_do_struct;
mod update_task;
mod view_all_tasks;
use std::num::ParseIntError;

pub const FILE_PATH: &str = "data/todo.json";

fn main() {
    helpers::create_file_if_not_exists(FILE_PATH);
    menu::show_menu(true);
    let mut menu_input: Result<usize, ParseIntError> = menu::get_menu_selection();
    while menu_input.is_err() {
        println!("\nError parsing menu selection.\nPlease try again:\n");
        menu::show_menu(false);
        menu_input = menu::get_menu_selection();
    }
    let mut menu_selection = menu_input.unwrap();
    println!("Menu number chosen: {menu_selection}");
    while menu_selection > 6 {
        println!("Selected value {menu_selection}, greater than options");
        menu::show_menu(false);
        menu_input = menu::get_menu_selection();
        while menu_input.is_err() {
            println!("\nError parsing menu selection.\nPlease try again:\n");
            menu::show_menu(false);
            menu_input = menu::get_menu_selection();
        }
        menu_selection = menu_input.unwrap();
    }
    match menu_selection {
        1 => view_all_tasks::view_all(),
        2 => add_task::create_todo(),
        3 => update_task::update_todo(),
        4 => delete_task::delete_task(),
        5 => println!("Have a nice day, good bye!"),
        _ => println!("\nDon't know what action to perform\n"),
    }
}
