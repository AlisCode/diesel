//! Implementation of a read-only connection

use crate::Connection;

#[derive(Debug)]
/// Wrapper for a read-only documentation
pub struct ReadOnly<C>(pub(crate) C);

/// Extension trait that allows a connection to be wrapped as read-only
pub trait ReadOnlyExt: Sized {
    /// Wraps the connection making it read-only
    fn read_only(self) -> ReadOnly<Self> {
        ReadOnly(self)
    }
}

impl<C: Connection> ReadOnlyExt for C {}
