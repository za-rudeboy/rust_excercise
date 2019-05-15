fn count_characters(s: String) {
    print!("{text} has {length} characters\n", text = s, length = s.len())
}





#[cfg(test)]
mod tests {
    use crate::count_characters;

    #[test]
    fn blah() {
        count_characters(String::from("Homer"))
    }
}
