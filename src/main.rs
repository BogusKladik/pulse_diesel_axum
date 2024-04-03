use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, Router,
};
// use axum_macros::debug_handler;
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

use pulse_diesel_axum::{
    config::config, infra::{
        db::schema::users,
        repositories::user::{NewUser, User},
    }
};

#[tokio::main]
async fn main() {
    let conf = config().await;

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(conf.db_url());

    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    let app = Router::new()
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

// #[debug_handler]
async fn create_user(
    State(pool): State<Pool>,
    Json(mut new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    // пароль?
    new_user.hashed_password = "Как-то поменяли пароль)".to_string();

    let res = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn list_users(State(pool): State<Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}
