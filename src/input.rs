use std::io::{self, Write};
use std::fmt::Display;

/// Reads a line of text from standard input.
///
/// The function waits for the user to enter text and press Enter.
///
/// # Returns
///
/// Returns the entered text as a `String`, or an I/O error.
///
/// # Examples
///
/// ```no_run
/// use readio::input;
///
/// let name = input().expect("Failed to read input");
/// println!("Hello, {name}!");
/// ```
pub fn input() -> io::Result<String> {
    let mut valor = String::new();

    io::stdin().read_line(&mut valor)?;

    // Remove apenas as quebras de linha.
    while valor.ends_with('\n') || valor.ends_with('\r') {
        valor.pop();
    }

    Ok(valor)
}

/// Displays a prompt and reads a line of text from standard input.
///
/// # Arguments
///
/// * `mensagem` - The message displayed before reading input.
///
/// # Returns
///
/// Returns the entered text as a `String`, or an I/O error.
///
/// # Examples
///
/// ```no_run
/// use readio::prompt;
///
/// let name = prompt("Enter your name: ")?;
/// println!("Hello, {name}!");
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn prompt(mensagem: &str) -> io::Result<String> {
    print!("{mensagem}");
    io::stdout().flush()?;
    input()
}
/// Reads an integer from standard input.
///
/// The function displays a prompt and converts the entered value to an integer.
/// Invalid input handling depends on the implementation.
///
/// # Arguments
///
/// * `mensagem` - The message displayed before reading input.
///
/// # Returns
///
/// Returns the integer value or an I/O error.
///
/// # Examples
///
/// ```no_run
/// use readio::input_int;
///
/// let age = input_int("Enter your age: ")?;
/// println!("Age: {age}");
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn input_int(mensagem: &str) -> io::Result<i64> {
    loop {
        let valor = prompt(mensagem)?;

        match valor.trim().parse::<i64>() {
            Ok(numero) => return Ok(numero),
            Err(_) => println!("Erro: introduza um número inteiro válido."),
        }
    }
}
/// Reads a floating-point number from standard input.
///
/// # Arguments
///
/// * `mensagem` - The message displayed before reading input.
///
/// # Returns
///
/// Returns the numeric value or an I/O error.
///
/// # Examples
///
/// ```no_run
/// use readio::input_float;
///
/// let price = input_float("Enter a price: ")?;
/// println!("Price: {price}");
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn input_float(mensagem: &str) -> io::Result<f64> {
    loop {
        let valor = prompt(mensagem)?;

        match valor.trim().parse::<f64>() {
            Ok(numero) if numero.is_finite() => return Ok(numero),
            _ => println!("Erro: introduza um número decimal válido."),
        }
    }
}
/// Asks the user to confirm an operation.
///
/// The accepted responses and their interpretation depend on the implementation.
///
/// # Arguments
///
/// * `mensagem` - The confirmation question displayed to the user.
///
/// # Returns
///
/// Returns `true` or `false` according to the user's response,
/// or an I/O error.
///
/// # Examples
///
/// ```no_run
/// use readio::confirm;
///
/// if confirm("Do you want to continue? ")? {
///     println!("Continuing...");
/// } else {
///     println!("Operation cancelled.");
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn confirm(mensagem: &str) -> io::Result<bool> {
    loop {
        let resposta = prompt(&format!("{mensagem} [s/n]: "))?;
        let resposta = resposta.trim().to_lowercase();

        match resposta.as_str() {
            "s" | "sim" | "y" | "yes" => return Ok(true),
            "n" | "nao" | "não" | "no" => return Ok(false),
            _ => println!("Resposta inválida. Escreva s ou n."),
        }
    }
}
/// Reads input and parses it into the requested type.
///
/// If the input cannot be parsed, the function may ask the user to try again,
/// depending on the implementation.
///
/// # Type Parameters
///
/// * `T` - The type into which the input will be parsed.
///
/// # Arguments
///
/// * `mensagem` - The message displayed before reading input.
///
/// # Returns
///
/// Returns a value of type `T` wrapped in `io::Result`.
///
/// # Examples
///
/// ```no_run
/// use readio::input_parse;
///
/// let age: u32 = input_parse("Enter your age: ")?;
/// let active: bool = input_parse("Is the account active? ")?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn input_parse<T>(mensagem: &str) -> io::Result<T>
where
    T: std::str::FromStr,
    T::Err: Display,
{
    loop {
        let valor = super::input::prompt(mensagem)?;

        match valor.trim().parse::<T>() {
            Ok(resultado) => return Ok(resultado),
            Err(erro) => {
                println!("Entrada inválida: {erro}");
            }
        }
    }
}
