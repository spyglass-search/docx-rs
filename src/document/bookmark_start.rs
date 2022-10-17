use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{__setter, __xml_test_suites};

/// Beginning of bookmark
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "bookmarkStart")]
pub struct BookmarkStart<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the bookmark name.
    #[xml(attr = "name")]
    pub name: Option<Cow<'a, str>>,
}

impl<'a> BookmarkStart<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(name: Option<Cow<'a, str>>);
}

__xml_test_suites!(
    BookmarkStart,
    BookmarkStart::default(),
    r#"<bookmarkStart/>"#,
    BookmarkStart::default().id("id"),
    r#"<bookmarkStart id="id"/>"#,
    BookmarkStart::default().name("name"),
    r#"<bookmarkStart name="name"/>"#,
);
