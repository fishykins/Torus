use corale::core::OrdNum;

pub struct Arc<T> where T: OrdNum {
    pub a: T,
    pub b: T,
    pub r: T,
}