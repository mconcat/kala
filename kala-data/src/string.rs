#[derive(Clone, Debug, PartialEq)]
pub struct JSString(String);

impl JSString {
    fn concat(&mut self, other: &Self) -> &mut Self {
        self.0.push_str(&other.0);
        self
    }
}
