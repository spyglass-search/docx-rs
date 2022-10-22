use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    formatting::{Borders, Justification, NumberingProperty},
};

/// Paragraph Property
///
/// ```rust
/// use docx::formatting::{ParagraphProperty, JustificationVal};
///
/// let prop = ParagraphProperty::default()
///     .style_id("foo")
///     .justification(JustificationVal::Start)
///     .numbering((10usize, 20usize));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pPr")]
pub struct ParagraphProperty<'a> {
    /// Specifies the style ID of the paragraph style.
    #[xml(child = "pStyle")]
    pub style_id: Option<ParagraphStyleId<'a>>,
    /// Specifies the paragraph alignment.
    #[xml(child = "jc")]
    pub justification: Option<Justification>,
    /// Specifies borders for the paragraph.
    #[xml(child = "pBdr")]
    pub border: Option<Borders<'a>>,
    /// Specifies that the paragraph should be numbered.
    #[xml(child = "numPr")]
    pub numbering: Option<NumberingProperty>,
}

impl<'a> ParagraphProperty<'a> {
    __setter!(style_id: Option<ParagraphStyleId<'a>>);
    __setter!(justification: Option<Justification>);
    __setter!(border: Option<Borders<'a>>);
    __setter!(numbering: Option<NumberingProperty>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pStyle")]
pub struct ParagraphStyleId<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for ParagraphStyleId<'a> {
    fn from(val: T) -> Self {
        ParagraphStyleId { value: val.into() }
    }
}

#[cfg(test)]
use crate::formatting::JustificationVal;

__xml_test_suites!(
    ParagraphProperty,
    ParagraphProperty::default(),
    r#"<pPr/>"#,
    ParagraphProperty::default().style_id("id"),
    r#"<pPr><pStyle val="id"/></pPr>"#,
    ParagraphProperty::default().justification(JustificationVal::Start),
    r#"<pPr><jc val="start"/></pPr>"#,
    ParagraphProperty::default().border(Borders::default()),
    r#"<pPr><pBdr/></pPr>"#,
    ParagraphProperty::default().numbering(NumberingProperty::default()),
    r#"<pPr><numPr/></pPr>"#,
);
