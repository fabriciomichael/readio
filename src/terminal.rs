use std::io::{self, Write};

/// Clears the terminal screen.
///
/// The exact behaviour may vary depending on the operating system
/// and the implementation used by the library.
///
/// # Examples
///
/// ```no_run
/// use readio::clear;
///
/// clear();
/// ```
pub fn clear() -> io::Result<()> {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush()
}
