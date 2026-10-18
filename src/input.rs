use std::io::{self, Write};
use std::fmt::Display;

/// Lê uma linha de texto sem mostrar uma mensagem.
pub fn input() -> io::Result<String> {
    let mut valor = String::new();

    io::stdin().read_line(&mut valor)?;

    // Remove apenas as quebras de linha.
    while valor.ends_with('\n') || valor.ends_with('\r') {
        valor.pop();
    }

    Ok(valor)
}

/// Mostra uma mensagem e lê a resposta.
pub fn prompt(mensagem: &str) -> io::Result<String> {
    print!("{mensagem}");
    io::stdout().flush()?;

    input()
}

/// Lê um número inteiro, repetindo até receber um valor válido.
pub fn input_int(mensagem: &str) -> io::Result<i64> {
    loop {
        let valor = prompt(mensagem)?;

        match valor.trim().parse::<i64>() {
            Ok(numero) => return Ok(numero),
            Err(_) => println!("Erro: introduza um número inteiro válido."),
        }
    }
}

/// Lê um número decimal, repetindo até receber um valor válido.
pub fn input_float(mensagem: &str) -> io::Result<f64> {
    loop {
        let valor = prompt(mensagem)?;

        match valor.trim().parse::<f64>() {
            Ok(numero) if numero.is_finite() => return Ok(numero),
            _ => println!("Erro: introduza um número decimal válido."),
        }
    }
}

/// Solicita uma confirmação com sim ou não.
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

/// Lê um valor da consola e converte-o para o tipo solicitado.
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
