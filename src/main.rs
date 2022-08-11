use arboard::Clipboard;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

fn reverse_brackets(text: &str) -> String {
    let mut text = text.to_owned();

    text = text.replace('(', "right_bracket");
    text = text.replace(')', "(");
    text = text.replace("right_bracket", ")");

    text = text.replace('[', "right_bracket");
    text = text.replace(']', "[");
    text = text.replace("right_bracket", "]");

    text = text.replace('{', "right_bracket");
    text = text.replace('}', "{");
    text = text.replace("right_bracket", "}");

    text
}

fn extract_numbers(s: &str) -> Option<HashSet<&str>> {
    // `lazy_static!` to run once and prevent loops from needlessly repeating this section
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\d+"#).unwrap();
    }

    let mut numbers_set = HashSet::new();

    for found_match in RE.find_iter(s) {
        numbers_set.insert(found_match.as_str());
    }

    Some(numbers_set)
}

fn reverse_numbers(text: &str) -> Option<String> {
    // Detect numbers, reverse them and replace them
    match extract_numbers(text) {
        Some(numbers_set) => {
            let mut text = text.to_owned();

            for number in numbers_set {
                text = text.replace(number, &reverse(number));
            }

            Some(text)
        }
        _ => None,
    }
}

fn reverse_clipboard(text: &str) -> String {
    text.lines()
        .map(|line| {
            if let Some(reversed_numbers_line) = reverse_numbers(line) {
                format!("{}\r\n", &reverse(&reversed_numbers_line))
            } else {
                format!("{}\r\n", &reverse(line))
            }
        })
        .map(|line| reverse_brackets(&line))
        .collect()
}

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    let content = clipboard.get_text().unwrap();

    clipboard.set_text(reverse_clipboard(&content)).unwrap();
}
