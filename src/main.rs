mod post;
mod slugify;
mod utils;

use post::generate_post;
use pulldown_cmark::{html, Options, Parser};
use std::{env, fs};
use utils::get_file_content;

use crate::post::build_post;

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
                let tmp_file = path.unwrap();
                let name = str::replace(tmp_file.file_name().to_str().unwrap(), ".md", ".html");
                let file = tmp_file.path().display().to_string();
                let (_, content): (Vec<String>, Vec<String>) = get_file_content(file);

                let text = content.join("\n");
                let parser = Parser::new_ext(&text, Options::all());
                let mut output = String::new();

                html::push_html(&mut output, parser);

                match build_post(name, output) {
                    Ok(_) => print!("OK"),
                    Err(_) => println!("Nope"),
                };
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
