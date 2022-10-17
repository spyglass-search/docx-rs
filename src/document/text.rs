use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__string_enum, __xml_test_suites};

/// Literal Text
///
/// A literal text that shall be displayed in the document.
///
/// ```rust
/// use docx::document::{Text, TextSpace};
///
/// let text = Text::from("text");
/// let text = Text::from(String::from("text"));
/// let text = Text::from(("text", TextSpace::Preserve));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "t")]
pub struct Text<'a> {
    /// Specifies how to handle whitespace
    #[xml(attr = "space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: Cow<'a, str>,
}

impl From<String> for Text<'_> {
    fn from(val: String) -> Self {
        Text {
            text: val.into(),
            space: None,
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(val: &'a str) -> Self {
        Text {
            text: val.into(),
            space: None,
        }
    }
}

impl From<(String, TextSpace)> for Text<'_> {
    fn from(val: (String, TextSpace)) -> Self {
        Text {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

impl<'a> From<(&'a str, TextSpace)> for Text<'a> {
    fn from(val: (&'a str, TextSpace)) -> Self {
        Text {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

/// Text Space Rules
///
/// Specifies how whitespace should be handled in a literal text.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TextSpace {
    /// Default rules
    Default,
    /// Using the W3C space preservation rules
    Preserve,
}

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

__xml_test_suites!(
    Text,
    Text::from("text"),
    "<t>text</t>",
    Text::from(String::from("text")),
    "<t>text</t>",
    Text::from(("text", TextSpace::Preserve)),
    r#"<t space="preserve">text</t>"#,
    Text::from((String::from("text"), TextSpace::Default)),
    r#"<t space="default">text</t>"#,
);
