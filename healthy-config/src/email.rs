use healthy_core::Configuration;

/// Email configuration.
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfiguration {
    from: String,
    to: String,
    content: String
}

impl Configuration for EmailConfiguration {}

impl EmailConfiguration {
    /// Creates default configuration.
    /// Chain with builder methods to construct configuration instead of parsing it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate healthy_config;
    /// healthy_config::EmailConfiguration::new()
    ///     .to("foo@bar.com".to_owned())
    ///     .from("biacoder@gmail.com".to_owned())
    ///     .content("<p>Hello, World!</p>".to_owned());
    /// ```
    pub fn new() -> Self {
        debug!("Creating a new default email configuration");
        Default::default()
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