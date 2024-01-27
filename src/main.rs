use std::error::Error;
use std::io::Write;
struct WordCounter<'a> {
    text: &'a str,
}

impl<'a> WordCounter<'a> {
    // `new` function to create a new WordCounter instance
    fn new(text: &'a str) -> WordCounter<'a> {
        WordCounter { text }
    }

    // `count_words` function to count the number of words in the text
    fn count_words(&self) -> Result<usize, Box<dyn Error>> {
        if self.text.is_empty() {
            return Err("No text provided".into());
        }
        return Ok(self.text.split_whitespace().count());
    }
}

fn main() {
    println!("Please type some text to count the words");

    let mut input = String::new();
    std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input
    std::io::stdin().read_line(&mut input).unwrap();
    let word_counter = WordCounter::new(&input);

    if let Ok(count) = word_counter.count_words() {
        println!("Word count: {}", count);
    } else {
        println!("Error: {}", word_counter.count_words().unwrap_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let word_counter = WordCounter::new("");
        assert!(word_counter.count_words().is_err());
    }

    #[test]
    fn test_single_word() {
        let word_counter = WordCounter::new("Hello");
        assert_eq!(word_counter.count_words().unwrap(), 1);
    }

    #[test]
    fn test_multiple_words() {
        let word_counter = WordCounter::new("Hello World");
        assert_eq!(word_counter.count_words().unwrap(), 2);
    }
}
