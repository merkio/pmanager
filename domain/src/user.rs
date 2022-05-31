use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: Role,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    USER,
    ADMIN,
    GUEST,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Role, Self::Err> {
        match input.to_lowercase().as_str() {
            "user" => Ok(Role::USER),
            "admin" => Ok(Role::ADMIN),
            "guest" => Ok(Role::GUEST),
            _ => Err(()),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_owned(),
            email: "".to_owned(),
            password: "".to_owned(),
            enabled: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            role: Role::GUEST,
        }
    }
}

#[allow(dead_code)]
impl User {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = email.to_owned();
        self
    }

    pub fn with_password(mut self, password: &str) -> Self {
        self.password = password.to_owned();
        self
    }

    pub fn enable(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }
}
