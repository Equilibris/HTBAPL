use crate::{
    errors::BaseErr,
    tokenizer::{Loc, Token, TokenStream},
};

#[derive(Debug, Clone)]
pub enum Partitioner {
    ExpressionSeperator,

    Expression(TokenStream),
    Statement(PartitionStream),
    RoundContainer(PartitionStream),
    SquareContainer(PartitionStream),
    CurlyContainer(PartitionStream),
}

pub type PartitionStream = Vec<Partitioner>;

fn process(
    token_stream: &mut std::slice::Iter<(Token, Loc)>,
    output: &mut PartitionStream,
    stop_on: Token,
    fail_on: Vec<Token>,
) -> Result<(), (Token, Loc)> {
    let mut expression: TokenStream = Vec::with_capacity(16);
    let mut statement = Vec::with_capacity(8);

    while let Some((token, loc)) = token_stream.next() {
        if fail_on.contains(token) {
            return Err((token.clone(), loc.clone()));
        }

        if token.clone() == stop_on {
            statement.push(Partitioner::Expression(expression.clone()));
            output.push(Partitioner::Statement(statement));
            return Ok(());
        }

        match token {
            Token::OpenSquareBracket => {
                statement.push(Partitioner::Expression(expression.clone()));
                expression.clear();

                let mut v = Vec::with_capacity(16);
                match process(
                    token_stream,
                    &mut v,
                    Token::CloseSquareBracket,
                    vec![
                        Token::EOF,
                        Token::CloseRoundBracket,
                        Token::CloseCurlyBracket,
                    ],
                ) {
                    Ok(()) => {}
                    Err(e) => return Err(e),
                }
                statement.push(Partitioner::SquareContainer(v))
            }
            Token::OpenRoundBracket => {
                statement.push(Partitioner::Expression(expression.clone()));
                expression.clear();

                let mut v = Vec::with_capacity(16);
                match process(
                    token_stream,
                    &mut v,
                    Token::CloseRoundBracket,
                    vec![
                        Token::EOF,
                        Token::CloseSquareBracket,
                        Token::CloseCurlyBracket,
                    ],
                ) {
                    Ok(()) => {}
                    Err(e) => return Err(e),
                }
                statement.push(Partitioner::RoundContainer(v))
            }
            Token::OpenCurlyBracket => {
                statement.push(Partitioner::Expression(expression.clone()));
                expression.clear();

                let mut v = Vec::with_capacity(16);
                match process(
                    token_stream,
                    &mut v,
                    Token::CloseCurlyBracket,
                    vec![
                        Token::EOF,
                        Token::CloseRoundBracket,
                        Token::CloseSquareBracket,
                    ],
                ) {
                    Ok(()) => {}
                    Err(e) => return Err(e),
                }
                statement.push(Partitioner::CurlyContainer(v))
            }
            Token::Diamond => {
                statement.push(Partitioner::Expression(expression.clone()));
                expression.clear();
                statement.push(Partitioner::ExpressionSeperator);
            }
            Token::NL => {
                if expression.len() > 0 {
                    statement.push(Partitioner::Expression(expression.clone()));
                    expression.clear();
                    // TODO: should this be clone?
                }
                if statement.len() > 0 {
                    output.push(Partitioner::Statement(statement.clone()));
                    statement.clear();
                }
            }
            _ => expression.push((token.clone(), loc.clone())),
        }
    }
    // Unreachable
    Ok(())
}

pub fn tokenize_to_partition<'a>(
    token_stream: TokenStream,
) -> Result<PartitionStream, BaseErr<'a>> {
    let mut token_stream = token_stream.iter();
    let mut output = Vec::with_capacity(128);

    if let Err((token, loc)) = process(&mut token_stream, &mut output, Token::EOF, vec![]) {
        Err(BaseErr::new(
            "Unexpected token", // format!("Unexpected token {:?} at {:?}", token, loc).as_str() as &'a str,
        ))
    } else {
        Ok(output)
    }
}

pub fn deserialize_to_string(partition_stream: PartitionStream) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::bracket_partitioner::tokenize_to_partition;
    use crate::normalizer::normalize_apl_code;
    use crate::tokenizer::tokenize;

    #[test]
    fn it_partitions() {
        let stream = tokenize(normalize_apl_code(
            "nDCube ← {v←⍵ ⋄ ⍺{⍺=1:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}\nnDCube ← {v←⍵ ⋄ ⍺{⍺=1:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string(),
        ));

        println!("{:?}", tokenize_to_partition(stream).unwrap());
    }
}
