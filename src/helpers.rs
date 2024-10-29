use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

pub fn check_file_exists(file_path_string: &str) -> bool {
    let file_path = create_path(file_path_string);
    let result: bool = file_path.is_file();
    // println!("File exists: {result}");
    result
}

pub fn read_a_file(file_path_string: &str) -> Vec<String> {
    let file_path = create_path(file_path_string);
    let mut items_vector: Vec<String> = Vec::new();
    // open a file
    let file = File::open(file_path);
    match file {
        Ok(opened_file) => {
            // create a buffered reader
            let reader = io::BufReader::new(opened_file);
            // read the file line by line
            for line in reader.lines() {
                let line = line;
                match line {
                    Ok(line_value) => {
                        // println!("{}", line_value);
                        items_vector.push(line_value);
                    }
                    Err(_) => println!("Error reading a line"),
                }
            }
            let total_lines = items_vector.len();
            println!("Total lines {total_lines}");
        }
        Err(_) => println!("Error opening file to read!"),
    }
    items_vector
}

pub fn write_to_file(content: Vec<String>, overwrite: bool) {
    let file_path_string = super::FILE_PATH;
    let file_path = create_path(file_path_string);
    let mut file = File::options().append(true).open(file_path);
    if overwrite {
        file = File::create(file_path);
    }
    match file {
        Ok(mut file) => {
            let mut content_to_write = String::new();
            for item_to_write in content {
                content_to_write = content_to_write + &item_to_write + "\n";
            }
            let write_result = file.write_all(content_to_write.as_bytes());
            match write_result {
                Ok(_) => println!("Write was successful"),
                Err(_) => println!("Write was NOT successful!"),
            }
        }
        Err(_) => println!("Error opening file"),
    }
}

pub fn create_path(path_string: &str) -> &Path {
    let path = Path::new(path_string);
    path
}

pub fn get_input_from_cli(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn create_file_if_not_exists(file_path_string: &str) {
    if !check_file_exists(file_path_string) {
        let file_path = create_path(file_path_string);
        let _ = File::create_new(file_path);
    }
}

pub fn get_single_task(index: usize) -> String {
    let all_todos = read_a_file(super::FILE_PATH);
    let todo_item = String::from(&all_todos[index]);
    todo_item
}
