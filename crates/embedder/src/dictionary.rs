use std::collections::HashMap;

use crate::tokenizer::Token;

#[derive(Debug)]
pub struct HashDictionary<'a> {
    embeddings: HashMap<Token<'a>, usize>,
}

impl<'a> HashDictionary<'a> {
    pub fn new(tokens: impl Iterator<Item = Token<'a>>) -> Self {
        let mut embeddings = HashMap::new();
        for token in tokens {
            let next_id = embeddings.len();
            embeddings.entry(token).or_insert(next_id);
        }
        Self { embeddings }
    }
}

pub trait Dictionary {
    fn get_embedding(&self, token: &Token) -> Option<f32>;
}

impl<'a> Dictionary for HashDictionary<'a> {
    fn get_embedding(&self, token: &Token) -> Option<f32> {
        self.embeddings.get(token).map(|id| *id as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_dictionary() {
        let tokens = Token::many(["Hello", "world"]);
        let dictionary = HashDictionary::new(tokens.into_iter());

        assert_eq!(dictionary.get_embedding(&Token("Hello")), Some(0.0));
        assert_eq!(dictionary.get_embedding(&Token("world")), Some(1.0));
        assert_eq!(dictionary.get_embedding(&Token("unknown")), None);
    }
}
