use crate::helpers;
use crate::to_do_struct::ToDoItem;
use crate::view_all_tasks;
use std::num::ParseIntError;

pub fn delete_task() {
    println!("Here are the current tasks:");
    view_all_tasks::view_all();
    let all_todos: Vec<String> = helpers::read_a_file(super::FILE_PATH);
    let mut task_id: String =
        helpers::get_input_from_cli("What's the number of the task you want to delete? ");
    let mut task_id_parsed: Result<usize, ParseIntError> = task_id.parse::<usize>();
    while task_id_parsed.is_err() || (task_id_parsed.as_ref().unwrap() - 1 >= all_todos.len()) {
        task_id =
            helpers::get_input_from_cli("Please enter a valid number for the task to be deleted ");
        task_id_parsed = task_id.parse::<usize>();
    }
    let index: usize = task_id_parsed.unwrap();
    let selected_task: String = helpers::get_single_task(index - 1);
    let todo: ToDoItem = serde_json::from_str(&selected_task).unwrap();
    println!("Are you sure you want to delete this task: {:?}", todo);
    let mut result_string: String = helpers::get_input_from_cli("1. No 2. Yes: ");
    let mut result_string_parsed: Result<i32, ParseIntError> = result_string.parse::<i32>();
    while result_string_parsed.is_err()
        || result_string_parsed
            .as_ref()
            .map_or(false, |value| *value > 2 || *value < 1)
    {
        result_string = helpers::get_input_from_cli("1. No 2. Yes: ");
        result_string_parsed = result_string.parse::<i32>();
    }
    let delete_confirmation = result_string_parsed.unwrap();
    if delete_confirmation == 1 {
        println!("Nothing deleted, confirmation was a 'No'");
    } else if delete_confirmation == 2 {
        let mut updated_vector: Vec<String> = Vec::new();
        let mut counter: usize = 1;
        for item in all_todos {
            if counter != index {
                updated_vector.push(item);
            }
            counter += 1;
        }
        helpers::write_to_file(updated_vector, true);
    }
}
