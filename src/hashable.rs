/// A trait for hashing an object.
pub trait Hashable {
    fn calculate_hash (&mut self);
}
