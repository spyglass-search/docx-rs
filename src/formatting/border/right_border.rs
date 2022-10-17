use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "right")]
pub struct RightBorder<'a> {
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

impl<'a> RightBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    RightBorder,
    RightBorder::default(),
    r#"<right/>"#,
    RightBorder::default().color("000000"),
    r#"<right color="000000"/>"#,
    RightBorder::default().shadow(false),
    r#"<right shadow="false"/>"#,
    RightBorder::default().space(40usize),
    r#"<right space="40"/>"#,
    RightBorder::default().size(20usize),
    r#"<right sz="20"/>"#,
    RightBorder::default().style(BorderStyle::Dotted),
    r#"<right val="dotted"/>"#,
);
