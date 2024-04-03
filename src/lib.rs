use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub mod domain;
pub mod infra;
pub mod config;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;