use crate::helpers;
use crate::to_do_struct::{ToDoItem, ToDoStatus};
use crate::view_all_tasks;

pub fn update_todo() {
    println!("Here are the current tasks:");
    view_all_tasks::view_all();
    let all_todos = helpers::read_a_file(super::FILE_PATH);
    let mut task_id =
        helpers::get_input_from_cli("What's the number of the task you want to edit? ");
    let mut task_id_parsed = task_id.parse::<usize>();
    while task_id_parsed.is_err() || (task_id_parsed.as_ref().unwrap() - 1 >= all_todos.len()) {
        task_id = helpers::get_input_from_cli(
            "Please enter a valid number, that's the task you want to edit? ",
        );
        task_id_parsed = task_id.parse::<usize>();
    }
    println!("Selected id: {:?}", task_id_parsed);
    let index = task_id_parsed.unwrap();
    let selected_task = helpers::get_single_task(index - 1);
    let mut todo: ToDoItem = serde_json::from_str(&selected_task).unwrap();
    println!("Selected task {:?}", todo);
    // println!("Current title: {}", todo.title);
    let title_input = helpers::get_input_from_cli(
        "To update the title, enter the new title and press enter;
    \nTo leave as is, don't type anything, just press enter: ",
    );
    if title_input.len() > 0 {
        todo.title = title_input;
    }
    let desc_input = helpers::get_input_from_cli(
        "To update the description, enter the new description and press enter;
    \nTo leave as is, don't type anything, just press enter: ",
    );
    if desc_input.len() > 0 {
        todo.description = desc_input;
    }
    let mut get_status_string = helpers::get_input_from_cli(
        "Enter a number for the status you want to update to:\n
        1. Pending
        2. Started
        3. Completed
        4. Cancelled
        5. Leave as it is\n: ",
    );
    let mut get_status_parsed = get_status_string.parse::<i32>();
    while get_status_parsed.is_err()
        || get_status_parsed
            .as_ref()
            .map_or(false, |status| *status > 4 || *status < 1)
    {
        println!("Invalid input");
        get_status_string = helpers::get_input_from_cli(
            "Enter a number for the status you want to update to:\n
            1. Pending
            2. Started
            3. Completed
            4. Cancelled
            5. Leave as it is\n: ",
        );
        get_status_parsed = get_status_string.parse::<i32>();
    }
    println!("Selected status {}", get_status_parsed.as_ref().unwrap());
    println!("Updated todo {:?}", todo);
    let updated_status: ToDoStatus = match get_status_parsed.unwrap() {
        1 => ToDoStatus::Pending,
        2 => ToDoStatus::Started,
        3 => ToDoStatus::Completed,
        4 => ToDoStatus::Cancelled,
        _ => todo.status,
    };
    todo.status = updated_status;
    let mut todos_to_write_to_file: Vec<String> = Vec::new();
    let mut counter: usize = 0;
    for item in all_todos {
        counter += 1;
        if counter == index {
            let serialized = serde_json::to_string(&todo).unwrap();
            todos_to_write_to_file.push(serialized);
        } else {
            todos_to_write_to_file.push(item);
        }
    }
    helpers::write_to_file(todos_to_write_to_file, true);
    println!("Here are the UPDATED tasks:");
    view_all_tasks::view_all();
}
