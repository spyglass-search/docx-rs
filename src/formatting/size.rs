use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Size
///
/// ```rust
/// use docx::formatting::*;
///
/// let sz = Size::from(42usize);
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "sz")]
pub struct Size {
    #[xml(attr = "val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for Size {
    fn from(val: T) -> Self {
        Size { value: val.into() }
    }
}

__xml_test_suites!(Size, Size::from(42usize), r#"<sz val="42"/>"#,);
