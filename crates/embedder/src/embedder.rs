use crate::dictionary::Dictionary;
use crate::tokenizer::Token;

pub trait Embedder {
    fn encode<'a>(&self, tokens: impl Iterator<Item = Token<'a>>) -> Vec<i32>;

    fn decode<'a>(&'a self, embedings: &[i32]) -> impl Iterator<Item = Token<'a>>;
}

pub struct DefaultEmbedder {
    dict: Box<dyn Dictionary>,
}

impl Embedder for DefaultEmbedder {
    fn encode<'a>(&self, tokens: impl Iterator<Item = Token<'a>>) -> Vec<i32> {
        let eot = self.dict.get_embedding(Token::end_of_text());
        tokens
            .map(|token| self.dict.get_embedding(token))
            .chain(std::iter::once(eot))
            .collect()
    }

    fn decode<'a>(&'a self, embedings: &[i32]) -> impl Iterator<Item = Token<'a>> {
        embedings
            .iter()
            .map(|embedding| self.dict.get_token(*embedding))
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

        let subject = "Hello world lkj";
        let tokens = tokenizer.tokenize(subject);
        let embeddings = embedder.encode(tokens);

        assert_eq!(embeddings, vec![0, 2, 5, 6]);
    }

    #[test]
    fn test_default_embedder_decode() {
        let text = "Hello unknown world your mom";
        let tokenizer = DefaultTokenizer;
        let tokens = tokenizer.tokenize(text);

        let dictionary = HashDictionary::new(tokens);

        let embedder = DefaultEmbedder {
            dict: Box::new(dictionary),
        };

        let subject = "Hello world mm";
        let tokens = tokenizer.tokenize(subject);
        let embeddings = embedder.encode(tokens);

        let decoded_tokens: Vec<_> = embedder.decode(&embeddings).collect();

        assert_eq!(
            decoded_tokens,
            Token::many(["Hello", "world", "<|UNK|>", "<|EOT|>"])
        );
    }
}
