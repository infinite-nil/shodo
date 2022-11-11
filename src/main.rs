mod post;
mod slugify;

use crate::post::generate_post;
use std::env;

fn handle_gerenator(file: Option<String>) {
    match file {
        Some(f) => {
            let post = generate_post(f);

            match post {
                Ok(_) => println!("Content created"),
                Err(e) => println!("Something went wrong: {:?}", e),
            }
        }
        None => print!("Nope"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Option<String> = args.get(1).cloned();
    let arg: Option<String> = args.get(2).cloned();

    match command {
        Some(a) => match a.as_str() {
            "-g" | "--generate" => handle_gerenator(arg),
            _ => println!("Command not found"),
        },
        None => println!("Command not found"),
    }
}
