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

    fn preprocess_input(option: PreprocessingOption, input: &str) -> String {
        option.run(PreprocessedInput::no_preprocessing(input)).preprocessed_input
    }

    #[test]
    fn test_remove_diacritics() {
        assert_eq!("abc", preprocess_input(PreprocessingOption::RemoveDiacritics, "abc"));
        assert_eq!("abc", preprocess_input(PreprocessingOption::RemoveDiacritics, "abc"));
        assert_eq!("abe", preprocess_input(PreprocessingOption::RemoveDiacritics, "abé"));
        assert_eq!("오백칠십구", preprocess_input(PreprocessingOption::RemoveDiacritics, "오백칠십구"));
        assert_eq!("3eme", preprocess_input(PreprocessingOption::RemoveDiacritics, "3éme"));
    }

    #[test]
    fn test_lowercase()  {
        assert_eq!("abc", preprocess_input(PreprocessingOption::Lowercase, "aBc"));
        assert_eq!("abc", preprocess_input(PreprocessingOption::Lowercase, "ABc"));
    }

    #[test]
    fn test_lowercase_and_remove_diacritics() {
        assert_eq!("abe", PreprocessingOptions::new(vec![
            PreprocessingOption::Lowercase,
            PreprocessingOption::RemoveDiacritics,
            ]).run("aBé").preprocessed_input);
    }

    #[test]
    fn test_same_result_despite_option_order() {
        let input = "fünf tausend ētė ₩ 유로 euro";
        let a = PreprocessingOptions::new(vec![
            PreprocessingOption::Lowercase,
            PreprocessingOption::RemoveDiacritics,
            PreprocessingOption::NormalizeWhitespace,
        ]);
        let b = PreprocessingOptions::new(vec![
            PreprocessingOption::Lowercase,
            PreprocessingOption::NormalizeWhitespace,
            PreprocessingOption::RemoveDiacritics,
        ]);
        let c = PreprocessingOptions::new(vec![
            PreprocessingOption::NormalizeWhitespace,
            PreprocessingOption::Lowercase,
            PreprocessingOption::RemoveDiacritics,
        ]);

        assert_eq!(a.run(input), b.run(input));
        assert_eq!(a.run(input), c.run(input));
        assert_eq!(c.run(input), b.run(input));
    }
}
