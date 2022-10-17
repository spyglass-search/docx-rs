use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;
use crate::document::GridColumn;

/// Table Grid
///
/// ```rust
/// use docx::document::*;
///
/// let grid = TableGrid::from(vec![42, 42]);
///
/// let grid = TableGrid::default()
///     .push_column(42)
///     .push_column(42);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "tblGrid")]
pub struct TableGrid {
    #[xml(child = "gridCol")]
    pub columns: Vec<GridColumn>,
}

impl TableGrid {
    pub fn push_column<T: Into<GridColumn>>(mut self, col: T) -> Self {
        self.columns.push(col.into());
        self
    }
}

impl From<Vec<usize>> for TableGrid {
    fn from(cols: Vec<usize>) -> TableGrid {
        TableGrid {
            columns: cols.into_iter().map(Into::into).collect(),
        }
    }
}

__xml_test_suites!(
    TableGrid,
    TableGrid::default(),
    "<tblGrid/>",
    TableGrid::default().push_column(42),
    r#"<tblGrid><gridCol w="42"/></tblGrid>"#,
    TableGrid::default().push_column(42).push_column(42),
    r#"<tblGrid><gridCol w="42"/><gridCol w="42"/></tblGrid>"#,
    TableGrid::from(vec![42, 42]),
    r#"<tblGrid><gridCol w="42"/><gridCol w="42"/></tblGrid>"#,
);
