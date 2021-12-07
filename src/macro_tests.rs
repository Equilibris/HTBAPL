#[cfg(test)]
mod tests {
    use macros::repeat_code;

    #[test]
    fn hello_world() {
        
        repeat_code!{
            let s = "$a";
            println!("{} is $b",s);
        }
        
    }
}
