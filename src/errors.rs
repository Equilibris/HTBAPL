#[derive(Debug, PartialEq)]
pub struct BaseErr<'a> {
    reason: &'a str,
    // TODO: explanation
}

impl<'a> BaseErr<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { reason }
    }
}
