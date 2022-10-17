use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tcPr")]
pub struct TableCellProperty {}

impl TableCellProperty {}

__xml_test_suites!(
    TableCellProperty,
    TableCellProperty::default(),
    r#"<tcPr/>"#,
);
