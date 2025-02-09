use crate::dyn_map::DynMap;
use crate::persistence::DBConds;
use crate::{CanBeCreated, CanBeDeleted, Create, CanBeUpdated};
use crate::{Email, Id, Name};
use derive_more::{Display, From, Into};

/// What is the state of the Account.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display)]
pub enum AccountState {
    Active,
    Dormant,
    Deleted,
}

#[derive(Debug, Display, From, Into)]
pub struct InscriptionDate(/*TODO: Add date in here*/);

impl InscriptionDate {
    pub fn new() -> Self {
        //TODO: Fill this function with appropriate values
        InscriptionDate()
    }
}

#[derive(Debug, Display)]
#[display("Name: {name}, Email: {email}, Inscription date: {inscription_date}, Account state: {account_state}, Id: {id}")]
pub struct User {
    name: Name,
    email: Email,
    inscription_date: InscriptionDate,
    account_state: AccountState,
    id: Id,
    /// This attribute is not displayed with the `Display` trait
    ext: DynMap<&'static str>,
}

impl User {
    pub fn new(name: Name, email: Email) -> Self {
        let id = Id::new();
        let inscription_date = InscriptionDate::new();
        let account_state = AccountState::Active;
        Self {
            name,
            email,
            inscription_date,
            account_state,
            id,
            ext: DynMap::default(),
        }
    }
    pub fn get_name(&self) -> &Name {
        &self.name
    }
    pub fn set_name(&mut self, new: Name) {
        self.name = new
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }
    pub fn set_id(&mut self, new: Id) {
        self.id = new
    }

    pub fn get_email(&self) -> &Email {
        &self.email
    }
    pub fn set_email(&mut self, new: Email) {
        self.email = new
    }

    pub fn get_inscription_date(&self) -> &InscriptionDate {
        &self.inscription_date
    }
    pub fn set_inscription_date(&mut self, new: InscriptionDate) {
        self.inscription_date = new
    }

    pub fn get_account_state(&self) -> &AccountState {
        &self.account_state
    }
    pub fn set_account_state(&mut self, new: AccountState) {
        self.account_state = new
    }

}
impl<'a, DB> CanBeCreated<(DB, &'a Email)> for User
where
    DB: DBConds<&'a Email>,
{
    fn can_be_created(ctx: (DB, &'a Email)) -> bool {
        let (db, email) = ctx;
        if db.exists(email) {
            return false;
        } else {
            true
        }
    }
}
impl Create<(Name, Email)> for User {
    fn create(ctx: (Name, Email)) -> Self {

    }
}
