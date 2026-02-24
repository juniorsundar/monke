pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
    }
}
