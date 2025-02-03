use lazy_static::lazy_static;
use nutype::nutype;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref UUID_REGEX: Regex =
        Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
}

#[nutype(
    sanitize(trim, lowercase),
    validate(
        len_char_min = 5,
        len_char_max = 20,
        regex = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
    ),
    derive(
        Serialize,
        Deserialize,
        Display,
        Debug,
        Clone,
        AsRef,
        Deref,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct EmailAddress(String);

#[nutype(
    sanitize(trim),
    validate(
        len_char_min = 36,
        len_char_max = 36,
        regex = "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
    ),
    derive(
        Serialize,
        Deserialize,
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct UUID(String);

#[nutype(
    sanitize(trim),
    validate(len_char_min = 6, len_char_max = 12,),
    derive(
        Serialize,
        Deserialize,
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct Password(String);

#[nutype(
    validate(greater_or_equal = 16, less_or_equal = 150),
    derive(
        Serialize,
        Deserialize,
        Debug,
        Clone,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash
    )
)]
pub struct Age(u16);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub uuid: UUID,
    pub email: EmailAddress,
    pub password: Password,
    pub age: Option<Age>,
}

impl User {
    pub fn new(id: u64, uuid: &str, email: &str, password: &str, age: u16) -> anyhow::Result<Self> {
        let user = User {
            id,
            uuid: UUID::try_new(uuid.to_string())?,
            email: EmailAddress::try_new(email.to_string())?,
            password: Password::try_new(password.to_string())?,
            age: Age::try_new(age).ok(),
        };
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age() {
        let user_age = Age::try_new(32).unwrap();
        println!("{:?}", user_age)
    }

    #[test]
    fn test_email() {
        let email = EmailAddress::try_new("deMo@example.com").unwrap();
        println!("{}", email);
        println!("uppercase: {}", email.to_uppercase());
    }

    #[test]
    fn test_user() {
        let user = User::new(
            1,
            "a50c4df7-6c08-428d-ad5a-258a2fadfed7",
            "demo@example.com",
            "$ww6%234",
            36,
        )
        .unwrap();
        println!("{}", serde_json::to_string(&user).unwrap());
    }
}
