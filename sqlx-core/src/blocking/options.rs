use super::{Connection, Runtime};

/// Options which can be used to configure how a SQL connection is opened.
///
/// For detailed information, refer to the asynchronous version of
/// this: [`ConnectOptions`][crate::ConnectOptions].
///
#[allow(clippy::module_name_repetitions)]
pub trait ConnectOptions<Rt>: crate::ConnectOptions<Rt>
where
    Rt: Runtime,
    Self::Connection: crate::Connection<Rt, Options = Self> + Connection<Rt>,
{
    /// Parse a connection URL into connection options.
    #[inline]
    fn parse(url: &str) -> crate::Result<Self> {
        <Self as crate::ConnectOptions<Rt>>::parse(url)
    }

    /// Establish a connection to the database.
    ///
    /// For detailed information, refer to the asynchronous version of
    /// this: [`connect()`][crate::ConnectOptions::connect].
    ///
    fn connect(&self) -> crate::Result<Self::Connection>
    where
        Self::Connection: Sized;
}