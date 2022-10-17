use hard_xml::{XmlRead, XmlWrite};

use crate::{__string_enum, __xml_test_suites};

/// Table Indent
///
/// ```rust
/// use docx::formatting::*;
///
/// let ind = TableIndent::from(42);
/// let ind = TableIndent::from(TableIndentUnit::Pct);
/// let ind = TableIndent::from((42, TableIndentUnit::Dxa));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tblInd")]
pub struct TableIndent {
    #[xml(attr = "w")]
    pub value: Option<usize>,
    #[xml(attr = "type")]
    pub unit: Option<TableIndentUnit>,
}

impl From<usize> for TableIndent {
    fn from(val: usize) -> Self {
        TableIndent {
            value: Some(val),
            unit: None,
        }
    }
}

impl From<TableIndentUnit> for TableIndent {
    fn from(val: TableIndentUnit) -> Self {
        TableIndent {
            value: None,
            unit: Some(val),
        }
    }
}

impl From<(usize, TableIndentUnit)> for TableIndent {
    fn from(val: (usize, TableIndentUnit)) -> Self {
        TableIndent {
            value: Some(val.0),
            unit: Some(val.1),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableIndentUnit {
    Auto,
    Dxa,
    Nil,
    Pct,
}

__string_enum! {
    TableIndentUnit {
        Auto = "auto",
        Dxa = "dxa",
        Nil = "nil",
        Pct = "pct",
    }
}

__xml_test_suites!(
    TableIndent,
    TableIndent::default(),
    "<tblInd/>",
    TableIndent::from(42),
    r#"<tblInd w="42"/>"#,
    TableIndent::from(TableIndentUnit::Pct),
    r#"<tblInd type="pct"/>"#,
    TableIndent::from((42, TableIndentUnit::Dxa)),
    r#"<tblInd w="42" type="dxa"/>"#,
);
