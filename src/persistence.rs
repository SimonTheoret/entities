/// This trait is to be implemented on the Database/driver.
pub trait DBConds<Target> {
    /// We can verify if `data` exists in the persistence layer.
    fn exists(&self, data: Target) -> bool;
    /// We can verify if `data` is unique in the persistence layer.
    fn is_unique(&self, data: Target) -> bool;
}
