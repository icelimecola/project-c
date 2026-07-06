pub mod clips;
mod connection;
mod schema;
mod seed;

type DbResult<T> = Result<T, String>;
