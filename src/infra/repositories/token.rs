use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::infra::db::schema::tokens;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub user_id: i32,
    pub token: String,
}