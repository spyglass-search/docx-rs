use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty},
};

/// Default Style
///
/// This style is inherited by every paragraph and run.
///
/// ```rust
/// use docx::formatting::*;
/// use docx::styles::*;
///
/// let style = DefaultStyle::default()
///     .character(CharacterProperty::default())
///     .paragraph(ParagraphProperty::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(default, child = "rPrDefault")]
    pub character: DefaultCharacterProperty<'a>,
    #[xml(default, child = "pPrDefault")]
    pub paragraph: DefaultParagraphProperty<'a>,
}

impl<'a> DefaultStyle<'a> {
    __setter!(character: DefaultCharacterProperty<'a>);
    __setter!(paragraph: DefaultParagraphProperty<'a>);
}

/// Default Character Properties
#[derive(Default, Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "rPrDefault")]
pub struct DefaultCharacterProperty<'a> {
    /// character properties
    #[xml(default, child = "rPr")]
    pub inner: CharacterProperty<'a>,
}

impl<'a, T: Into<CharacterProperty<'a>>> From<T> for DefaultCharacterProperty<'a> {
    fn from(val: T) -> Self {
        DefaultCharacterProperty { inner: val.into() }
    }
}

/// Default Paragraph Properties
#[derive(Default, Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "pPrDefault")]
pub struct DefaultParagraphProperty<'a> {
    /// paragraph properties
    #[xml(default, child = "pPr")]
    pub inner: ParagraphProperty<'a>,
}

impl<'a, T: Into<ParagraphProperty<'a>>> From<T> for DefaultParagraphProperty<'a> {
    fn from(val: T) -> Self {
        DefaultParagraphProperty { inner: val.into() }
    }
}

__xml_test_suites!(
    DefaultStyle,
    DefaultStyle::default(),
    r#"<docDefaults><rPrDefault><rPr/></rPrDefault><pPrDefault><pPr/></pPrDefault></docDefaults>"#,
);
