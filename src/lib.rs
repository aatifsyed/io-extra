//! An extension trait for [`io::Error`], with shorthand constructors for various
//! [`io::ErrorKind`]s.
//!
//! ```
//! use std::io;
//! use io_extra::IoErrorExt as _;
//!
//! fn read_to_string(mut r: impl io::Read) -> io::Result<String> {
//!     let mut buf = vec![];
//!     r.read_to_end(&mut buf)?;
//!     String::from_utf8(buf).map_err(io::Error::invalid_data)
//! }
//!
//! fn check_magic_number(mut r: impl io::Read) -> io::Result<()> {
//!     let mut buf = [0; 2];
//!     r.read_exact(&mut buf)?;
//!     match buf == 0xDEAD_u16.to_le_bytes() {
//!         true => Ok(()),
//!         false => Err(io::Error::invalid_data("unrecognised format"))
//!     }
//! }
//! ```
use sealed::Sealed;
use std::{
    error::Error,
    io::{
        self,
        ErrorKind::{
            AddrInUse, AddrNotAvailable, AlreadyExists, BrokenPipe, ConnectionAborted,
            ConnectionRefused, ConnectionReset, Interrupted, InvalidData, InvalidInput,
            NotConnected, NotFound, OutOfMemory, PermissionDenied, TimedOut, UnexpectedEof,
            Unsupported, WouldBlock, WriteZero,
        },
    },
};

mod sealed {
    pub trait Sealed {}
}

macro_rules! ctor {
    ($($name:ident -> $kind:expr),* $(,)?) => {
        $(
            #[doc = concat!(
                "Create an [`io::Error`] with kind [`",
                stringify!($kind),
                "`], wrapping the passed in `error`."
            )]
            fn $name(error: impl Into<Box<dyn Error + Send + Sync>>) -> io::Error {
                io::Error::new($kind, error)
            }
        )*
    };
}

/// An extension trait for [`io::Error`], with shorthand constructors for various
/// [`io::ErrorKind`]s.
///
/// ```
/// use std::io;
/// use io_extra::IoErrorExt as _;
///
/// fn read_to_string(mut r: impl io::Read) -> io::Result<String> {
///     let mut buf = vec![];
///     r.read_to_end(&mut buf)?;
///     String::from_utf8(buf).map_err(io::Error::invalid_data)
/// }
///
/// fn check_magic_number(mut r: impl io::Read) -> io::Result<()> {
///     let mut buf = [0; 2];
///     r.read_exact(&mut buf)?;
///     match buf == 0xDEAD_u16.to_le_bytes() {
///         true => Ok(()),
///         false => Err(io::Error::invalid_data("unrecognised format"))
///     }
/// }
/// ```
///
/// This trait is _sealed_, and cannot be implemented by types outside this library.
pub trait IoErrorExt: Sealed {
    ctor! {
        addr_in_use -> AddrInUse,
        addr_not_available -> AddrNotAvailable,
        already_exists -> AlreadyExists,
        broken_pipe -> BrokenPipe,
        connection_aborted -> ConnectionAborted,
        connection_refused -> ConnectionRefused,
        connection_reset -> ConnectionReset,
        interrupted -> Interrupted,
        invalid_data -> InvalidData,
        invalid_input -> InvalidInput,
        not_connected -> NotConnected,
        not_found -> NotFound,
        out_of_memory -> OutOfMemory,
        permission_denied -> PermissionDenied,
        timed_out -> TimedOut,
        unexpected_eof -> UnexpectedEof,
        unsupported -> Unsupported,
        would_block -> WouldBlock,
        write_zero -> WriteZero,
    }
}

impl Sealed for io::Error {}
impl IoErrorExt for io::Error {}
