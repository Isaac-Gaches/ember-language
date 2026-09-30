pub struct LexerError {
    pub pos: usize,
}

pub struct ParserError {
    pub pos: usize,
    pub message: String,
}

pub struct RuntimeError{
    pub message: String,
}
