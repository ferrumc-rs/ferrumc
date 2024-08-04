// To allow stuff like this:
// pub type EntityIdentity = impl Into<usize>;
#![feature(type_alias_impl_trait)]


mod entity;
mod component;
mod helpers;
mod error;
mod query;
mod tests;
mod query_tests;
mod test;
#[tokio::main]
async fn main() {

}