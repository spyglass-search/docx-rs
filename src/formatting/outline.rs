use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Outline
///
/// ```rust
/// use docx::formatting::*;
///
/// let outline = Outline::from(false);
/// let outline = Outline::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "outline")]
pub struct Outline {
    #[xml(attr = "val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Outline {
    fn from(val: T) -> Self {
        Outline { value: val.into() }
    }
}

__xml_test_suites!(
    Outline,
    Outline::default(),
    r#"<outline/>"#,
    Outline::from(false),
    r#"<outline val="false"/>"#,
    Outline::from(true),
    r#"<outline val="true"/>"#,
);
