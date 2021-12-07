use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn repeat_code(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let mut output = TokenStream::new();

    for (id, str) in [
        ("+", "Plus"),
        ("-", "Minus"),
        ("×", "Times"),
        ("÷", "Divide"),
        ("⌈", "Upstile"),
        ("⌊", "Downstile"),
        ("*", "Star"),
        ("!", "ExclamationMark"),
        ("|", "Stile"),
        ("⍟", "Log"),
        ("○", "Circle"),
        ("⌹", "Domino"),
        ("⊥", "UpTack"),
        ("⊤", "DownTack"),
        ("?", "QuestionMark"),
        ("~", "Tilde"),
        ("∧", "LogicalAND"),
        ("∨", "LogicalOR"),
        ("⍲", "LogicalNAND"),
        ("⍱", "LogicalNOR"),
        ("<", "LessThan"),
        (">", "GreaterThan"),
        ("≤", "LessThanOrEqualTo"),
        ("≥", "GreaterThanOrEqualTo"),
        ("=", "Equal"),
        ("≠", "NotEqual"),
        ("≡", "EqualUnderbar"),
        ("≢", "EqualUnderbarSlash"),
        ("⍴", "Rho"),
        (",", "Comma"),
        ("⍪", "CommaBar"),
        ("⌽", "CircleStile"),
        ("⊖", "CircleBar"),
        ("⍉", "Transpose"),
        ("↑", "UpArrow"),
        ("↓", "DownArrow"),
        ("⊂", "LeftShoe"),
        ("⊆", "LeftShoeUnderbar"),
        ("∊", "Epsilon"),
        ("⌷", "Squad"),
        ("⊃", "RightShoe"),
        ("/", "Slash"),
        ("⌿", "SlashBar"),
        ("\\\\", "Backslash"),
        ("⍀", "BackslashBar"),
        ("∪", "DownShoe"),
        ("∩", "UpShoe"),
        ("⊣", "LeftTack"),
        ("⊢", "RightTack"),
        ("⍳", "Iota"),
        ("⍸", "IotaUnderbar"),
        ("⍷", "EpsilonUnderbar"),
        ("⍋", "GradeUp"),
        ("⍒", "GradeDown"),
        ("¨", "Diaeresis"),
        ("⍨", "TildeDiaeresis"),
        ("⍣", "StarDiaeresis"),
        (".", "Dot"),
        ("∘", "Jot"),
        ("⌸", "QuadEqual"),
        ("⍤", "JotDiaeresis"),
        ("⍥", "CircleDieresis"),
        ("⌺", "QuadDiamond"),
        ("@", "At"),
        ("⍠", "QuadColon"),
        ("←", "LeftArrow"),
        ("⍬", "Zilde"),
        ("⍎", "Hydrant"),
        ("⍕", "Thorn"),
        ("⋄", "Diamond"),
        ("∇", "Del"),
        ("⍺", "Alpha"),
        ("⍵", "Omega"),
        (":", "Colon"),
        ("(", "OpenRoundBracket"),
        ("[", "OpenSquareBracket"),
        ("{", "OpenCurlyBracket"),
        (")", "CloseRoundBracket"),
        ("]", "CloseSquareBracket"),
        ("}", "CloseCurlyBracket"),
    ] {
        output.extend(TokenStream::from_str(
            input.clone().replace("$a", id).replace("$b", str).as_str(),
        ))
    }

    output
    // TODO, make it good
    // let mut intermediate_stream = TokenStream::new();

    // let mut peek_in = input.into_iter().peekable();

    // let mut a_insert_index = Vec::new();
    // let mut b_insert_index = Vec::new();

    // while let Some(curr) = peek_in.next() {
    //     let next = peek_in.peek();

    //     match (curr, next) {
    //         (a, None) => intermediate_stream.extend(TokenTree::),
    //         (proc_macro::TokenTree::Group(_), Some(_)) => todo!(),
    //         (proc_macro::TokenTree::Ident(_), Some(_)) => todo!(),
    //         (proc_macro::TokenTree::Punct(_), Some(_)) => todo!(),
    //         (proc_macro::TokenTree::Literal(_), Some(_)) => todo!(),
    //     }
    // }

    // println!("values: {:?}", input);
    // println!("hello_world: {:?}", input.to_string());
    // TokenStream::from_str("println!(\"Hello world\")").unwrap()

    // // Parse the input tokens into a syntax tree
    // let input = parse_macro_input!(input as DeriveInput);

    // // Build the output, possibly using quasi-quotation
    // let expanded = quote! {
    //     // ...
    // };

    // // Hand the output tokens back to the compiler
    // TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use crate::repeat_code;

    #[test]
    fn it_works() {
        repeat_code!(hello)
    }
}
