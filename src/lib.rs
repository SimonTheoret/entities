mod order;
mod persistence;
mod product;
mod user;
use derive_more::{Deref, DerefMut, Display, Into};
use std::error::Error;
use uuid::Uuid;

pub use crate::user::User;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Deref, DerefMut, Into)]
pub struct Email(String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Deref, DerefMut, Into)]
pub struct Name(String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Deref, DerefMut, Into, Copy,
)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}

/// Trait for types that can be created with a fixed size
pub trait Create<Ctx>: Sized {
    fn create(ctx: Ctx) -> Self;
}
/// Async version of the `Create` trait
pub trait AsyncCreate<Ctx>: Create<Ctx> {
    async fn async_create(ctx: Ctx) -> Self {
        <Self as Create<Ctx>>::create(ctx)
    }
}

/// Trait for verifying the type can be created
pub trait CanBeCreated<Ctx> {
    fn can_be_created(ctx: Ctx) -> bool;
}

/// Async version of the `CanBeCreated` trait
pub trait AsyncCanBeCreated<Ctx>: CanBeCreated<Ctx> {
    async fn async_can_be_created(ctx: Ctx) -> bool {
        <Self as CanBeCreated<Ctx>>::can_be_created(ctx)
    }
}

/// This trait is used to instantiate types given some conditions are true. The types implementing
/// this trait must have a method to specify if they can be instantiated given the conditions.
pub trait SafeCreate<Ctx, Conds>: Create<Ctx> + CanBeCreated<Conds> {
    fn safe_create(ctx: (Ctx, Conds)) -> Option<Self> {
        let (create, can) = ctx;
        if Self::can_be_created(can) {
            Some(Self::create(create))
        } else {
            None
        }
    }
}

/// Async version of the `SafeCreate` trait.
pub trait AsyncSafeCreate<Ctx, Conds>: AsyncCreate<Ctx> + AsyncCanBeCreated<Conds> {
    async fn safe_create(ctx: (Ctx, Conds)) -> Option<Self> {
        let (create, can) = ctx;
        if Self::async_can_be_created(can).await {
            Some(Self::async_create(create).await)
        } else {
            None
        }
    }
}

/// Trait for types that can be deleted with a fixed size
pub trait Delete<Ctx>: Sized {
    fn delete(self, ctx: Ctx) {
        std::mem::drop(self)
    }
}
pub trait AsyncDelete<Ctx>: Delete<Ctx> {
    async fn async_delete(self, ctx: Ctx) {
        self.delete(ctx)
    }
}

/// Trait for verifying the type can be deleted
pub trait CanBeDeleted<Ctx> {
    fn can_be_deleted(&self, ctx: Ctx) -> bool;
}

/// This trait is used to delete types given some that conditions are true. The types implementing
/// this trait must have a method to specify if they can be instantiated given the conditions.
pub trait SafeDelete<Ctx, Conds>: Delete<Ctx> + CanBeDeleted<Conds> + Sized {
    type Err: Error;
    fn safe_delete(self, ctx: (Ctx, Conds)) -> Option<Self> {
        let (del_ctx, conditions) = ctx;
        if self.can_be_deleted(conditions) {
            self.delete(del_ctx);
            None
        } else {
            Some(self)
        }
    }
}

/// Trait for types that can be updated
pub trait Update<Ctx> {
    fn update(&mut self, ctx: Ctx);
}

/// Trait for verifying the type can be updated
pub trait CanBeUpdated<Ctx> {
    fn can_be_updated(&self, ctx: Ctx) -> bool;
}
pub trait SafeUpdate<Ctx, Conds>: Update<Ctx> + CanBeUpdated<Conds> {
    type Err: Error;
    fn safe_update(&mut self, ctx: (Ctx, Conds)) -> Result<(), Self::Err>;
}

mod dyn_map {
    use derive_more::{Deref, DerefMut, Display, From, Into};
    use std::any::Any;
    use std::collections::HashMap;

    #[derive(Debug, Display, Deref, DerefMut, Into, From, Default)]
    /// Dynamic structure capable of holding any kind of data.
    pub struct DynMap<K>(HashMap<K, Box<dyn Any>>);

    /// Trait for accessing the `DynMap`.
    pub trait Extended<K> {
        fn extended_content(&self) -> &DynMap<K>;
        fn extended_content_mut(&mut self) -> &mut DynMap<K>;
    }
}
