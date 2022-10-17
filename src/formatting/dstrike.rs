use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Double Strike
///
/// ```rust
/// use docx::formatting::*;
///
/// let dstrike = Dstrike::from(false);
/// let dstrike = Dstrike::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "dstrike")]
pub struct Dstrike {
    #[xml(attr = "val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Dstrike {
    fn from(val: T) -> Self {
        Dstrike { value: val.into() }
    }
}

__xml_test_suites!(
    Dstrike,
    Dstrike::default(),
    r#"<dstrike/>"#,
    Dstrike::from(false),
    r#"<dstrike val="false"/>"#,
    Dstrike::from(true),
    r#"<dstrike val="true"/>"#,
);
