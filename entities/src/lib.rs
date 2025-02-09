use derive_more::{Deref, DerefMut, Display, Into};
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// What is the state of the Account.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Display, Serialize, Deserialize)]
pub enum AccountState {
    Active,
    Deleted,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct UserName(pub Name);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct InscriptionDate(pub Date);

impl Default for InscriptionDate {
    fn default() -> Self {
        Self::new()
    }
}

impl InscriptionDate {
    pub fn new() -> Self {
        InscriptionDate(Date(Zoned::now()))
    }
}

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[display("Name: {first_name} {last_name}, Email: {email}, Inscription date: {inscription_date}, Account state: {account_state}, Id: {id}")]
pub struct User {
    id: Id,
    pub first_name: UserName,
    pub last_name: UserName,
    pub email: Email,
    /// This attribute is not displayed with the `Display` trait.
    pub order_list: Vec<Order>,
    pub inscription_date: InscriptionDate,
    /// This attribute is not directly accessible. That's due to
    account_state: AccountState,
}

impl User {
    pub fn new(first_name: UserName, last_name: UserName, email: Email) -> Self {
        let id = Id::new();
        let inscription_date = InscriptionDate::new();
        let account_state = AccountState::Active;
        Self {
            first_name,
            last_name,
            email,
            inscription_date,
            account_state,
            id,
            order_list: Vec::default(),
        }
    }

    pub fn get_account_state(&self) -> &AccountState {
        &self.account_state
    }

    pub fn delete_account(&mut self) {
        self.account_state = AccountState::Deleted
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn set_id(&mut self, new: Id) {
        self.id = new;
        for order in self.order_list.iter_mut() {
            order.from_user_id = new
        }
    }
}

/// Date at which a product was introduced
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct IntroductionDate(pub Date);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct ProductName(pub Name);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct OwnerFirstName(pub Name);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct OwnerLastName(pub Name);

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct ProductEndDate(pub Date);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Product {
    pub id: Id,
    pub introduction_date: IntroductionDate,
    pub product_name: ProductName,
    pub end_date: Option<ProductEndDate>,
}
impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end_date_str = match &self.end_date {
            None => String::from("None"),
            Some(ped) => ped.to_string(),
        };
        write!(
            f,
            "Product name: {}, introduction date: {}, end date: {}, Id: {}",
            self.product_name, self.introduction_date, end_date_str, self.id
        )
    }
}

impl Product {
    /// Create a new product. It will have a unique `Id` and its introduction will be the current
    /// datetime when this function is called.
    pub fn new(product_name: ProductName, end_date: Option<ProductEndDate>) -> Self {
        Self {
            id: Id::new(),
            introduction_date: IntroductionDate(Date(Zoned::now())),
            product_name,
            end_date,
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
)]
pub struct OrderDate(Date);

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[display("Order from: {from_user_id}, product: {product}, date: {date}, id: {id}")]
pub struct Order {
    pub date: OrderDate,
    pub product: Product,
    pub from_user_id: Id,
    pub id: Id,
}

impl Order {
    pub fn new(from_user: Id, product: Product) -> Self {
        Self {
            date: OrderDate(Date(Zoned::now())),
            product,
            from_user_id: from_user,
            id: Id::new(),
        }
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Into,
    Deserialize,
    Serialize,
)]
pub struct Email(pub String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Into,
    Deserialize,
    Serialize,
)]
pub struct Name(pub String);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Into,
    Copy,
    Serialize,
    Deserialize,
)]
pub struct Id(pub Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Id(Uuid::now_v7())
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Display,
    Deref,
    DerefMut,
    Into,
    Serialize,
    Deserialize,
)]
pub struct Date(pub Zoned);
