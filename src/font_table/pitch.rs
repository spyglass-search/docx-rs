use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pitch")]
pub struct Pitch<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Pitch<'a> {
    fn from(s: S) -> Self {
        Pitch { value: s.into() }
    }
}
