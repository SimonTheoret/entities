use crate::crud::{Consume, Context, Create, Delete, Retrieve, Update};
use crate::user::user_registry::UserRegistry;
use derive_more::{Deref, DerefMut, Display, Into};

mod user_registry;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Deref, DerefMut, Into)]
struct Email(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Deref, DerefMut, Into)]
struct Name(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display)]
/// What is the state of the Account.
enum AccountState {
    Active,
    Dormant,
    Deleted,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display)]
#[display("Name: {name}, Email: {email}, Account state: {account_state}")]
struct User {
    name: Name,
    email: Email,
    account_state: AccountState,
}

impl<New> Create<UserRegistry, New> for User
where
    New: Into<(Name, Email, AccountState)>,
{
    async fn can_be_created<'a>(ctx: Context<UserRegistry>) -> bool {
        todo!()
    }
    async fn new<'a>(ctx: Context<New>) -> Self {
        let (name, email, account_state) = ctx.into_inner().into();
        Self {
            name,
            email,
            account_state,
        }
    }
}
