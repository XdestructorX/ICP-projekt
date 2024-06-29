use uuid::Uuid;
use diesel::Insertable;
use bcrypt::{DEFAULT_COST, hash};

use crate::schema::users;

#[derive(Insertable, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub haslo: String,
    pub unique_id: String,
}

impl NewUser {
    pub fn new(username: String, haslo: String) -> NewUser {
        let hashed_password: String = hash(haslo.as_str(), DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        return NewUser {
            username,
            haslo: hashed_password,
            unique_id: uuid,
        };
    }
}