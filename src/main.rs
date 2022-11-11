mod post;
mod slugify;

use crate::post::generate_post;
use std::{env, fs};

static FM_SIGNAL: &str = "---";

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

fn handle_build() {
    let content_path = fs::read_dir("./content");

    match content_path {
        Ok(_) => {
            for path in content_path.unwrap() {
                let file = path.unwrap().path().display().to_string();
                let content = fs::read_to_string(file).unwrap();
                let lines = content.split("\n");
                let mut front_matter: Vec<&str> = Vec::new();
                let mut front_matter_started = false;
                let mut front_matter_ended = false;
                let mut markdown: Vec<&str> = Vec::new();

                for line in lines {
                    if line == FM_SIGNAL && !front_matter_started {
                        front_matter_started = true;
                        continue;
                    }

                    if line == FM_SIGNAL && front_matter_started {
                        front_matter_ended = true;
                        continue;
                    }

                    if front_matter_started && !front_matter_ended {
                        front_matter.push(line);
                    } else {
                        markdown.push(line);
                    }
                }

                dbg!(front_matter);
                dbg!(markdown);
            }
        }
        Err(_) => println!("Error"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Option<String> = args.get(1).cloned();
    let arg: Option<String> = args.get(2).cloned();

    match command {
        Some(a) => match a.as_str() {
            "-g" | "--generate" => handle_gerenator(arg),
            "-b" | "--build" => handle_build(),
            _ => println!("Command not found"),
        },
        None => println!("Command not found"),
    }
}
