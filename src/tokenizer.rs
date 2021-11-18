#[derive(Debug)]
pub struct Loc {
    line: usize,
    col: usize,
}

impl Loc {
    pub fn new(line: usize, col: usize) -> Loc {
        Loc { col, line }
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    NumericLiteral(String),
    StringLiteral(String),

    Comment(String), // ⍝

    OpenRoundBracket,
    OpenSquareBracket,
    OpenCurlyBracket,
    CloseRoundBracket,
    CloseSquareBracket,
    CloseCurlyBracket,

    NL, // \n
    Spacer,

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

    HighMinus, // ¯
    LeftArrow, // ←
    Zilde,     // ⍬
    Hydrant,   // ⍎
    Thorn,     // ⍕
    Diamond,   // ⋄
    Del,       // ∇
    Alpha,     // ⍺
    Omega,     // ⍵

    Colon, // :
}

pub type TokenStream = Vec<(Token, Loc)>;

pub fn tokenize(str: String) -> TokenStream {
    let mut out = Vec::with_capacity(str.len() / 2);

    let mut in_identifier = false;
    let mut in_numeric_literal = false;
    let mut in_string_literal = false;

    let mut str_search_value = String::new();

    let mut line: usize = 0;
    let mut col: usize = 0;

    let mut last_char: char = '\0';

    for char in str.chars() {
        if in_numeric_literal {
        } else if in_string_literal {
            if last_char == '\\' {
                if char != '\\' {
                    str_search_value.push('\\');
                } else if char == 'n' {
                    str_search_value.push('\'');
                } else if char == '\'' {
                    str_search_value.push('\n');
                } else if char == 'r' {
                    str_search_value.push('\r');
                } else {
                    panic!("un implemented escape character \\{}", char);
                }
            } else if char == '\'' {
                out.push((
                    Token::StringLiteral(str_search_value.clone()),
                    Loc::new(line, col),
                ));
                str_search_value.clear();
                in_string_literal = false;
            } else if char != '\\' {
                str_search_value.push(char);
            }
        } else if ('a' < char && char < 'z')
            || ('A' < char && char < 'Z')
            || (in_identifier && ('0' < char && char < '9'))
        {
            str_search_value.push(char);
            in_identifier = true;
        } else {
            if char == '\n' {
                out.push((Token::NL, Loc::new(line, col)));

                line += 1;
            } else if char == '\'' {
                in_string_literal = true;
            } else if char == '+' {
                out.push((Token::Plus, Loc::new(line, col)));
            } else if char == '-' {
                out.push((Token::Minus, Loc::new(line, col)));
            } else if char == '×' {
                out.push((Token::Times, Loc::new(line, col)));
            } else if char == '÷' {
                out.push((Token::Divide, Loc::new(line, col)));
            } else if char == '⌈' {
                out.push((Token::Upstile, Loc::new(line, col)));
            } else if char == '⌊' {
                out.push((Token::Downstile, Loc::new(line, col)));
            } else if char == '*' {
                out.push((Token::Star, Loc::new(line, col)));
            } else if char == '!' {
                out.push((Token::ExclamationMark, Loc::new(line, col)));
            } else if char == '|' {
                out.push((Token::Stile, Loc::new(line, col)));
            } else if char == '⍟' {
                out.push((Token::Log, Loc::new(line, col)));
            } else if char == '○' {
                out.push((Token::Circle, Loc::new(line, col)));
            } else if char == '⌹' {
                out.push((Token::Domino, Loc::new(line, col)));
            } else if char == '⊥' {
                out.push((Token::UpTack, Loc::new(line, col)));
            } else if char == '⊤' {
                out.push((Token::DownTack, Loc::new(line, col)));
            } else if char == '?' {
                out.push((Token::QuestionMark, Loc::new(line, col)));
            } else if char == '~' {
                out.push((Token::Tilde, Loc::new(line, col)));
            } else if char == '∧' {
                out.push((Token::LogicalAND, Loc::new(line, col)));
            } else if char == '∨' {
                out.push((Token::LogicalOR, Loc::new(line, col)));
            } else if char == '⍲' {
                out.push((Token::LogicalNAND, Loc::new(line, col)));
            } else if char == '⍱' {
                out.push((Token::LogicalNOR, Loc::new(line, col)));
            } else if char == '<' {
                out.push((Token::LessThan, Loc::new(line, col)));
            } else if char == '>' {
                out.push((Token::GreaterThan, Loc::new(line, col)));
            } else if char == '≤' {
                out.push((Token::LessThanOrEqualTo, Loc::new(line, col)));
            } else if char == '≥' {
                out.push((Token::GreaterThanOrEqualTo, Loc::new(line, col)));
            } else if char == '=' {
                out.push((Token::Equal, Loc::new(line, col)));
            } else if char == '≠' {
                out.push((Token::NotEqual, Loc::new(line, col)));
            } else if char == '≡' {
                out.push((Token::EqualUnderbar, Loc::new(line, col)));
            } else if char == '≢' {
                out.push((Token::EqualUnderbarSlash, Loc::new(line, col)));
            } else if char == '⍴' {
                out.push((Token::Rho, Loc::new(line, col)));
            } else if char == ',' {
                out.push((Token::Comma, Loc::new(line, col)));
            } else if char == '⍪' {
                out.push((Token::CommaBar, Loc::new(line, col)));
            } else if char == '⌽' {
                out.push((Token::CircleStile, Loc::new(line, col)));
            } else if char == '⊖' {
                out.push((Token::CircleBar, Loc::new(line, col)));
            } else if char == '⍉' {
                out.push((Token::Transpose, Loc::new(line, col)));
            } else if char == '↑' {
                out.push((Token::UpArrow, Loc::new(line, col)));
            } else if char == '↓' {
                out.push((Token::DownArrow, Loc::new(line, col)));
            } else if char == '⊂' {
                out.push((Token::LeftShoe, Loc::new(line, col)));
            } else if char == '⊆' {
                out.push((Token::LeftShoeUnderbar, Loc::new(line, col)));
            } else if char == '∊' {
                out.push((Token::Epsilon, Loc::new(line, col)));
            } else if char == '⌷' {
                out.push((Token::Squad, Loc::new(line, col)));
            } else if char == '⊃' {
                out.push((Token::RightShoe, Loc::new(line, col)));
            } else if char == '/' {
                out.push((Token::Slash, Loc::new(line, col)));
            } else if char == '⌿' {
                out.push((Token::SlashBar, Loc::new(line, col)));
            } else if char == '\\' {
                out.push((Token::Backslash, Loc::new(line, col)));
            } else if char == '⍀' {
                out.push((Token::BackslashBar, Loc::new(line, col)));
            } else if char == '∪' {
                out.push((Token::DownShoe, Loc::new(line, col)));
            } else if char == '∩' {
                out.push((Token::UpShoe, Loc::new(line, col)));
            } else if char == '⊣' {
                out.push((Token::LeftTack, Loc::new(line, col)));
            } else if char == '⊢' {
                out.push((Token::RightTack, Loc::new(line, col)));
            } else if char == '⍳' {
                out.push((Token::Iota, Loc::new(line, col)));
            } else if char == '⍸' {
                out.push((Token::IotaUnderbar, Loc::new(line, col)));
            } else if char == '⍷' {
                out.push((Token::EpsilonUnderbar, Loc::new(line, col)));
            } else if char == '⍋' {
                out.push((Token::GradeUp, Loc::new(line, col)));
            } else if char == '⍒' {
                out.push((Token::GradeDown, Loc::new(line, col)));
            } else if char == '¨' {
                out.push((Token::Diaeresis, Loc::new(line, col)));
            } else if char == '⍨' {
                out.push((Token::TildeDiaeresis, Loc::new(line, col)));
            } else if char == '⍣' {
                out.push((Token::StarDiaeresis, Loc::new(line, col)));
            } else if char == '.' {
                out.push((Token::Dot, Loc::new(line, col)));
            } else if char == '∘' {
                out.push((Token::Jot, Loc::new(line, col)));
            } else if char == '⌸' {
                out.push((Token::QuadEqual, Loc::new(line, col)));
            } else if char == '⍤' {
                out.push((Token::JotDiaeresis, Loc::new(line, col)));
            } else if char == '⍥' {
                out.push((Token::CircleDieresis, Loc::new(line, col)));
            } else if char == '⌺' {
                out.push((Token::QuadDiamond, Loc::new(line, col)));
            } else if char == '@' {
                out.push((Token::At, Loc::new(line, col)));
            } else if char == '⍠' {
                out.push((Token::QuadColon, Loc::new(line, col)));
            } else if char == '¯' {
                out.push((Token::HighMinus, Loc::new(line, col)));
            } else if char == '←' {
                out.push((Token::LeftArrow, Loc::new(line, col)));
            } else if char == '⍬' {
                out.push((Token::Zilde, Loc::new(line, col)));
            } else if char == '⍎' {
                out.push((Token::Hydrant, Loc::new(line, col)));
            } else if char == '⍕' {
                out.push((Token::Thorn, Loc::new(line, col)));
            } else if char == '⋄' {
                out.push((Token::Diamond, Loc::new(line, col)));
            } else if char == '∇' {
                out.push((Token::Del, Loc::new(line, col)));
            } else if char == '⍺' {
                out.push((Token::Alpha, Loc::new(line, col)));
            } else if char == '⍵' {
                out.push((Token::Omega, Loc::new(line, col)));
            } else if char == '{' {
                out.push((Token::OpenCurlyBracket, Loc::new(line, col)));
            } else if char == '}' {
                out.push((Token::CloseCurlyBracket, Loc::new(line, col)));
            } else if char == '(' {
                out.push((Token::OpenRoundBracket, Loc::new(line, col)));
            } else if char == ')' {
                out.push((Token::CloseRoundBracket, Loc::new(line, col)));
            } else if char == '[' {
                out.push((Token::OpenSquareBracket, Loc::new(line, col)));
            } else if char == ']' {
                out.push((Token::CloseSquareBracket, Loc::new(line, col)));
            } else if char == ':' {
                out.push((Token::Colon, Loc::new(line, col)));
            // } else if char == 32 as char {
            } else {
                println!("undefined char {} {}", char, char as u8);
            }
            if in_identifier {
                in_identifier = false;
                out.push((
                    Token::Identifier(str_search_value.clone()),
                    Loc::new(line, col),
                ));
                str_search_value.clear();
            }
        }
        if char != '\n' {
            col += 1;
        }
        last_char = char;
    }

    out
}

pub fn destream(stream: TokenStream) -> String {
    let mut out = String::with_capacity(stream.len());

    for (token, _) in stream {
        match token {
            Token::Identifier(s) => out.push_str(s.as_ref()),
            Token::NumericLiteral(s) => out.push_str(s.as_ref()),
            Token::StringLiteral(s) => out.push_str(s.as_ref()),
            Token::Comment(s) => out.push_str(s.as_ref()),

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
            Token::HighMinus => out.push('¯'),
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
                "nDCube ← {v←⍵ ⋄ ⍺{⍺=1:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string(),
            ),))
        );

        // println!(
        //     "{:?}",
        //     tokenize(normalize_apl_code(",/ 'ab' 'cd' 'ef'".to_string(),), 0,)
        // );
    }
}
