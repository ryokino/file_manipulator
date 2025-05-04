use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command: &str = &args[1];

    match command {
        "reverse" => {
            let input_path = &args[2];
            let output_path = &args[3];

            let mut file = File::open(input_path).expect("file not found");

            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            let reversed_contents = contents.chars().rev().collect::<String>();

            let path = Path::new(output_path);

            let mut output_file = File::create(path).expect("failed to create file");

            output_file
                .write_all(reversed_contents.as_bytes())
                .expect("failed to write to file");
        }
        "copy" => {
            let input_path = &args[2];
            let output_path = &args[3];

            let mut file = File::open(input_path).expect("file not found");

            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            let path = Path::new(output_path);

            let mut output_file = File::create(path).expect("failed to create file");

            output_file
                .write_all(contents.as_bytes())
                .expect("failed to write to file");
        }
        "duplicate-contents" => {
            let input_path = &args[2];
            let n = args[3].parse::<usize>().unwrap();

            let mut file = File::open(input_path).expect("file not found");

            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            let duplicated_contents = contents.repeat(n as usize);

            let path = Path::new("duplicated_contents_output.txt");

            let mut output_file = File::create(path).expect("failed to create file");

            output_file
                .write_all(duplicated_contents.as_bytes())
                .expect("failed to write to file");
        }
        "replace-string" => {
            let input_path = &args[2];
            let old_string = &args[3];
            let new_string = &args[4];

            let mut file = File::open(input_path).expect("file not found");

            let mut contents = String::new();

            file.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            let replaced_contents = contents.replace(old_string, new_string);

            let path = Path::new("replaced_contents_output.txt");

            let mut output_file = File::create(path).expect("failed to create file");

            output_file
                .write_all(replaced_contents.as_bytes())
                .expect("failed to write to file");
        }
        _ => {
            println!("Invalid command");
            return;
        }
    }
}
