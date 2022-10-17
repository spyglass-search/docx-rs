use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    formatting::{TableBorders, TableIndent, TableJustification, TableWidth},
};

/// Table Property
///
/// ```rust
/// use docx::formatting::*;
///
/// let prop = TableProperty::default()
///     .style_id("foo")
///     .justification(TableJustificationVal::Start)
///     .indent((50, TableIndentUnit::Pct))
///     .width((50, TableWidthUnit::Pct));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tblPr")]
pub struct TableProperty<'a> {
    #[xml(child = "tblStyle")]
    pub style_id: Option<TableStyleId<'a>>,
    #[xml(child = "jc")]
    pub justification: Option<TableJustification>,
    #[xml(child = "tblBorders")]
    pub borders: Option<TableBorders<'a>>,
    #[xml(child = "tblInd")]
    pub indent: Option<TableIndent>,
    #[xml(child = "tblW")]
    pub width: Option<TableWidth>,
}

impl<'a> TableProperty<'a> {
    __setter!(style_id: Option<TableStyleId<'a>>);
    __setter!(justification: Option<TableJustification>);
    __setter!(borders: Option<TableBorders<'a>>);
    __setter!(indent: Option<TableIndent>);
    __setter!(width: Option<TableWidth>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tblStyle")]
pub struct TableStyleId<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for TableStyleId<'a> {
    fn from(val: T) -> Self {
        TableStyleId { value: val.into() }
    }
}

__xml_test_suites!(
    TableProperty,
    TableProperty::default(),
    r#"<tblPr/>"#,
    TableProperty::default().style_id("id"),
    r#"<tblPr><tblStyle val="id"/></tblPr>"#,
    TableProperty::default().justification(crate::formatting::TableJustificationVal::Start),
    r#"<tblPr><jc val="start"/></tblPr>"#,
    TableProperty::default().borders(TableBorders::default()),
    r#"<tblPr><tblBorders/></tblPr>"#,
    TableProperty::default().indent(TableIndent::default()),
    r#"<tblPr><tblInd/></tblPr>"#,
    TableProperty::default().width(TableWidth::default()),
    r#"<tblPr><tblW/></tblPr>"#,
);
