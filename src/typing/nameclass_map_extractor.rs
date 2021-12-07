pub(crate) fn construct_nameclass_map(
    token_stream: &crate::tokenizer::bracket_partitioner::PartitionStream,
) {
    for i in token_stream {}
}

#[cfg(test)]
mod tests {
    use crate::normalize_apl_code;
    use crate::tokenize;
    use crate::tokenize_to_partition;

    #[test]
    fn experiment_simple() {
        let stream = tokenize(normalize_apl_code(
            "nDCube ← {v←⍵ ⋄ ⍺{⍺=1:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}\nnDCube ← {v←⍵ ⋄ ⍺{⍺=1:v/⍵ ⋄ v/[⍺-1] (⍺-1) ∇ ⍵} (⍺/⍵) ⍴ ⍳⍵*⍺}".to_string(),
        )).unwrap();

        println!("{:?}", tokenize_to_partition(stream).unwrap());
    }
}
