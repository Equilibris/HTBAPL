use crate::{errors::BaseErr, numeric_literal::NumericLiteral};

#[derive(Debug, Clone)]
pub struct Loc {
    line: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    NumericLiteral(NumericLiteral),
    StringLiteral(String),

    Comment(String), // ⍝

    OpenRoundBracket,
    OpenSquareBracket,
    OpenCurlyBracket,
    CloseRoundBracket,
    CloseSquareBracket,
    CloseCurlyBracket,

    NL, // \n

    Plus,            // +
    Minus,           // -
    Times,           // ×
    Divide,          // ÷
    Upstile,         // ⌈
    Downstile,       // ⌊
    Star,            // *
    ExclamationMark, // !
    Stile,           // |
    Log,             // ⍟
    Circle,          // ○
    Domino,          // ⌹
    UpTack,          // ⊥
    DownTack,        // ⊤
    QuestionMark,    // ?

    Tilde,                // ~
    LogicalAND,           // ∧
    LogicalOR,            // ∨
    LogicalNAND,          // ⍲
    LogicalNOR,           // ⍱
    LessThan,             // <
    GreaterThan,          // >
    LessThanOrEqualTo,    // ≤
    GreaterThanOrEqualTo, // ≥
    Equal,                // =
    NotEqual,             // ≠
    EqualUnderbar,        // ≡
    EqualUnderbarSlash,   // ≢

    Rho,              // ⍴
    Comma,            // ,
    CommaBar,         // ⍪
    CircleStile,      // ⌽
    CircleBar,        // ⊖
    Transpose,        // ⍉
    UpArrow,          // ↑
    DownArrow,        // ↓
    LeftShoe,         // ⊂
    LeftShoeUnderbar, // ⊆
    Epsilon,          // ∊

    Squad,        // ⌷
    RightShoe,    // ⊃
    Slash,        // /
    SlashBar,     // ⌿
    Backslash,    // \
    BackslashBar, // ⍀
    DownShoe,     // ∪
    UpShoe,       // ∩
    LeftTack,     // ⊣
    RightTack,    // ⊢

    Iota,            // ⍳
    IotaUnderbar,    // ⍸
    EpsilonUnderbar, // ⍷
    GradeUp,         // ⍋
    GradeDown,       // ⍒

    Diaeresis,      // ¨
    TildeDiaeresis, // ⍨
    StarDiaeresis,  // ⍣
    Dot,            // .
    Jot,            // ∘
    QuadEqual,      // ⌸
    JotDiaeresis,   // ⍤
    CircleDieresis, // ⍥
    QuadDiamond,    // ⌺
    At,             // @
    QuadColon,      // ⍠

    LeftArrow, // ←
    Zilde,     // ⍬
    Hydrant,   // ⍎
    Thorn,     // ⍕
    Diamond,   // ⋄
    Del,       // ∇
    Alpha,     // ⍺
    Omega,     // ⍵

    Colon, // :

    EOF,
}

pub type TokenStream = Vec<(Token, Loc)>;
type Stream<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn numeric_literal_extractor<'a>(
    stream: &mut Stream<'a>,
    output: &mut TokenStream,
    line: &mut usize,
    col: &mut usize,
) -> Result<(), BaseErr<'a>> {
    let mut counting_cycle = String::new();
    while let Some('0'..='9' | 'u' | 'f' | 'i' | 'E' | 'J' | '¯') = stream.peek() {
        counting_cycle.push(stream.next().unwrap());
        *col += 1;
    }
    output.push((
        Token::NumericLiteral(counting_cycle.parse::<NumericLiteral>()?),
        Loc {
            col: *col,
            line: *line,
        },
    ));

    Ok(())
}

pub fn tokenize(str: String) -> TokenStream {
    let mut output: TokenStream = Vec::with_capacity(str.len() / 2);

    let mut line: usize = 0;
    let mut col: usize = 0;

    let mut stream: Stream = str.chars().peekable();

    while let Some(token) = stream.peek() {
        col += 1;
        match *token {
            '0'..='9' => {
                numeric_literal_extractor(&mut stream, &mut output, &mut line, &mut col);
            }
            '\'' => {
                in_string_literal = true;
            }
            _ => {
                stream.next();
            }
            '\n' => {
                output.push((Token::NL, Loc { line, col }));
            }
            '+' => {
                output.push((Token::Plus, Loc { line, col }));
            }
            '-' => {
                output.push((Token::Minus, Loc { line, col }));
            }
            '×' => {
                output.push((Token::Times, Loc { line, col }));
            }
            '÷' => {
                output.push((Token::Divide, Loc { line, col }));
            }
            '⌈' => {
                output.push((Token::Upstile, Loc { line, col }));
            }
            '⌊' => {
                output.push((Token::Downstile, Loc { line, col }));
            }
            '*' => {
                output.push((Token::Star, Loc { line, col }));
            }
            '!' => {
                output.push((Token::ExclamationMark, Loc { line, col }));
            }
            '|' => {
                output.push((Token::Stile, Loc { line, col }));
            }
            '⍟' => {
                output.push((Token::Log, Loc { line, col }));
            }
            '○' => {
                output.push((Token::Circle, Loc { line, col }));
            }
            '⌹' => {
                output.push((Token::Domino, Loc { line, col }));
            }
            '⊥' => {
                output.push((Token::UpTack, Loc { line, col }));
            }
            '⊤' => {
                output.push((Token::DownTack, Loc { line, col }));
            }
            '?' => {
                output.push((Token::QuestionMark, Loc { line, col }));
            }
            '~' => {
                output.push((Token::Tilde, Loc { line, col }));
            }
            '∧' => {
                output.push((Token::LogicalAND, Loc { line, col }));
            }
            '∨' => {
                output.push((Token::LogicalOR, Loc { line, col }));
            }
            '⍲' => {
                output.push((Token::LogicalNAND, Loc { line, col }));
            }
            '⍱' => {
                output.push((Token::LogicalNOR, Loc { line, col }));
            }
            '<' => {
                output.push((Token::LessThan, Loc { line, col }));
            }
            '>' => {
                output.push((Token::GreaterThan, Loc { line, col }));
            }
            '≤' => {
                output.push((Token::LessThanOrEqualTo, Loc { line, col }));
            }
            '≥' => {
                output.push((Token::GreaterThanOrEqualTo, Loc { line, col }));
            }
            '=' => {
                output.push((Token::Equal, Loc { line, col }));
            }
            '≠' => {
                output.push((Token::NotEqual, Loc { line, col }));
            }
            '≡' => {
                output.push((Token::EqualUnderbar, Loc { line, col }));
            }
            '≢' => {
                output.push((Token::EqualUnderbarSlash, Loc { line, col }));
            }
            '⍴' => {
                output.push((Token::Rho, Loc { line, col }));
            }
            ',' => {
                output.push((Token::Comma, Loc { line, col }));
            }
            '⍪' => {
                output.push((Token::CommaBar, Loc { line, col }));
            }
            '⌽' => {
                output.push((Token::CircleStile, Loc { line, col }));
            }
            '⊖' => {
                output.push((Token::CircleBar, Loc { line, col }));
            }
            '⍉' => {
                output.push((Token::Transpose, Loc { line, col }));
            }
            '↑' => {
                output.push((Token::UpArrow, Loc { line, col }));
            }
            '↓' => {
                output.push((Token::DownArrow, Loc { line, col }));
            }
            '⊂' => {
                output.push((Token::LeftShoe, Loc { line, col }));
            }
            '⊆' => {
                output.push((Token::LeftShoeUnderbar, Loc { line, col }));
            }
            '∊' => {
                output.push((Token::Epsilon, Loc { line, col }));
            }
            '⌷' => {
                output.push((Token::Squad, Loc { line, col }));
            }
            '⊃' => {
                output.push((Token::RightShoe, Loc { line, col }));
            }
            '/' => {
                output.push((Token::Slash, Loc { line, col }));
            }
            '⌿' => {
                output.push((Token::SlashBar, Loc { line, col }));
            }
            '\\' => {
                output.push((Token::Backslash, Loc { line, col }));
            }
            '⍀' => {
                output.push((Token::BackslashBar, Loc { line, col }));
            }
            '∪' => {
                output.push((Token::DownShoe, Loc { line, col }));
            }
            '∩' => {
                output.push((Token::UpShoe, Loc { line, col }));
            }
            '⊣' => {
                output.push((Token::LeftTack, Loc { line, col }));
            }
            '⊢' => {
                output.push((Token::RightTack, Loc { line, col }));
            }
            '⍳' => {
                output.push((Token::Iota, Loc { line, col }));
            }
            '⍸' => {
                output.push((Token::IotaUnderbar, Loc { line, col }));
            }
            '⍷' => {
                output.push((Token::EpsilonUnderbar, Loc { line, col }));
            }
            '⍋' => {
                output.push((Token::GradeUp, Loc { line, col }));
            }
            '⍒' => {
                output.push((Token::GradeDown, Loc { line, col }));
            }
            '¨' => {
                output.push((Token::Diaeresis, Loc { line, col }));
            }
            '⍨' => {
                output.push((Token::TildeDiaeresis, Loc { line, col }));
            }
            '⍣' => {
                output.push((Token::StarDiaeresis, Loc { line, col }));
            }
            '.' => {
                output.push((Token::Dot, Loc { line, col }));
            }
            '∘' => {
                output.push((Token::Jot, Loc { line, col }));
            }
            '⌸' => {
                output.push((Token::QuadEqual, Loc { line, col }));
            }
            '⍤' => {
                output.push((Token::JotDiaeresis, Loc { line, col }));
            }
            '⍥' => {
                output.push((Token::CircleDieresis, Loc { line, col }));
            }
            '⌺' => {
                output.push((Token::QuadDiamond, Loc { line, col }));
            }
            '@' => {
                output.push((Token::At, Loc { line, col }));
            }
            '⍠' => {
                output.push((Token::QuadColon, Loc { line, col }));
            }
            '←' => {
                output.push((Token::LeftArrow, Loc { line, col }));
            }
            '⍬' => {
                output.push((Token::Zilde, Loc { line, col }));
            }
            '⍎' => {
                output.push((Token::Hydrant, Loc { line, col }));
            }
            '⍕' => {
                output.push((Token::Thorn, Loc { line, col }));
            }
            '⋄' => {
                output.push((Token::Diamond, Loc { line, col }));
            }
            '∇' => {
                output.push((Token::Del, Loc { line, col }));
            }
            '⍺' => {
                output.push((Token::Alpha, Loc { line, col }));
            }
            '⍵' => {
                output.push((Token::Omega, Loc { line, col }));
            }
            '{' => {
                output.push((Token::OpenCurlyBracket, Loc { line, col }));
            }
            '}' => {
                output.push((Token::CloseCurlyBracket, Loc { line, col }));
            }
            '(' => {
                output.push((Token::OpenRoundBracket, Loc { line, col }));
            }
            ')' => {
                output.push((Token::CloseRoundBracket, Loc { line, col }));
            }
            '[' => {
                output.push((Token::OpenSquareBracket, Loc { line, col }));
            }
            ']' => {
                output.push((Token::CloseSquareBracket, Loc { line, col }));
            }
            ':' => {
                output.push((Token::Colon, Loc { line, col }));
            }
        }
    }

    output
}

pub fn destream(stream: TokenStream) -> String {
    let mut out = String::with_capacity(stream.len());

    for (token, _) in stream {
        match token {
            Token::Identifier(s) => out.push_str(s.as_ref()),
            Token::StringLiteral(s) => out.push_str(s.as_ref()),
            Token::Comment(s) => out.push_str(s.as_ref()),
            Token::NumericLiteral(s) => {
                if let Some('0'..='9' | 'u' | 'f' | 'i' | 'E' | 'J' | '¯') = out.chars().last() {
                    out.push(' ');
                }
                out.push_str(s.to_string().as_ref());
            }

            Token::Plus => out.push('+'),
            Token::Minus => out.push('-'),
            Token::Times => out.push('×'),
            Token::Divide => out.push('÷'),
            Token::Upstile => out.push('⌈'),
            Token::Downstile => out.push('⌊'),
            Token::Star => out.push('*'),
            Token::ExclamationMark => out.push('!'),
            Token::Stile => out.push('|'),
            Token::Log => out.push('⍟'),
            Token::Circle => out.push('○'),
            Token::Domino => out.push('⌹'),
            Token::UpTack => out.push('⊥'),
            Token::DownTack => out.push('⊤'),
            Token::QuestionMark => out.push('?'),
            Token::Tilde => out.push('~'),
            Token::LogicalAND => out.push('∧'),
            Token::LogicalOR => out.push('∨'),
            Token::LogicalNAND => out.push('⍲'),
            Token::LogicalNOR => out.push('⍱'),
            Token::LessThan => out.push('<'),
            Token::GreaterThan => out.push('>'),
            Token::LessThanOrEqualTo => out.push('≤'),
            Token::GreaterThanOrEqualTo => out.push('≥'),
            Token::Equal => out.push('='),
            Token::NotEqual => out.push('≠'),
            Token::EqualUnderbar => out.push('≡'),
            Token::EqualUnderbarSlash => out.push('≢'),
            Token::Rho => out.push('⍴'),
            Token::Comma => out.push(','),
            Token::CommaBar => out.push('⍪'),
            Token::CircleStile => out.push('⌽'),
            Token::CircleBar => out.push('⊖'),
            Token::Transpose => out.push('⍉'),
            Token::UpArrow => out.push('↑'),
            Token::DownArrow => out.push('↓'),
            Token::LeftShoe => out.push('⊂'),
            Token::LeftShoeUnderbar => out.push('⊆'),
            Token::Epsilon => out.push('∊'),
            Token::Squad => out.push('⌷'),
            Token::RightShoe => out.push('⊃'),
            Token::Slash => out.push('/'),
            Token::SlashBar => out.push('⌿'),
            Token::Backslash => out.push('\\'),
            Token::BackslashBar => out.push('⍀'),
            Token::DownShoe => out.push('∪'),
            Token::UpShoe => out.push('∩'),
            Token::LeftTack => out.push('⊣'),
            Token::RightTack => out.push('⊢'),
            Token::Iota => out.push('⍳'),
            Token::IotaUnderbar => out.push('⍸'),
            Token::EpsilonUnderbar => out.push('⍷'),
            Token::GradeUp => out.push('⍋'),
            Token::GradeDown => out.push('⍒'),
            Token::Diaeresis => out.push('¨'),
            Token::TildeDiaeresis => out.push('⍨'),
            Token::StarDiaeresis => out.push('⍣'),
            Token::Dot => out.push('.'),
            Token::Jot => out.push('∘'),
            Token::QuadEqual => out.push('⌸'),
            Token::JotDiaeresis => out.push('⍤'),
            Token::CircleDieresis => out.push('⍥'),
            Token::QuadDiamond => out.push('⌺'),
            Token::At => out.push('@'),
            Token::QuadColon => out.push('⍠'),
            Token::LeftArrow => out.push('←'),
            Token::Zilde => out.push('⍬'),
            Token::Hydrant => out.push('⍎'),
            Token::Thorn => out.push('⍕'),
            Token::Diamond => out.push('⋄'),
            Token::Del => out.push('∇'),
            Token::Alpha => out.push('⍺'),
            Token::Omega => out.push('⍵'),
            Token::OpenCurlyBracket => out.push('{'),
            Token::CloseCurlyBracket => out.push('}'),
            Token::OpenRoundBracket => out.push('('),
            Token::CloseRoundBracket => out.push(')'),
            Token::OpenSquareBracket => out.push('['),
            Token::CloseSquareBracket => out.push(']'),
            Token::Colon => out.push(':'),

            _ => todo!(),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::{normalizer::normalize_apl_code, tokenizer::*};

    #[test]
    fn it_tokenizes() {
        println!(
            "{:?}",
            destream(tokenize(normalize_apl_code(
                "nDCube ← {v←⍵ ⋄ ⍺{⍺=1u4:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string(),
            ),))
        );

        // println!(
        //     "{:?}",
        //     tokenize(normalize_apl_code(",/ 'ab' 'cd' 'ef'".to_string(),), 0,)
        // );
    }
}
