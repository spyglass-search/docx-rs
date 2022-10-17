use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    font_table::{Charset, Family, Pitch},
};

/// Font
///
/// ```rust
/// use docx::font_table::*;
///
/// let font = Font::new("Arial")
///     .charset("00")
///     .family("swiss")
///     .pitch("variable");
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "font")]
pub struct Font<'a> {
    #[xml(attr = "name")]
    pub name: Cow<'a, str>,
    #[xml(child = "charset")]
    pub charset: Option<Charset<'a>>,
    #[xml(child = "family")]
    pub family: Option<Family<'a>>,
    #[xml(child = "pitch")]
    pub pitch: Option<Pitch<'a>>,
}

impl<'a> Font<'a> {
    __setter!(charset: Option<Charset<'a>>);
    __setter!(family: Option<Family<'a>>);
    __setter!(pitch: Option<Pitch<'a>>);

    pub fn new<T: Into<Cow<'a, str>>>(name: T) -> Self {
        Font {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for Font<'a> {
    fn from(val: T) -> Self {
        Font::new(val)
    }
}

__xml_test_suites!(
    Font,
    Font::new("Arial"),
    r#"<font name="Arial"/>"#,
    Font::new("Arial").charset("00"),
    r#"<font name="Arial"><charset val="00"/></font>"#,
    Font::new("Arial").family("swiss"),
    r#"<font name="Arial"><family val="swiss"/></font>"#,
    Font::new("Arial").pitch("variable"),
    r#"<font name="Arial"><pitch val="variable"/></font>"#,
);
