use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "family")]
pub struct Family<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for Family<'a> {
    fn from(s: S) -> Self {
        Family { value: s.into() }
    }
}
