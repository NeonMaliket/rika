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

        embeddings.insert(Token::unknown(), embeddings.len() as i32);
        embeddings.insert(Token::end_of_text(), embeddings.len() as i32);

        rev_embeddings.insert(rev_embeddings.len() as i32, Token::unknown());
        rev_embeddings.insert(rev_embeddings.len() as i32, Token::end_of_text());

        Self {
            embeddings,
            rev_embeddings,
        }
    }
}

pub trait Dictionary {
    fn get_embedding(&self, token: Token) -> i32;
    fn get_token<'a>(&'a self, embedding: i32) -> Token<'a>;
}

impl<'a> Dictionary for HashDictionary<'a> {
    fn get_embedding(&self, token: Token) -> i32 {
        self.embeddings
            .get(&token)
            .copied()
            .unwrap_or_else(|| self.embeddings[&Token::unknown()])
    }

    fn get_token(&self, embedding: i32) -> Token<'a> {
        self.rev_embeddings
            .get(&embedding)
            .cloned()
            .unwrap_or(Token::unknown())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_dictionary() {
        let tokens = Token::many(["Hello", "world"]);
        let dictionary = HashDictionary::new(tokens.into_iter());

        assert_eq!(dictionary.get_embedding(Token("Hello")), 0);
        assert_eq!(dictionary.get_embedding(Token("world")), 1);
        assert_eq!(dictionary.get_embedding(Token::unknown()), 2);
        assert_eq!(dictionary.get_embedding(Token::end_of_text()), 3);
    }

    #[test]
    fn test_hash_dictionary_rev() {
        let tokens = Token::many(["Hello", "world"]);
        let dictionary = HashDictionary::new(tokens.into_iter());

        assert_eq!(dictionary.get_token(0), Token("Hello"));
        assert_eq!(dictionary.get_token(1), Token("world"));
        assert_eq!(dictionary.get_token(2), Token::unknown());
        assert_eq!(dictionary.get_token(3), Token::end_of_text());
    }
}
