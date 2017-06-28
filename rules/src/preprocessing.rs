use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;
use rustling::{Preprocessor, PreprocessedInput};

#[derive(Clone, Debug)]
pub struct PreprocessingOptions(Vec<PreprocessingOption>);

impl PreprocessingOptions {
    pub fn new(options: Vec<PreprocessingOption>) -> PreprocessingOptions {
        PreprocessingOptions(options)
    }
}

#[derive(Clone, Debug)]
pub enum PreprocessingOption {
    RemoveDiacritics,
    Lowercase,
}

impl PreprocessingOption {
    fn run(&self, input: &PreprocessedInput) -> PreprocessedInput {
        unimplemented!();
        // match self {
        //     &PreprocessingOption::RemoveDiacritics => {
        //         input.nfd()
        //             .filter(|c| !c.is_mark_nonspacing() ) // (Mn)
        //             .nfc()
        //             .collect()
        //     },
        //     &PreprocessingOption::Lowercase => {
        //         input.to_lowercase()
        //     },
        // }
    }
}

fn remove_diacritics(input: &PreprocessedInput) -> PreprocessedInput {
    unimplemented!()
}


impl Preprocessor for PreprocessingOptions {
    fn run(&self, input: &str) -> PreprocessedInput {
        self.0.iter()
            .fold(PreprocessedInput::no_preprocessing(input), |prev, next| {
                next.run(&prev)
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
    fn test_regex() {
        let regex = Regex::new(r#"0*(\d+) ?(ere?|eme|ieme)"#).unwrap();
        execute(&regex, PreprocessingOption::RemoveDiacritics.run("3éme").as_ref());
        execute(&regex, PreprocessingOption::RemoveDiacritics.run("3eme").as_ref());
        println!("{:?}", regex.find(PreprocessingOption::RemoveDiacritics.run("3éme").as_ref()));
        println!("{:?}", regex.find(PreprocessingOption::RemoveDiacritics.run("3eme").as_ref()));
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
