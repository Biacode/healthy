//! Author - Arthur Asatryan<br/>
//! Email - biacoder@gmail.com

use super::*;

/// Email configuration.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfiguration {
    from: String,
    to: String,
    content: String
}

impl Configuration for EmailConfiguration {}

impl EmailConfiguration {
    /// Creates default configuration.
    ///
    /// Chain with builder methods to construct configuration instead of parsing it.
    /// # Example
    /// ```rust
    ///EmailConfiguration::new()
    ///    .to("foo@bar.com")
    ///    .from("biacoder@gmail.com")
    ///    .content("<p>Hello, World!</p>");
    /// ```
    pub fn new() -> Self {
        debug!("Creating a new default email configuration");
        EmailConfiguration {
            from: "".to_owned(),
            to: "".to_owned(),
            content: "".to_owned()
        }
    }

    /// To email.
    pub fn to(mut self, to: String) -> Self {
        self.to = to;
        self
    }

    /// From email.
    pub fn from(mut self, from: String) -> Self {
        self.from = from;
        self
    }

    /// Either html or plain text.
    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn has_email_configuration_builder() {
        let to = "foo@bar.com".to_owned();
        let from = "biacoder@gmail.com".to_owned();
        let content = "<p>Hello, World!</p>".to_owned();
        let builder = EmailConfiguration::new()
            .to(to.clone())
            .from(from.clone())
            .content(content.clone());
        assert_eq!(to, builder.to);
        assert_eq!(from, builder.from);
        assert_eq!(content, builder.content);
    }
}