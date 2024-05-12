use super::ValueObject;

#[derive(Default)]
pub struct Price {
    value: u32,
}

impl ValueObject<u32> for Price {
    fn value(&self) -> u32 {
        self.value.clone()
    }
}
