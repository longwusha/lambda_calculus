//! **lambda_calculus** is a simple implementation of the untyped lambda calculus in Rust.

#![deny(missing_docs)]

#[macro_use]

pub mod term;
pub mod parser;
pub mod reduction;
pub mod combinators;
pub mod church;
pub mod scott;
pub mod parigot;

pub use self::term::{Term, abs, app};
pub use self::term::Term::*;
pub use self::term::Notation::*;
pub use self::reduction::{beta, beta_verbose};
pub use self::reduction::Order::*;
pub use self::parser::parse;

pub use self::church::conversions::IntoChurch;
pub use self::scott::conversions::IntoScott;
pub use self::parigot::conversions::IntoParigot;
