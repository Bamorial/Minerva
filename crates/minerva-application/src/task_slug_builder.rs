use minerva_domain::TaskSlug;

pub fn build_slug(title: &str) -> Option<TaskSlug> {
    let mut slug = String::new();
    for ch in title.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            slug.push(ch);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    (!slug.is_empty()).then(|| TaskSlug::new(slug).expect("generated slug is valid"))
}
