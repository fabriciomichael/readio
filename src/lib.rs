pub mod input;
pub mod terminal;

pub use input::{confirm, input, input_float, input_int, input_parse, prompt};
pub use terminal::clear;

#[cfg(test)]
mod tests {
    use super::*;
    use input::input;

    #[test]
    fn it_works_one() {}
}
