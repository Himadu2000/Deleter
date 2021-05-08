use std::fs;

pub fn delete(path: &str) {
    match remove_directory(path) {
        Ok(_value) => {
            println!("Your folder: {} has been deleted!", path);
        }
        Err(error) => {
            println!("{}", error)
        }
    }
}

fn remove_directory(path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}
