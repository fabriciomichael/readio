use std::io::{self, Write};

/// Tenta limpar o conteúdo visível do terminal.
pub fn clear() -> io::Result<()> {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush()
}
