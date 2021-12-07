use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Unexpected token {0:?} at {1}")]
    UnexpectedToken(crate::tokenizer::Token, crate::tokenizer::Loc),
}
