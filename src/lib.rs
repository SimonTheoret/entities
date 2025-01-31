mod order;
mod product;
mod user;

/// This module contains the trait associated with CRUD: `Copy`, `Retrieve`, `Update` and `Delete`.
/// It also has a `Consume` trait. To fully used this module, it is better to implement the NewType
/// pattern for most strucures.
mod crud {

    pub struct Context<T>(T);

    impl<T> Context<T> {
        pub fn ref_inner(&self) -> &T {
            &self.0
        }
        pub fn mut_ref_inner(&mut self) -> &mut T {
            &mut self.0
        }
        pub fn into_inner(self) -> T {
            self.0
        }
    }

    pub trait Delete<T> {
        /// Delete data from self
        fn delete<'a>(&mut self, ctx: Context<T>);
    }

    pub trait Create<Can, New> {
        /// Verifies that the conditions a
        /// re met to create a `Self` object
        #[allow(unused_variables)]
        async fn can_be_created<'a>(ctx: Context<Can>) -> bool {
            false
        }
        /// Create a `Self` object
        async fn new<'a>(ctx: Context<New>) -> Self;
    }

    pub trait Retrieve<'s, T, K>
    where
        T: 's,
    {
        /// Retrieves a type T from self
        async fn retrieve<'a>(&'s self, ctx: Context<K>) -> &'s T;
        /// Verifies if `self` contains some data
        #[allow(unused_variables)]
        async fn contains<'a>(&self, ctx: Context<K>) -> bool {
            false
        }
    }
    pub trait Consume<T, K> {
        /// Consumes self in order to retrieve some data
        async fn consume<'a>(self, ctx: Context<K>) -> T;
    }

    pub trait Update<K, V> {
        /// Update some value inside the structure and return the old value
        async fn update<'a>(&mut self, ctx: Context<(K, V)>) -> V;
    }
}
