use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites, formatting::BorderStyle};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "between")]
pub struct BetweenBorder<'a> {
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

impl<'a> BetweenBorder<'a> {
    __setter!(color: Option<Cow<'a, str>>);
    __setter!(shadow: Option<bool>);
    __setter!(space: Option<usize>);
    __setter!(size: Option<usize>);
    __setter!(style: Option<BorderStyle>);
}

__xml_test_suites!(
    BetweenBorder,
    BetweenBorder::default(),
    r#"<between/>"#,
    BetweenBorder::default().color("000000"),
    r#"<between color="000000"/>"#,
    BetweenBorder::default().shadow(false),
    r#"<between shadow="false"/>"#,
    BetweenBorder::default().space(40usize),
    r#"<between space="40"/>"#,
    BetweenBorder::default().size(20usize),
    r#"<between sz="20"/>"#,
    BetweenBorder::default().style(BorderStyle::Dotted),
    r#"<between val="dotted"/>"#,
);
