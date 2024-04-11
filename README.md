<!-- cargo-rdme start -->

An extension trait for [`io::Error`], with shorthand constructors for various
[`io::ErrorKind`]s, and a [`context()`] method.

```rust
use std::io;
use io_extra::IoErrorExt as _;

fn read_to_string(mut r: impl io::Read) -> io::Result<String> {
    let mut buf = vec![];
    r.read_to_end(&mut buf)?;
    String::from_utf8(buf).map_err(io::Error::invalid_data)
}

fn check_magic_number(mut r: impl io::Read) -> io::Result<()> {
    let mut buf = [0; 2];
    r.read_exact(&mut buf)?;
    match buf == 0xDEAD_u16.to_le_bytes() {
        true => Ok(()),
        false => Err(io::Error::invalid_data("unrecognised format"))
    }
}
```

<!-- cargo-rdme end -->
