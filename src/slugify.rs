pub fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());

    for c in text.chars() {
        let replacement = match c {
            ' ' => Some("-"),
            'à' | 'À' | 'á' | 'Á' | 'â' | 'Â' | 'ã' | 'ä' | 'Ä' | 'Å' | 'å' | 'ª' => {
                Some("a")
            }
            'è' | 'È' | 'é' | 'É' | 'ê' | 'Ê' | 'ë' | 'Ë' => Some("e"),
            'ì' | 'Ì' | 'í' | 'Í' | 'î' | 'Î' | 'ï' | 'Ï' => Some("i"),
            'ò' | 'Ò' | 'ó' | 'Ó' | 'ô' | 'Ô' | 'õ' | 'Õ' | 'ö' | 'Ö' | 'º' => Some("o"),
            'ù' | 'Ù' | 'ú' | 'Ú' | 'û' | 'Û' | 'ü' | 'Ü' => Some("u"),
            'ç' | 'Ç' => Some("c"),
            _ => None,
        };

        if let Some(replacement) = replacement {
            slug.push_str(replacement);
            continue;
        }

        match c.to_lowercase().last().unwrap() {
            a @ 'a'..='z' | a @ '0'..='9' => slug.push(a),
            _ => (),
        }
    }

    return slug;
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn slugify_test() {
        let slug = slugify("àÀáÁâÂãäÄÅåªèÈéÉêÊëËìÌíÍîÎïÏòÒóÓôÔõÕöÖºùÙúÚûÛüÜçÇ");

        assert_eq!(slug, "aaaaaaaaaaaaeeeeeeeeiiiiiiiiooooooooooouuuuuuuucc")
    }
}
