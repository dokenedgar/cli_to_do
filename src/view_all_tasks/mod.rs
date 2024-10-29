use crate::helpers;
use crate::to_do_struct::ToDoItem;

/**
 * CONTINUE HERE!!
 */
pub fn view_all() {
    let file_path_string: &str = super::FILE_PATH;
    let todo_items = helpers::read_a_file(file_path_string);
    println!("\nOUTPUT\n");
    let mut counter = 0;
    for item in todo_items {
        let deserialized: ToDoItem = serde_json::from_str(&item).unwrap();
        counter += 1;

        println!(
            "{counter}. Title: {}.\nDescription: {}.\nStatus: {:?}\n",
            deserialized.title, deserialized.description, deserialized.status
        )
    }
}
