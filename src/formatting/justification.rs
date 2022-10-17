use hard_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites};

/// Justification
///
/// ```rust
/// use docx::formatting::*;
///
/// let jc = Justification::from(JustificationVal::Start);
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "jc")]
pub struct Justification {
    #[xml(attr = "val")]
    pub value: JustificationVal,
}

impl From<JustificationVal> for Justification {
    fn from(value: JustificationVal) -> Self {
        Justification { value }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum JustificationVal {
    Start,
    End,
    Center,
    Both,
    Distribute,
    Right,
    Left,
}

__string_enum! {
    JustificationVal {
        Start = "start",
        End = "end",
        Center = "center",
        Both = "both",
        Distribute = "distribute",
        Right = "right",
        Left = "left",
    }
}

__xml_test_suites!(
    Justification,
    Justification::from(JustificationVal::Start),
    r#"<jc val="start"/>"#,
);
