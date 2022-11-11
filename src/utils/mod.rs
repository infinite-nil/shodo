use std::fs;

static FM_SIGNAL: &str = "---";

pub fn get_file_content(file: String) -> (Vec<String>, Vec<String>) {
    let content = fs::read_to_string(file).unwrap();
    let lines = content.split("\n");
    let mut front_matter_started = false;
    let mut front_matter_ended = false;
    let mut front_matter: Vec<String> = Vec::new();
    let mut markdown: Vec<String> = Vec::new();

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
            front_matter.push(line.to_string());
        } else {
            markdown.push(line.to_string());
        }
    }

    (front_matter, markdown)
}
