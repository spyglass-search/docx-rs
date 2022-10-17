use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "top")]
pub struct TopBorder<'a> {
    #[xml(attr = "color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "space")]
    pub space: Option<usize>,
    #[xml(attr = "sz")]
    pub size: Option<usize>,
    #[xml(attr = "val")]
    pub style: Option<BorderStyle>,
}

impl<'a> TopBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    TopBorder,
    TopBorder::default(),
    r#"<top/>"#,
    TopBorder::default().color("000000"),
    r#"<top color="000000"/>"#,
    TopBorder::default().shadow(false),
    r#"<top shadow="false"/>"#,
    TopBorder::default().space(40usize),
    r#"<top space="40"/>"#,
    TopBorder::default().size(20usize),
    r#"<top sz="20"/>"#,
    TopBorder::default().style(BorderStyle::Dotted),
    r#"<top val="dotted"/>"#,
);
