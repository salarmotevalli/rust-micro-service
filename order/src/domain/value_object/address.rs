use super::ValueObject;

#[derive(Default)]
pub struct Address {
    value: String,
}

impl ValueObject<String> for Address {
    fn value(&self) -> String {
        self.value.clone()
    }
}
