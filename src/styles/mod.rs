//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod style;

pub use self::{default_style::*, style::*};

use hard_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};
use std::io::Write;

use crate::__xml_test_suites;
use crate::schema::SCHEMA_MAIN;

/// Styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
///
/// ```rust
/// use docx::styles::*;
///
/// let style = Styles::new()
///     .default(DefaultStyle::default())
///     .push(Style::new(StyleType::Paragraph, "style_id"));
/// ```
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "styles")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(default, child = "docDefaults")]
    pub default: DefaultStyle<'a>,
    /// Specifies a set of properties.
    #[xml(child = "style")]
    pub styles: Vec<Style<'a>>,
}

impl<'a> XmlWrite for Styles<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Styles { default, styles } = self;

        log::debug!("[Styles] Started writing.");

        writer.write_element_start("styles")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_element_end_open()?;

        default.to_writer(writer)?;

        for ele in styles {
            ele.to_writer(writer)?;
        }

        writer.write_element_end_close("styles")?;

        log::debug!("[Styles] Finished writing.");

        Ok(())
    }
}

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        <Styles as Default>::default()
    }

    pub fn default(&mut self, style: DefaultStyle<'a>) -> &mut Self {
        self.default = style;
        self
    }

    pub fn push(&mut self, style: Style<'a>) -> &mut Self {
        self.styles.push(style);
        self
    }
}

__xml_test_suites!(
    Styles,
    Styles::new(),
    format!(
        r#"<styles xmlns:w="{}"><docDefaults><rPrDefault><rPr/></rPrDefault><pPrDefault><pPr/></pPrDefault></docDefaults></styles>"#,
        SCHEMA_MAIN
    )
    .as_str(),
    Styles {
        styles: vec![Style::new(crate::styles::StyleType::Paragraph, "id")],
        ..Default::default()
    },
    format!(
        r#"<styles xmlns:w="{}"><docDefaults><rPrDefault><rPr/></rPrDefault><pPrDefault><pPr/></pPrDefault></docDefaults><style type="paragraph" styleId="id"><pPr/><rPr/></style></styles>"#,
        SCHEMA_MAIN
    )
    .as_str(),
);
