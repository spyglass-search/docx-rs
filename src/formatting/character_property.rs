use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter, __xml_test_suites,
    formatting::{Bold, Color, Dstrike, Italics, Outline, Size, Strike, Underline},
};

/// Character Property
///
/// ```rust
/// use docx::formatting::{CharacterProperty, UnderlineStyle};
///
/// let prop = CharacterProperty::default()
///     .style_id("foo")
///     .color("00ff00")
///     .color(0xff0000)
///     .color((0x00, 0x00, 0xff))
///     .size(42usize)
///     .bold(true)
///     .italics(false)
///     .strike(true)
///     .dstrike(false)
///     .outline(true)
///     .underline("00ff00")
///     .underline(("ff0000", UnderlineStyle::Dash));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "rPr")]
pub struct CharacterProperty<'a> {
    /// Specifies the style ID of the character style.
    #[xml(child = "rStyle")]
    pub style_id: Option<CharacterStyleId<'a>>,
    /// Specifies the color to be used to display text.
    #[xml(child = "color")]
    pub color: Option<Color<'a>>,
    /// Specifies the font size in half points.
    #[xml(child = "sz")]
    pub size: Option<Size>,
    /// Specifies that the text of the text run is to be bold.
    #[xml(child = "b")]
    pub bold: Option<Bold>,
    /// Specifies that the text of the text run is to be italics.
    #[xml(child = "i")]
    pub italics: Option<Italics>,
    /// Specifies that the contents are to be displayed with a horizontal line through the center of the line.
    #[xml(child = "strike")]
    pub strike: Option<Strike>,
    /// Specifies that the contents are to be displayed with two horizontal lines through each character.
    #[xml(child = "dstrike")]
    pub dstrike: Option<Dstrike>,
    /// Specifies that the content should be displayed as if it had an outline.
    #[xml(child = "outline")]
    pub outline: Option<Outline>,
    /// Specifies that the content should be displayed with an underline
    #[xml(child = "u")]
    pub underline: Option<Underline<'a>>,
}

impl<'a> CharacterProperty<'a> {
    __setter!(style_id: Option<CharacterStyleId<'a>>);
    __setter!(color: Option<Color<'a>>);
    __setter!(bold: Option<Bold>);
    __setter!(dstrike: Option<Dstrike>);
    __setter!(italics: Option<Italics>);
    __setter!(outline: Option<Outline>);
    __setter!(strike: Option<Strike>);
    __setter!(size: Option<Size>);
    __setter!(underline: Option<Underline<'a>>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "rStyle")]
pub struct CharacterStyleId<'a> {
    #[xml(attr = "val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for CharacterStyleId<'a> {
    fn from(val: T) -> Self {
        CharacterStyleId { value: val.into() }
    }
}

__xml_test_suites!(
    CharacterProperty,
    CharacterProperty::default(),
    r#"<rPr/>"#,
    CharacterProperty::default().style_id("id"),
    r#"<rPr><rStyle val="id"/></rPr>"#,
    CharacterProperty::default().color("00ff00"),
    r#"<rPr><color val="00ff00"/></rPr>"#,
    CharacterProperty::default().size(42usize),
    r#"<rPr><sz val="42"/></rPr>"#,
    CharacterProperty::default().bold(true),
    r#"<rPr><b val="true"/></rPr>"#,
    CharacterProperty::default().italics(false),
    r#"<rPr><i val="false"/></rPr>"#,
    CharacterProperty::default().outline(true),
    r#"<rPr><outline val="true"/></rPr>"#,
    CharacterProperty::default().strike(false),
    r#"<rPr><strike val="false"/></rPr>"#,
    CharacterProperty::default().dstrike(true),
    r#"<rPr><dstrike val="true"/></rPr>"#,
    CharacterProperty::default().underline(Underline::default()),
    r#"<rPr><u/></rPr>"#,
);
