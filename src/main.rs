fn count_characters(s: String) {
    println!(
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

fn madlib(noun: String, verb: String, adjective: String, adverb: String) {
    println!("Do you {} your {} {} {}?", noun, verb, adjective, adverb)
}

fn simple_calculator(first: String, second: String) {
    let first_int = first.parse::<i32>().unwrap();
    let second_int = first.parse::<i32>().unwrap();

    println!("{}", first_int - second_int);
    println!("{}", first_int + second_int);
    println!("{}", first_int / second_int);
    println!("{}", first_int * second_int);
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::count_characters;
    use crate::madlib;
    use crate::print_quotes;
    use crate::simple_calculator;

    #[test]
    fn should_count_characters() {
        count_characters(String::from("Homer"))
    }

    #[test]
    fn should_print_quote() {
        print_quotes(String::from("These aren't the droids you're looking for"))
    }

    #[test]
    fn should_madlib() {
        madlib(
            String::from("dog"),
            String::from("walk"),
            String::from("blue"),
            String::from("quickly"),
        )
    }

    #[test]
    fn should_calculate_numbers() {
        simple_calculator(String::from("1"), String::from("23"))
    }
}
