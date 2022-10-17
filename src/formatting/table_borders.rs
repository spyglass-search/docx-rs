use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{BottomBorder, TopBorder},
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "bottom")]
    pub bottom: Option<BottomBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
}

__xml_test_suites!(
    TableBorders,
    TableBorders::default(),
    r#"<tblBorders/>"#,
    TableBorders::default().top(TopBorder::default()),
    r#"<tblBorders><top/></tblBorders>"#,
    TableBorders::default().bottom(BottomBorder::default()),
    r#"<tblBorders><bottom/></tblBorders>"#,
);
