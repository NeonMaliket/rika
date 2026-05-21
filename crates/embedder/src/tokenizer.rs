#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Token<'a>(pub(crate) &'a str);

impl<'a> Token<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn many(strs: impl IntoIterator<Item = &'a str>) -> Vec<Self> {
        strs.into_iter().map(Token).collect()
    }
}

pub trait Tokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> impl Iterator<Item = Token<'a>>;
}

pub struct DefaultTokenizer;

impl Tokenizer for DefaultTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> impl Iterator<Item = Token<'a>> {
        text.split_whitespace().map(Token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_tokens() {
        let tokenizer = DefaultTokenizer;
        let subject = "Hello world";
        let tokens = tokenizer.tokenize(subject).collect::<Vec<_>>();

        assert_eq!(tokens, Token::many(["Hello", "world"]));
    }

    #[test]
    fn test_default_tokens_empty() {
        let tokenizer = DefaultTokenizer;
        let subject = "";
        let tokens = tokenizer.tokenize(subject).collect::<Vec<_>>();
        assert_eq!(tokens, Vec::<Token>::new());
    }

    #[test]
    fn test_default_tokens_with_spec_symbols() {
        let tokenizer = DefaultTokenizer;
        let subject = "Hello, world! This is a test.";
        let tokens = tokenizer.tokenize(subject).collect::<Vec<_>>();
        assert_eq!(
            tokens,
            Token::many(["Hello,", "world!", "This", "is", "a", "test."])
        );
    }
}
