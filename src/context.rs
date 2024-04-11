use std::{error::Error, fmt, io, iter};

/// Attach a message to this [`io::Error`].
///
/// This is provided as a free function to not conflict with [`anyhow::Context`]
///
/// [`anyhow::Context`]: (https://docs.rs/anyhow/1/anyhow/trait.Context.html#method.context).
pub fn context(e: io::Error, context: impl fmt::Display) -> io::Error {
    let kind = e.kind();
    let stringified = e.to_string();
    let source = match (
        e.raw_os_error(),
        stringified == kind.to_string(),
        e.into_inner(),
    ) {
        // ErrorData::Custom
        (_, _, Some(source)) => Some(source),
        // ErrorData::Os
        (Some(code), _, None) => Some(Box::new(io::Error::from_raw_os_error(code)) as _),
        // ErrorData::Simple
        (None, true, None) => None,
        // ErrorData::SimpleMessage
        (None, false, None) => Some(Box::new(SimpleMessage(stringified)) as _),
    };
    io::Error::new(
        kind,
        Context {
            context: context.to_string(),
            source,
        },
    )
}

#[derive(Debug)]
struct SimpleMessage(String);
impl fmt::Display for SimpleMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for SimpleMessage {}

#[derive(Debug)]
struct Context {
    context: String,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}
impl Error for Context {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(it) => Some(it.as_ref()),
            None => None,
        }
    }
}
impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.context.fmt(f)?;
        if f.alternate() {
            for parent in Chain::new(self.source()) {
                write!(f, ": {}", parent)?
            }
        }
        Ok(())
    }
}

/// An iterator of [`Error::source`]s.
#[derive(Debug)]
struct Chain<'a> {
    #[allow(clippy::type_complexity)]
    inner: iter::Successors<&'a dyn Error, fn(&&'a dyn Error) -> Option<&'a dyn Error>>,
}

impl<'a> Chain<'a> {
    fn new(root: Option<&'a dyn Error>) -> Self {
        Self {
            inner: iter::successors(root, |e| (*e).source()),
        }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a dyn Error;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
