use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::{fmt, io, mem};

use bitflags::bitflags;

use crate::Result;
use crate::Storager;

bitflags! {
    #[derive(Default)]
    pub struct ObjectMode: u32 {
        const DIR = 1 << 0;
        const READ = 1 << 1;
        const LINK = 1 << 2;
        const PART = 1 << 3;
        const BLOCK = 1 << 4;
        const PAGE = 1 << 5;
        const APPEND = 1 << 6;
    }
}

#[derive(Default)]
pub struct Object {
    o: super::internal::Object,

    store: Option<Box<dyn Storager>>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.o.fmt(f)
    }
}

impl Deref for Object {
    type Target = super::internal::Object;

    fn deref(&self) -> &Self::Target {
        &self.o
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.o
    }
}

impl Object {
    pub fn new(store: Box<dyn Storager>) -> Object {
        Object {
            o: super::internal::Object::default(),
            store: Some(store),
        }
    }
}

pub struct ObjectIterator {
    o: Object,
}

impl Iterator for ObjectIterator {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        Some(mem::take(&mut self.o))
    }
}
