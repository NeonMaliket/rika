use std::collections::HashMap;

use crate::tokenizer::Token;

#[derive(Debug)]
pub struct HashDictionary<'a> {
    embeddings: HashMap<Token<'a>, i32>,
    rev_embeddings: HashMap<i32, Token<'a>>,
}

impl<'a> HashDictionary<'a> {
    pub fn new(tokens: impl Iterator<Item = Token<'a>>) -> Self {
        let mut embeddings = HashMap::new();
        let mut rev_embeddings = HashMap::new();
        for token in tokens {
            let next_id = embeddings.len() as i32;
            embeddings.entry(token).or_insert(next_id);
            rev_embeddings.entry(next_id).or_insert(token);
        }
        Self {
            embeddings,
            rev_embeddings,
        }
    }
}

pub trait Dictionary {
    fn get_embedding(&self, token: Token) -> Option<i32>;
    fn get_token<'a>(&'a self, embedding: i32) -> Option<Token<'a>>;
}

impl<'a> Dictionary for HashDictionary<'a> {
    fn get_embedding(&self, token: Token) -> Option<i32> {
        self.embeddings.get(&token).copied()
    }

    fn get_token(&self, embedding: i32) -> Option<Token<'a>> {
        self.rev_embeddings.get(&embedding).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_dictionary() {
        let tokens = Token::many(["Hello", "world"]);
        let dictionary = HashDictionary::new(tokens.into_iter());

        assert_eq!(dictionary.get_embedding(Token("Hello")), Some(0));
        assert_eq!(dictionary.get_embedding(Token("world")), Some(1));
        assert_eq!(dictionary.get_embedding(Token("unknown")), None);
    }

    #[test]
    fn test_hash_dictionary_rev() {
        let tokens = Token::many(["Hello", "world"]);
        let dictionary = HashDictionary::new(tokens.into_iter());

        assert_eq!(dictionary.get_token(0), Some(Token("Hello")));
        assert_eq!(dictionary.get_token(1), Some(Token("world")));
        assert_eq!(dictionary.get_token(2), None);
    }
}
