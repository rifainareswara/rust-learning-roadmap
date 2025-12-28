use std::{
    fs::File,
    io::{Read},
};

fn main() {
    let contents = read_file("example.txt");

    // Ok, Err
    // Option<Some,  None>
    match contents {
        Ok(contents) => {
            println!("{}", contents);
        }
        Err(error) => {
            panic!("There was a problem reading the file: {:?}", error);
        }
    }
}

fn read_file (filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}