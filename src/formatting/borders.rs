use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{BetweenBorder, BottomBorder, LeftBorder, RightBorder, TopBorder},
};

/// Borders
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pBdr")]
pub struct Borders<'a> {
    #[xml(child = "top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "bottom")]
    pub bottom: Option<BottomBorder<'a>>,
    #[xml(child = "left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "between")]
    pub between: Option<BetweenBorder<'a>>,
}

impl<'a> Borders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
    __setter!(left: Option<LeftBorder<'a>>);
    __setter!(right: Option<RightBorder<'a>>);
    __setter!(between: Option<BetweenBorder<'a>>);
}

__xml_test_suites!(
    Borders,
    Borders::default(),
    r#"<pBdr/>"#,
    Borders::default().top(TopBorder::default()),
    r#"<pBdr><top/></pBdr>"#,
    Borders::default().bottom(BottomBorder::default()),
    r#"<pBdr><bottom/></pBdr>"#,
    Borders::default().left(LeftBorder::default()),
    r#"<pBdr><left/></pBdr>"#,
    Borders::default().right(RightBorder::default()),
    r#"<pBdr><right/></pBdr>"#,
    Borders::default().between(BetweenBorder::default()),
    r#"<pBdr><between/></pBdr>"#,
);
