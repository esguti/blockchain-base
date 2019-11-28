/// A trait for converting to bytes an object.
pub trait Byteable {
    /// Return the memory representation as a byte array in little-endian byte order.
    fn bytes (&self) -> Vec<u8>;
    // fn to_le_bytes (&self) -> [u8; usize];
}

impl Byteable for i32 {
    fn bytes(&self) -> Vec<u8> {
        let data = &self.to_le_bytes();
        data.to_vec()
    }
}

impl Byteable for String{
    fn bytes(&self) -> Vec<u8> {    
        let data = &self.as_bytes();
        data.to_vec()
    }
}

impl<T> Byteable for [T]
where
    T: Byteable,
{
    fn bytes(&self) -> Vec<u8> {
        let mut data : Vec<u8> = Vec::new();
        for x in self {
            data.extend(x.bytes().iter().cloned());
        }
        data
    }
}
