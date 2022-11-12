use crate::slugify::slugify;
use std::{fs::File, io::Write};

pub fn generate_post(name: String) -> std::io::Result<()> {
    let slug = slugify(&name);
    let path = "./content/".to_string();
    let ext = &".md".to_string();
    let file_name = path + &slug + ext;
    let mut file = File::create(file_name)?;
    let content = format!(
        "---\nlayout: post\ntitle: {}\n---\n\n# Write your content here!",
        name
    );

    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn build_post(name: String, content: String) -> std::io::Result<()> {
    let path = "./.build/".to_string();
    let mut file = File::create(path + &name)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}
