use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "charset")]
pub struct Charset<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Charset<'a> {
    fn from(s: S) -> Self {
        Charset { value: s.into() }
    }
}
