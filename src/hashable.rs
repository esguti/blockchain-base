/// A trait for hashing an object.
pub trait Hashable {
    /// Calculate the hash of an object and store the result
    /// in their internal structures
    fn calculate_hash (&mut self);
}
