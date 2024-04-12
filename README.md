<!-- cargo-rdme start -->

An extension trait for [`io::Error`], with shorthand constructors for various
[`io::ErrorKind`]s, and a [`context()`] method.

```rust
use std::{fs::File, io::{self, Write as _}, str};
use io_extra::{IoErrorExt as _, context, with};

fn write_log(contents: &[u8], mut file: File) -> io::Result<()> {
    if let Err(e) = str::from_utf8(contents) {
        return Err(io::Error::invalid_input("`contents` was not UTF-8"))
                           // ^ shorthand constructor
    }
    file.write_all(contents).map_err(with("couldn't write file"))
                                  // ^ easily add context
}
```

<!-- cargo-rdme end -->
