use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "bottom")]
pub struct BottomBorder<'a> {
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

impl<'a> BottomBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    BottomBorder,
    BottomBorder::default(),
    r#"<bottom/>"#,
    BottomBorder::default().color("000000"),
    r#"<bottom color="000000"/>"#,
    BottomBorder::default().shadow(false),
    r#"<bottom shadow="false"/>"#,
    BottomBorder::default().space(40usize),
    r#"<bottom space="40"/>"#,
    BottomBorder::default().size(20usize),
    r#"<bottom sz="20"/>"#,
    BottomBorder::default().style(BorderStyle::Dotted),
    r#"<bottom val="dotted"/>"#,
);
