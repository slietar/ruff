#[allow(unused_imports)]
use alloc::{borrow::ToOwned, boxed::Box, format, string::{String, ToString}, vec, vec::Vec};

use crate::{Expr, Stmt};

#[derive(Clone)]
pub enum Node<'a> {
    Stmt(&'a Stmt),
    Expr(&'a Expr),
}
