#[derive(Debug, Clone)]
pub enum ParsingError {
    SyntaxError(String),
}
