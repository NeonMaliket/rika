use crate::dictionary::Dictionary;
use crate::tokenizer::Token;

pub trait Embedder {
    fn embed<'a>(&self, tokens: impl Iterator<Item = Token<'a>>) -> Vec<f32>;
}

pub struct DefaultEmbedder {
    dict: Box<dyn Dictionary>,
}

impl Embedder for DefaultEmbedder {
    fn embed<'a>(&self, tokens: impl Iterator<Item = Token<'a>>) -> Vec<f32> {
        tokens
            .filter_map(|token| self.dict.get_embedding(&token))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::HashDictionary;
    use crate::tokenizer::{DefaultTokenizer, Tokenizer};

    use super::*;

    #[test]
    fn test_default_embedder() {
        let text = "Hello unknown world your mom";
        let tokenizer = DefaultTokenizer;
        let tokens = tokenizer.tokenize(text);

        let dictionary = HashDictionary::new(tokens);

        let embedder = DefaultEmbedder {
            dict: Box::new(dictionary),
        };

        let subject = "Hello world";
        let tokens = tokenizer.tokenize(subject);
        let embeddings = embedder.embed(tokens);

        assert_eq!(embeddings, vec![0.0, 2.0]);
    }
}
