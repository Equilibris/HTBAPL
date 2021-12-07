use crate::normalizer::normalize_apl_code;
use crate::tokenizer::bracket_partitioner::tokenize_to_partition;
use crate::tokenizer::tokenize;

mod errors;
mod ext;
mod macro_tests;
mod normalizer;
mod tokenizer;
mod typing;

fn main() {
    let program_to_compile = "nDCube ← {v←⍵ ⋄ ⍺{⍺=1u4:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}";

    let stream = tokenize(normalize_apl_code(program_to_compile.to_string())).unwrap();

    println!("{:?}", tokenize_to_partition(stream).unwrap());
}
