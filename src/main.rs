use std::process;
mod confirm;
mod delete;
mod path;

fn main() {
    loop {
        let path = path::get_path();
        if path == "exit" {
            process::exit(0);
        }
        let confirmation = confirm::request_confirmation(&path);
        if confirmation == "yes" {
            delete::delete(&path);
        }
    }
}
