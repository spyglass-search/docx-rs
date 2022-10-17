use hard_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Grid Column
///
/// ```rust
/// use docx::document::*;
///
/// let col = GridColumn::from(42);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "gridCol")]
pub struct GridColumn {
    #[xml(attr = "w")]
    pub width: usize,
}

impl From<usize> for GridColumn {
    fn from(width: usize) -> GridColumn {
        GridColumn { width }
    }
}

__xml_test_suites!(
    GridColumn,
    GridColumn::from(42usize),
    r#"<gridCol w="42"/>"#,
);
