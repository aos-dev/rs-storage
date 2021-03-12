use std::error;
use std::fmt;
use std::fmt::Debug;
use std::io;
use std::io::{Read, Write};
use std::path::{Iter, Path};

use thiserror::Error;

mod internal;
mod object;

pub use internal::Pair;
pub use object::{Object, ObjectIterator, ObjectMode};

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Storager {
    fn list(&self, path: &str, pairs: &[Pair]) -> Result<ObjectIterator>;
    fn delete(&self, path: &str, pairs: &[Pair]) -> Result<()>;
    fn stat(&self, path: &str, pairs: &[Pair]) -> Result<Object>;
    fn read(&self, path: &str, w: &mut impl io::Write, pairs: &[Pair]) -> Result<i64>;
    fn write(&self, path: &str, r: &mut impl io::Read, size: i64, pairs: &[Pair]) -> Result<i64>;
}

pub struct Interceptor {}

pub struct IoCallback {}

pub struct ListMode {}

pub struct PairPolicy {}
