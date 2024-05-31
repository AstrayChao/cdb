
pub mod result;
pub mod response;
pub mod request;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1
}
