use crate::helpers;
use crate::to_do_struct::{ToDoItem, ToDoStatus};

/**
 * Todo
 * - ask user to enter title and then description
 * - set default status
 * - instantiate todo struct
 * - serialize using serde and save to file
 */
pub fn create_todo() {
    let title = helpers::get_input_from_cli("\nEnter ToDo title:\n");
    let description = helpers::get_input_from_cli("\nEnter ToDo description:\n");
    let todo = ToDoItem {
        title,
        description,
        status: ToDoStatus::Pending,
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    helpers::write_to_file(vec![serialized], false);
}
