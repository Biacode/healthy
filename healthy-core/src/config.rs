/// Implement this trait for custom configuration.
pub trait Configuration {}

/// Implement this trait for custom parser error.
pub trait ParserError {}

/// Implement this trait for custom configuration parser.
pub trait Parser<T: Configuration> {
    /// Parsing error type
    type ParserError: ParserError;

    /// Parses configuration and returns result of parse.
    fn parse(self) -> Result<T, Self::ParserError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}