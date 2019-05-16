fn count_characters(s: String) {
    print!(
        "{text} has {length} characters\n",
        text = s,
        length = s.len()
    )
}

fn print_quotes(quote: String) {
    let mut obi_said: String = "Obi shit head said \"".to_owned();
    let closing_quote_mark = "\"";
    obi_said.push_str(&quote);
    obi_said.push_str(&closing_quote_mark);
    println!("{}", obi_said);
}

#[cfg(test)]
mod tests {
    use crate::count_characters;
    use crate::print_quotes;

    #[test]
    fn should_count_characters() {
        count_characters(String::from("Homer"))
    }

    #[test]
    fn should_print_quote() {
        print_quotes(String::from("These aren't the droids you're looking for"))
    }
}
