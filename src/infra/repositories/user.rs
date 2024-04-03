use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::infra::db::schema::users;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub mail: String,
    pub hashed_password: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub mail: String,
    pub hashed_password: String,
}
