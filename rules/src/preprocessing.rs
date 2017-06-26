use unicode_normalization::UnicodeNormalization;
use unicode_categories::UnicodeCategories;
use rustling::Preprocessor;

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

impl Preprocessor for PreprocessingOption {
    fn run(&self, input: &str) -> String {
        match self {
            &PreprocessingOption::RemoveDiacritics => {
                input.nfd()
                    .filter(|c| !c.is_mark_nonspacing() ) // (Mn)
                    .nfc()
                    .collect()
            },
            &PreprocessingOption::Lowercase => {
                input.to_lowercase()
            },
        }
    }
}

impl Preprocessor for PreprocessingOptions {
    fn run(&self, input: &str) -> String {
        self.0.iter()
            .fold(input.to_string(), |prev, next| {
                println!("{:?}", prev);
                next.run(prev.as_ref())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_regex() {
        let regex = Regex::new(r#"0*(\d+) ?(ere?|eme|ieme)"#).unwrap();
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
