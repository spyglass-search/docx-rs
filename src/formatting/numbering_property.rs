use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;
use crate::formatting::{IndentLevel, NumberingId};

/// Numbering Property
///
/// ```rust
/// use docx::formatting::*;
///
/// let prop = NumberingProperty::from((20, 40));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "numPr")]
pub struct NumberingProperty {
    /// Specifies a reference to a numbering definition instance
    #[xml(child = "numId")]
    pub id: NumberingId,
    /// Specifies the numbering level of the numbering definition to use for the paragraph.
    #[xml(child = "ilvl")]
    pub level: IndentLevel,
}

impl From<(usize, usize)> for NumberingProperty {
    fn from(val: (usize, usize)) -> Self {
        NumberingProperty {
            id: NumberingId { value: val.0 },
            level: IndentLevel { value: val.1 },
        }
    }
}

__xml_test_suites!(
    NumberingProperty,
    NumberingProperty::default(),
    r#"<numPr><numId val="0"/><ilvl val="0"/></numPr>"#,
    NumberingProperty::from((20, 40)),
    r#"<numPr><numId val="20"/><ilvl val="40"/></numPr>"#,
);
