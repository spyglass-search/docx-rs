use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    document::{r#break::Break, text::Text},
    formatting::CharacterProperty,
};

/// Run
///
/// Run is a non-block region of text with properties.
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let run = Run::default()
///     .property(CharacterProperty::default())
///     .push_text("text")
///     .push_break(None)
///     .push_text((" text ", TextSpace::Preserve))
///     .push_break(BreakType::Column);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "r")]
pub struct Run<'a> {
    /// Specifies the properties of a run
    ///
    /// Just as paragraph, a run's properties is applied to all the contents of the run.
    #[xml(default, child = "rPr")]
    pub property: CharacterProperty<'a>,
    #[xml(child = "t", child = "br")]
    /// Specifies the content of a run
    pub content: Vec<RunContent<'a>>,
}

impl<'a> Run<'a> {
    __setter!(property: CharacterProperty<'a>);

    #[inline(always)]
    pub fn push<T: Into<RunContent<'a>>>(mut self, content: T) -> Self {
        self.content.push(content.into());
        self
    }

    #[inline(always)]
    pub fn push_text<T: Into<Text<'a>>>(mut self, content: T) -> Self {
        self.content.push(RunContent::Text(content.into()));
        self
    }

    #[inline(always)]
    pub fn push_break<T: Into<Break>>(mut self, br: T) -> Self {
        self.content.push(RunContent::Break(br.into()));
        self
    }

    pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
        self.content.iter().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::Break(_) => None,
        })
    }

    pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
        self.content.iter_mut().filter_map(|content| match content {
            RunContent::Text(Text { text, .. }) => Some(text),
            RunContent::Break(_) => None,
        })
    }
}

/// A set of elements that can be contained as the content of a run.
#[derive(Debug, From, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RunContent<'a> {
    #[xml(tag = "t")]
    Text(Text<'a>),
    #[xml(tag = "br")]
    Break(Break),
}

__xml_test_suites!(
    Run,
    Run::default(),
    r#"<r><rPr/></r>"#,
    Run::default().push_break(None),
    r#"<r><rPr/><br/></r>"#,
    Run::default().push_text("text"),
    r#"<r><rPr/><t>text</t></r>"#,
);
