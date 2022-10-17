use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Bold
///
/// ```rust
/// use docx::formatting::*;
///
/// let bold = Bold::from(false);
/// let bold = Bold::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "b")]
pub struct Bold {
    #[xml(attr = "val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Bold {
    fn from(val: T) -> Self {
        Bold { value: val.into() }
    }
}

__xml_test_suites!(
    Bold,
    Bold::default(),
    r#"<b/>"#,
    Bold::from(false),
    r#"<b val="false"/>"#,
    Bold::from(true),
    r#"<b val="true"/>"#,
);
