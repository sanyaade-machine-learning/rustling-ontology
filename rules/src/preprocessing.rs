use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;
use rustling::{Preprocessor, PreprocessedInput};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PreprocessingOptions(Vec<PreprocessingOption>);

impl PreprocessingOptions {
    pub fn new(options: Vec<PreprocessingOption>) -> PreprocessingOptions {
        PreprocessingOptions(options)
    }
}

#[derive(Clone, Debug)]
pub enum PreprocessingOption {
    NormalizeWhitespace,
    RemoveDiacritics,
    Lowercase,
}

impl PreprocessingOption {
    fn run(&self, input: PreprocessedInput) -> PreprocessedInput {
        match self {
            &PreprocessingOption::NormalizeWhitespace => normalize_whitespace(input),
            &PreprocessingOption::Lowercase => lowercase(input),
            &PreprocessingOption::RemoveDiacritics => remove_diacritics(input),
        }
    }
}

fn lowercase(input: PreprocessedInput) -> PreprocessedInput {
    let mut input_cursor = 0;
    let mut output_cursor = 0;
    let mut byte_mapping = HashMap::new();
    let output = input.preprocessed_input
        .chars()
        .filter_map(|c| {
            let output_c = c.to_string().to_lowercase().chars().next();
            byte_mapping.insert(output_cursor, input.map_byte(input_cursor).unwrap());
            input_cursor += c.len_utf8();
            output_cursor += output_c.map(|c| c.len_utf8()).unwrap_or(0);
            output_c
        })
        .collect::<String>();
    byte_mapping.insert(output.len(), input.original_input.len());
    PreprocessedInput {
        original_input: input.original_input,
        preprocessed_input: output,
        byte_mapping: byte_mapping
    }
}

fn normalize_whitespace(input: PreprocessedInput) -> PreprocessedInput {
    let mut input_cursor = 0;
    let mut output_cursor = 0;
    let mut byte_mapping = HashMap::new();
    let mut previous_space = false;
    let output = input.preprocessed_input
        .chars()
        .filter_map(|c| {
            byte_mapping.insert(output_cursor, input.map_byte(input_cursor).unwrap());
            input_cursor += c.len_utf8();
            if previous_space && c.is_whitespace() {
                None
            } else {
                previous_space = c.is_whitespace();
                output_cursor += c.len_utf8();
                Some(c)
            }
        })
        .collect::<String>();
    byte_mapping.insert(output.len(), input.original_input.len());
    PreprocessedInput {
        original_input: input.original_input,
        preprocessed_input: output,
        byte_mapping: byte_mapping
    }
}

fn remove_diacritics(input: PreprocessedInput) -> PreprocessedInput {
    let mut input_cursor = 0;
    let mut output_cursor = 0;
    let mut byte_mapping = HashMap::new();
    let output = input.preprocessed_input
        .chars()
        .filter_map(|c| {
            let output_c = c.to_string()
                .nfd()
                .filter(|c| !c.is_mark_nonspacing())
                .nfc().next();
            byte_mapping.insert(output_cursor, input.map_byte(input_cursor).unwrap());
            input_cursor += c.len_utf8();
            output_cursor += output_c.map(|c| c.len_utf8()).unwrap_or(0);
            output_c
        })
        .collect::<String>();
    byte_mapping.insert(output.len(), input.original_input.len());
    PreprocessedInput {
        original_input: input.original_input,
        preprocessed_input: output,
        byte_mapping: byte_mapping
    }
}

impl Preprocessor for PreprocessingOptions {
    fn run(&self, input: &str) -> PreprocessedInput {
        self.0.iter()
            .fold(PreprocessedInput::no_preprocessing(input), |prev, next| {
                next.run(prev)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    fn execute(regex: &Regex, sentence: &str) {
        let matches = regex.captures_iter(&sentence)
            .map(|cap| {
                cap.get(0).unwrap();
            })
            .collect::<Vec<_>>();
        println!("{:?}", matches);
    }

    #[test]
    fn test_remove_diacritics() {
        assert_eq!("abc", 
            PreprocessingOption::RemoveDiacritics.run("abc"));
        assert_eq!("abe", 
            PreprocessingOption::RemoveDiacritics.run("abé"));
        assert_eq!("오백칠십구",
            PreprocessingOption::RemoveDiacritics.run("오백칠십구"));
        assert_eq!("3eme",
            PreprocessingOption::RemoveDiacritics.run("3éme"));
    }

    #[test]
    fn test_lowercase()  {
        assert_eq!("abc", 
            PreprocessingOption::Lowercase.run("aBc"));
        assert_eq!("abc", 
            PreprocessingOption::Lowercase.run("ABc"));
    }

    #[test]
    fn test_lowercase_and_remove_diacritics() {
        assert_eq!("abe", PreprocessingOptions::new(vec![
            PreprocessingOption::Lowercase,
            PreprocessingOption::RemoveDiacritics,
            ]).run("aBé"));
    }
}
