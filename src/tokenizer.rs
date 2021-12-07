pub(crate) mod bracket_partitioner;
mod numeric_literal;

use crate::errors::Errors;
use crate::tokenizer::numeric_literal::NumericLiteral;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Loc {
    line: usize,
    col: usize,
}

impl Display for Loc {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_fmt(format_args!("{}:{}", self.line, self.col))
    }
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

fn numeric_literal_extractor(
    stream: &mut Stream,
    output: &mut TokenStream,
    line: &mut usize,
    col: &mut usize,
) -> anyhow::Result<()> {
    let mut counting_cycle = String::new();
    while let Some('0'..='9' | 'u' | 'f' | 'i' | 'E' | 'J' | '¯') = stream.peek() {
        counting_cycle.push(stream.next().unwrap());
        *col += 1;
    }
    output.push((
        Token::NumericLiteral(counting_cycle.parse::<NumericLiteral>()?),
        Loc {
            col: *col - 1,
            line: *line,
        },
    ));

    Ok(())
}

fn string_literal_extractor(
    stream: &mut Stream,
    output: &mut TokenStream,
    line: &mut usize,
    col: &mut usize,
) -> anyhow::Result<()> {
    let mut str = String::new();
    while let (Some(token), next) = (stream.next(), stream.peek()) {
        let next = match next {
            Some(v) => *v,
            None => {
                anyhow::bail!(Errors::UnexpectedToken(
                    Token::EOF,
                    Loc {
                        col: *col,
                        line: *line,
                    },
                ))
            }
        };
        if token != '\\' && next == '\'' {
            stream.next();
            break;
        } else if token == '\n' {
            anyhow::bail!(Errors::UnexpectedToken(
                Token::NL,
                Loc {
                    col: *col,
                    line: *line,
                },
            ));
        } else if token == '\\' {
            match next {
                '\n' => {
                    str.push('\n');
                    *col = 0;
                    *line += 1;
                    stream.next();
                }
                'n' => str.push('\n'),
                '\'' => str.push('\''),

                _ => anyhow::bail!("Unexpected escape character {}", next),
            }
        } else if next != '\\' {
            str.push(next);
        }
        *col += 1;
    }
    output.push((
        Token::StringLiteral(str),
        Loc {
            col: *col,
            line: *line,
        },
    ));
    Ok(())
}

fn identifier_extractor(
    stream: &mut Stream,
    output: &mut TokenStream,
    line: &mut usize,
    col: &mut usize,
) {
    let mut str = String::new();

    while let Some(next) = stream.peek() {
        let next = *next;

        match next {
            '+' | '-' | '×' | '÷' | '⌈' | '⌊' | '*' | '!' | '|' | '⍟' | '○' | '⌹' | '⊥' | '⊤'
            | '?' | '~' | '∧' | '∨' | '⍲' | '⍱' | '<' | '>' | '≤' | '≥' | '=' | '≠' | '≡' | '≢'
            | '⍴' | ',' | '⍪' | '⌽' | '⊖' | '⍉' | '↑' | '↓' | '⊂' | '⊆' | '∊' | '⌷' | '⊃' | '/'
            | '⌿' | '\\' | '⍀' | '∪' | '∩' | '⊣' | '⊢' | '⍳' | '⍸' | '⍷' | '⍋' | '⍒' | '¨'
            | '⍨' | '⍣' | '.' | '∘' | '⌸' | '⍤' | '⍥' | '⌺' | '@' | '⍠' | '←' | '⍬' | '⍎' | '⍕'
            | '⋄' | '∇' | '⍺' | '⍵' | '{' | '}' | '(' | ')' | '[' | ']' | ':' | ' ' | '\t' => {
                break
            }
            _ => {
                str.push(stream.next().unwrap());
                *col += 1;
            }
        }
    }
    output.push((
        Token::Identifier(str),
        Loc {
            line: *line,
            col: *col - 1,
        },
    ));
}

pub fn tokenize(str: String) -> anyhow::Result<TokenStream> {
    let mut output: TokenStream = Vec::with_capacity(str.len() / 2);

    let mut line: usize = 1;
    let mut col: usize = 1;

    let mut stream: Stream = str.chars().peekable();

    while let Some(token) = stream.peek() {
        let token = *token;

        match token {
            '0'..='9' => {
                numeric_literal_extractor(&mut stream, &mut output, &mut line, &mut col)?;
            }
            '\'' => {
                string_literal_extractor(&mut stream, &mut output, &mut line, &mut col)?;
                // in_string_literal = true;
            }
            '\n' => {
                output.push((Token::NL, Loc { line, col }));

                line += 1;
                col = 1;
                stream.next();
            }
            '+' => {
                output.push((Token::Plus, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '-' => {
                output.push((Token::Minus, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '×' => {
                output.push((Token::Times, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '÷' => {
                output.push((Token::Divide, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌈' => {
                output.push((Token::Upstile, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌊' => {
                output.push((Token::Downstile, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '*' => {
                output.push((Token::Star, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '!' => {
                output.push((Token::ExclamationMark, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '|' => {
                output.push((Token::Stile, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍟' => {
                output.push((Token::Log, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '○' => {
                output.push((Token::Circle, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌹' => {
                output.push((Token::Domino, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊥' => {
                output.push((Token::UpTack, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊤' => {
                output.push((Token::DownTack, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '?' => {
                output.push((Token::QuestionMark, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '~' => {
                output.push((Token::Tilde, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∧' => {
                output.push((Token::LogicalAND, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∨' => {
                output.push((Token::LogicalOR, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍲' => {
                output.push((Token::LogicalNAND, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍱' => {
                output.push((Token::LogicalNOR, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '<' => {
                output.push((Token::LessThan, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '>' => {
                output.push((Token::GreaterThan, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '≤' => {
                output.push((Token::LessThanOrEqualTo, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '≥' => {
                output.push((Token::GreaterThanOrEqualTo, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '=' => {
                output.push((Token::Equal, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '≠' => {
                output.push((Token::NotEqual, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '≡' => {
                output.push((Token::EqualUnderbar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '≢' => {
                output.push((Token::EqualUnderbarSlash, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍴' => {
                output.push((Token::Rho, Loc { line, col }));
                col += 1;
                stream.next();
            }
            ',' => {
                output.push((Token::Comma, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍪' => {
                output.push((Token::CommaBar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌽' => {
                output.push((Token::CircleStile, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊖' => {
                output.push((Token::CircleBar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍉' => {
                output.push((Token::Transpose, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '↑' => {
                output.push((Token::UpArrow, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '↓' => {
                output.push((Token::DownArrow, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊂' => {
                output.push((Token::LeftShoe, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊆' => {
                output.push((Token::LeftShoeUnderbar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∊' => {
                output.push((Token::Epsilon, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌷' => {
                output.push((Token::Squad, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊃' => {
                output.push((Token::RightShoe, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '/' => {
                output.push((Token::Slash, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌿' => {
                output.push((Token::SlashBar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '\\' => {
                output.push((Token::Backslash, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍀' => {
                output.push((Token::BackslashBar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∪' => {
                output.push((Token::DownShoe, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∩' => {
                output.push((Token::UpShoe, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊣' => {
                output.push((Token::LeftTack, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⊢' => {
                output.push((Token::RightTack, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍳' => {
                output.push((Token::Iota, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍸' => {
                output.push((Token::IotaUnderbar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍷' => {
                output.push((Token::EpsilonUnderbar, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍋' => {
                output.push((Token::GradeUp, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍒' => {
                output.push((Token::GradeDown, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '¨' => {
                output.push((Token::Diaeresis, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍨' => {
                output.push((Token::TildeDiaeresis, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍣' => {
                output.push((Token::StarDiaeresis, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '.' => {
                output.push((Token::Dot, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∘' => {
                output.push((Token::Jot, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌸' => {
                output.push((Token::QuadEqual, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍤' => {
                output.push((Token::JotDiaeresis, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍥' => {
                output.push((Token::CircleDieresis, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⌺' => {
                output.push((Token::QuadDiamond, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '@' => {
                output.push((Token::At, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍠' => {
                output.push((Token::QuadColon, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '←' => {
                output.push((Token::LeftArrow, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍬' => {
                output.push((Token::Zilde, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍎' => {
                output.push((Token::Hydrant, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍕' => {
                output.push((Token::Thorn, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⋄' => {
                output.push((Token::Diamond, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '∇' => {
                output.push((Token::Del, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍺' => {
                output.push((Token::Alpha, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '⍵' => {
                output.push((Token::Omega, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '{' => {
                output.push((Token::OpenCurlyBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '}' => {
                output.push((Token::CloseCurlyBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '(' => {
                output.push((Token::OpenRoundBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            ')' => {
                output.push((Token::CloseRoundBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            '[' => {
                output.push((Token::OpenSquareBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            ']' => {
                output.push((Token::CloseSquareBracket, Loc { line, col }));
                col += 1;
                stream.next();
            }
            ':' => {
                output.push((Token::Colon, Loc { line, col }));
                col += 1;
                stream.next();
            }
            ' ' | '\t' => {
                col += 1;
                stream.next();
            }
            _ => {
                identifier_extractor(&mut stream, &mut output, &mut line, &mut col);
            }
        }
        // match *token {}
    }

    Ok(output)
}

#[allow(dead_code)]
pub fn destream(stream: TokenStream) -> String {
    let mut out = String::with_capacity(stream.len());

    for (token, _) in stream {
        match token {
            Token::Identifier(s) => out.push_str(s.as_ref()),
            Token::StringLiteral(s) => {
                if let Some('0'..='9' | 'u' | 'f' | 'i' | 'E' | 'J' | '¯' | '\'') =
                    out.chars().last()
                {
                    out.push(' ');
                }
                out.push_str(format!("'{}'", s.replace("\n", "\\n")).as_ref());
            }
            Token::Comment(s) => out.push_str(s.as_ref()),
            Token::NumericLiteral(s) => {
                if let Some('0'..='9' | 'u' | 'f' | 'i' | 'E' | 'J' | '¯' | '\'') =
                    out.chars().last()
                {
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
#[allow(dead_code)]
pub fn destream_loc_indicators(stream: TokenStream) -> String {
    let mut out = vec![String::new()];

    for (token, Loc { line, col }) in stream {
        if token == Token::NL {
            continue;
        }

        let lookup_line = line - 1;
        let lookup_col = col - 1;
        let value = match out.get_mut(lookup_line) {
            Some(arr) => arr,
            None => {
                while out.len() < lookup_line {
                    out.push(String::new());
                }
                out.get_mut(lookup_line).unwrap()
            }
        };

        if value.len() < lookup_col {
            let extension = " ".to_string().repeat(lookup_col - value.len());
            (*value).push_str(extension.as_str());
        }
        value.insert_str(
            lookup_col,
            match token {
                Token::StringLiteral(_) => "s",
                Token::NumericLiteral(_) => "n",
                Token::Identifier(_) => "i",
                Token::OpenCurlyBracket => "{",
                Token::CloseCurlyBracket => "}",
                Token::OpenSquareBracket => "[",
                Token::CloseSquareBracket => "]",
                Token::OpenRoundBracket => "(",
                Token::CloseRoundBracket => ")",
                _ => "^",
            },
        )
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::{normalizer::normalize_apl_code, tokenizer::*};

    #[test]
    fn it_tokenizes() {
        println!(
            "{:?}",
            destream(
                tokenize(normalize_apl_code(
                    "nDCube ← {v←⍵ ⋄ ⍺{⍺=1u4:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string(),
                ))
                .unwrap()
            )
        );

        // println!(
        //     "{:?}",
        //     tokenize(normalize_apl_code(",/ 'ab' 'cd' 'ef'".to_string(),), 0,)
        // );
    }

    fn print_locs(src: String) {
        println!("{}", src);

        let stream = tokenize(src).unwrap();

        println!("{}", destream_loc_indicators(stream.clone()));
        println!("{:?}", stream);
    }

    #[test]
    fn it_places_locs_correctly() {
        let src = "nDCube ← {v←⍵ ⋄ ⍺{⍺=1u4:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string();
        print_locs(src);

        let src = "π ← ○ 1".to_string();
        print_locs(src);
    }

    #[test]
    fn it_tokenizes_literal() {
        println!(
            "{:?}",
            destream(
                tokenize(normalize_apl_code(
                    "('Aleth' 'xheta') (1u8 2f 1b)".to_string(),
                ))
                .unwrap()
            )
        );
    }
}
