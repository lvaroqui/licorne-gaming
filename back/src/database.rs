use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type Database = Pool<AsyncPgConnection>;

pub fn init_pool() -> Database {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&database_url);
    Pool::builder(config)
        .build()
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
