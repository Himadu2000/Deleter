use std::io;

pub fn request_confirmation(location: &str) -> String {
    println!("Are you sure delete, {} | Write (yes)", location);
    let mut anwser = String::new();
    io::stdin()
        .read_line(&mut anwser)
        .expect("Failed to read line");
    let anwser = anwser.trim();
    return anwser.to_string();
}
