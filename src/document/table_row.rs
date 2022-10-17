use hard_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites, document::TableCell, formatting::TableRowProperty};

/// Table Row
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let row = TableRow::default()
///     .property(TableRowProperty::default())
///     .push_cell(Paragraph::default())
///     .push_cell(
///         TableCell::pargraph(Paragraph::default())
///             .property(TableCellProperty::default())
///     );
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tr")]
pub struct TableRow<'a> {
    #[xml(default, child = "trPr")]
    pub property: TableRowProperty,
    #[xml(child = "tc")]
    pub cells: Vec<TableCell<'a>>,
}

impl<'a> TableRow<'a> {
    __setter!(property: TableRowProperty);

    pub fn push_cell<T: Into<TableCell<'a>>>(mut self, cell: T) -> Self {
        self.cells.push(cell.into());
        self
    }
}

#[cfg(test)]
use crate::document::Paragraph;

__xml_test_suites!(
    TableRow,
    TableRow::default(),
    "<tr><trPr/></tr>",
    TableRow::default().push_cell(Paragraph::default()),
    "<tr><trPr/><tc><tcPr/><p><pPr/></p></tc></tr>",
);
