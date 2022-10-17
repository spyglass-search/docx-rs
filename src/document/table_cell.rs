use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, document::Paragraph, formatting::TableCellProperty};

/// Table Cell
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let cell = TableCell::from(Paragraph::default());
///
/// let cell = TableCell::pargraph(Paragraph::default())
///     .property(TableCellProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tc")]
pub struct TableCell<'a> {
    #[xml(default, child = "tcPr")]
    pub property: TableCellProperty,
    #[xml(child = "p")]
    pub content: Vec<TableCellContent<'a>>,
}

impl<'a> TableCell<'a> {
    __setter!(property: TableCellProperty);

    pub fn pargraph<T: Into<Paragraph<'a>>>(par: T) -> Self {
        TableCell {
            property: TableCellProperty::default(),
            content: vec![TableCellContent::Paragraph(par.into())],
        }
    }
}

impl<'a, T: Into<TableCellContent<'a>>> From<T> for TableCell<'a> {
    fn from(content: T) -> Self {
        TableCell {
            property: TableCellProperty::default(),
            content: vec![content.into()],
        }
    }
}

#[derive(Debug, From, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableCellContent<'a> {
    #[xml(tag = "p")]
    Paragraph(Paragraph<'a>),
    // #[xml(tag = "tbl")]
    // Table(Table<'a>),
}

__xml_test_suites!(
    TableCell,
    TableCell::pargraph(Paragraph::default()),
    "<tc><tcPr/><p><pPr/></p></tc>",
);
