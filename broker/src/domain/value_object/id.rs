use super::ValueObject;

#[derive(Default)]
pub struct Id {
    value: u32,
}

impl ValueObject<u32> for Id {
    fn value(&self) -> u32 {
        self.value
    }
}
