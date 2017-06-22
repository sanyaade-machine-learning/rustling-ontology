enum PreprocessingOption {
    RemoveDiacritics,
    Lowercase,
}

impl PreprocessingOption {
    fn run<'a>(&self, input: &'a str) -> &'a str {
        unimplemented!();
    }
}