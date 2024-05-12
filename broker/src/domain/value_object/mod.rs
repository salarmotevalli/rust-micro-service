pub(super) mod address;
pub(super) mod id;

pub(super) trait ValueObject<T> {
    fn value(&self) -> T;
}
