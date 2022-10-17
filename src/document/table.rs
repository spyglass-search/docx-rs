use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    document::{TableGrid, TableRow},
    formatting::TableProperty,
};

/// Table
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let tbl = Table::default()
///     .property(TableProperty::default())
///     .push_grid(vec![1, 2, 3])
///     .push_grid(TableGrid::default())
///     .push_row(TableRow::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tbl")]
pub struct Table<'a> {
    #[xml(default, child = "tblPr")]
    pub property: TableProperty<'a>,
    #[xml(child = "tblGrid")]
    pub grids: Vec<TableGrid>,
    #[xml(child = "tr")]
    pub rows: Vec<TableRow<'a>>,
}

impl<'a> Table<'a> {
    __setter!(property: TableProperty<'a>);

    pub fn push_grid<T: Into<TableGrid>>(mut self, grid: T) -> Self {
        self.grids.push(grid.into());
        self
    }

    pub fn push_row<T: Into<TableRow<'a>>>(mut self, row: T) -> Self {
        self.rows.push(row.into());
        self
    }
}

__xml_test_suites!(
    Table,
    Table::default(),
    "<tbl><tblPr/></tbl>",
    Table::default().push_grid(TableGrid::default()),
    "<tbl><tblPr/><tblGrid/></tbl>",
    Table::default().push_row(TableRow::default()),
    "<tbl><tblPr/><tr><trPr/></tr></tbl>",
);
