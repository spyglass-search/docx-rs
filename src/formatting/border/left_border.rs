use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "left")]
pub struct LeftBorder<'a> {
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

impl<'a> LeftBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    LeftBorder,
    LeftBorder::default(),
    r#"<left/>"#,
    LeftBorder::default().color("000000"),
    r#"<left color="000000"/>"#,
    LeftBorder::default().shadow(false),
    r#"<left shadow="false"/>"#,
    LeftBorder::default().space(40usize),
    r#"<left space="40"/>"#,
    LeftBorder::default().size(20usize),
    r#"<left sz="20"/>"#,
    LeftBorder::default().style(BorderStyle::Dotted),
    r#"<left val="dotted"/>"#,
);
