use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Italics
///
/// ```rust
/// use docx::formatting::*;
///
/// let i = Italics::from(false);
/// let i = Italics::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "i")]
pub struct Italics {
    #[xml(attr = "val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Italics {
    fn from(val: T) -> Self {
        Italics { value: val.into() }
    }
}

__xml_test_suites!(
    Italics,
    Italics::default(),
    r#"<i/>"#,
    Italics::from(false),
    r#"<i val="false"/>"#,
    Italics::from(true),
    r#"<i val="true"/>"#,
);
