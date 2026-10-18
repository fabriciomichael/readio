# readio

**A lightweight Rust console input library.**

`readio` é uma biblioteca desenvolvida em Rust para simplificar a leitura e a validação de dados introduzidos através da linha de comando. Foi concebida para reduzir o código repetitivo associado à leitura da entrada padrão e facilitar a criação de aplicações CLI (*Command-Line Interface*).

Inspirada na simplicidade da função `input()` do Python, a `readio` disponibiliza funções para ler textos, números, confirmações e outros tipos de dados que possam ser convertidos a partir de strings.

## Funcionalidades

- Leitura de texto através da consola.
- Exibição de mensagens antes da leitura.
- Leitura genérica de diferentes tipos de dados.
- Validação e repetição da leitura quando a conversão falha.
- Leitura de números inteiros e decimais.
- Confirmações com respostas como `s`, `sim`, `n` e `não`.
- Limpeza do terminal através de sequências ANSI.
- Tratamento explícito de erros de entrada e saída.
- Implementação sem dependências externas.

## Requisitos

- Rust instalado.
- Cargo, incluído na instalação habitual do Rust.
- Um terminal compatível com a entrada padrão.

## Instalação

### Utilização local

Adiciona a biblioteca às dependências do teu projecto no ficheiro `Cargo.toml`:

```toml
[dependencies]
readio = { path = "../readio" }
```

Substitui o caminho pelo caminho relativo correcto para a pasta da biblioteca.

### Publicação no crates.io

Se a biblioteca for publicada no crates.io, poderá ser adicionada através da versão publicada:

```toml
[dependencies]
readio = "0.1.0"
```

Esta forma só estará disponível depois de a versão correspondente ser efectivamente publicada.

## Utilização

### 1. Ler texto

A função `prompt()` apresenta uma mensagem e lê o texto introduzido pelo utilizador.

```rust
use readio::prompt;

fn main() -> std::io::Result<()> {
    let nome = prompt("Qual é o teu nome? ")?;

    println!("Olá, {nome}!");

    Ok(())
}
```

### 2. Ler um valor de qualquer tipo compatível

A função genérica `input_parse()` permite converter a entrada para qualquer tipo que implemente `FromStr`.

```rust
use readio::input_parse;

fn main() -> std::io::Result<()> {
    let idade: u32 = input_parse("Introduza a idade: ")?;
    let altura: f64 = input_parse("Introduza a altura: ")?;
    let ativo: bool = input_parse("Está activo? (true/false): ")?;

    println!("Idade: {idade}");
    println!("Altura: {altura}");
    println!("Activo: {ativo}");

    Ok(())
}
```

Se a conversão falhar, a função apresenta uma mensagem de erro e solicita novamente a entrada. Os erros de entrada e saída são propagados através de `io::Result`.

### 3. Ler números inteiros

```rust
use readio::input_int;

fn main() -> std::io::Result<()> {
    let idade = input_int("Idade: ")?;

    println!("Idade introduzida: {idade}");

    Ok(())
}
```

### 4. Ler números decimais

```rust
use readio::input_float;

fn main() -> std::io::Result<()> {
    let preco = input_float("Preço: ")?;

    println!("Preço: {preco:.2}");

    Ok(())
}
```

A função utiliza o formato decimal reconhecido por `f64`, normalmente com ponto, como `125.50`.

### 5. Solicitar uma confirmação

```rust
use readio::confirm;

fn main() -> std::io::Result<()> {
    if confirm("Desejas continuar?")? {
        println!("Operação autorizada.");
    } else {
        println!("Operação cancelada.");
    }

    Ok(())
}
```

A função aceita respostas afirmativas e negativas, incluindo `s`, `sim`, `y`, `yes`, `n`, `nao`, `não` e `no`.

### 6. Limpar o terminal

```rust
use readio::clear;

fn main() -> std::io::Result<()> {
    clear()?;

    println!("Terminal limpo!");

    Ok(())
}
```

A limpeza utiliza sequências de controlo ANSI. O comportamento depende do terminal utilizado.

### 7. Ler texto sem uma mensagem

A função `input()` lê uma linha directamente da entrada padrão.

```rust
use readio::input;

fn main() -> std::io::Result<()> {
    println!("Introduza uma mensagem:");

    let mensagem = input()?;

    println!("Mensagem: {mensagem}");

    Ok(())
}
```

## API pública

| Função | Descrição |
|---|---|
| `input()` | Lê uma linha de texto. |
| `prompt(mensagem)` | Apresenta uma mensagem e lê texto. |
| `input_parse::<T>(mensagem)` | Lê e converte a entrada para um tipo genérico. |
| `input_int(mensagem)` | Lê um número inteiro de 64 bits. |
| `input_float(mensagem)` | Lê um número de ponto flutuante de 64 bits. |
| `confirm(mensagem)` | Solicita uma confirmação afirmativa ou negativa. |
| `clear()` | Tenta limpar o terminal. |

As funções de leitura devolvem `io::Result`, permitindo que a aplicação trate erros de entrada e saída.

## Estrutura do projecto

```text
readio/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── input.rs
    └── terminal.rs
```

- `lib.rs`: declara os módulos e expõe a API pública.
- `input.rs`: implementa as funções de leitura e conversão de dados.
- `terminal.rs`: implementa funcionalidades auxiliares do terminal.

## Compilar e testar

Na raiz do projecto, executa:

```bash
cargo check
cargo test
cargo doc --open
```

- `cargo check`: verifica o código.
- `cargo test`: executa os testes definidos no projecto.
- `cargo doc --open`: gera e abre a documentação da biblioteca.

## Tratamento de erros

As funções de entrada devolvem resultados do tipo `io::Result<T>`. Para propagar os erros, a função que as chama pode utilizar o operador `?`:

```rust
fn main() -> std::io::Result<()> {
    let nome = readio::prompt("Nome: ")?;

    println!("Olá, {nome}!");

    Ok(())
}
```

Se preferires manter a função `main()` sem retorno explícito, podes tratar o resultado com `match` e decidir como proceder em caso de erro.

## Limitações conhecidas

- A validação genérica depende das regras de conversão de cada tipo.
- `input_parse::<bool>()` reconhece os valores aceites pela conversão padrão de `bool`, como `true` e `false`.
- A limpeza do terminal depende do suporte a sequências ANSI.
- A função `input()` lê uma linha por chamada e não implementa leitura de palavras-passe sem eco.
- A função `confirm()` destina-se a confirmações simples e não substitui mecanismos de autenticação ou autorização.

## Objectivo do projecto

O objectivo da `readio` é proporcionar uma API simples, reutilizável e extensível para a leitura de dados em aplicações de consola desenvolvidas em Rust.

O projecto também serve como exercício de aprendizagem sobre módulos, funções genéricas, conversão de tipos, tratamento de erros e organização de bibliotecas Rust.

## Licença

Este projeto está licenciado sob a Licença MIT.
Consulte o arquivo [LICENSE](LICENSE) para obter mais detalhes.
---

**readio** — Simplificando a entrada de dados em Rust.