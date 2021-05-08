use std::io;

pub fn get_path() -> String {
    println!("Please input your folder location to delete. Write exit to quit.");

    let mut location = String::new();

    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    let location = location.trim();
    return location.to_string();
}
