use crate::slugify::slugify;

pub fn generate_post(name: String) {
    let slug = slugify(&name);

    print!("Generate new post {}", slug);
}
