//! Module for `Docx`.

use quick_xml::Reader as XmlReader;
use quick_xml::{events::*, Writer as XmlWriter};
use std::fs::File;
use std::io::{BufReader, Read, Seek, Write};
use std::path::Path;
use zip::{result::ZipError, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    app::App,
    content_type::ContentTypes,
    core::Core,
    document::{BodyContent, Document, Para},
    errors::{Error, Result},
    font_table::FontTable,
    rels::Relationships,
    schema::*,
    style::{Style, Styles},
};

/// A WordprocessingML package
#[derive(Debug, Default)]
pub struct Docx {
    /// Specifies package-level properties part
    pub app: Option<App>,
    /// Specifies core properties part
    pub core: Option<Core>,
    /// Specifies the content type of relationship parts and the main document part.
    pub content_types: ContentTypes,
    /// Specifies the main document part.
    pub document: Document,
    /// Specifies the font table part
    pub font_table: Option<FontTable>,
    /// Specifies the style definitions part
    pub styles: Option<Styles>,
    /// Specifies the package-level relationship to the main document part
    pub rels: Relationships,
    /// Specifies the part-level relationship to the main document part
    pub document_rels: Option<Relationships>,
}

impl Docx {
    /// Create a style, and returns it.
    pub fn create_style(&mut self) -> &mut Style {
        self.styles.get_or_insert(Styles::default()).create_style()
    }

    pub fn generate<T: Write + Seek>(&mut self, writer: T) -> Result<T> {
        let mut zip = ZipWriter::new(writer);
        let opt = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);

        macro_rules! write {
            ($xml:expr, $name:tt) => {{
                zip.start_file($name, opt)?;
                let mut writer = XmlWriter::new(zip);
                writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)))?;
                $xml.write(&mut writer)?;
                zip = writer.into_inner();
            }};

            ($xml:expr, $name:tt, $rel:expr, $schema:expr, $target:tt) => {{
                write!($xml, $name);
                $rel.add_rel($schema, $target);
            }};
        }

        macro_rules! option_write {
            ($xml:expr, $($rest:tt)*) => {{
                if let Some(ref xml) = $xml {
                    write!(xml, $($rest)*);
                }
            }};
        }

        // content types
        write!(self.content_types, "[Content_Types].xml");

        // document properties
        option_write!(
            self.app,
            "docProps/app.xml",
            self.rels,
            SCHEMA_REL_EXTENDED,
            "docProps/app.xml"
        );
        option_write!(
            self.core,
            "docProps/core.xml",
            self.rels,
            SCHEMA_CORE,
            "docProps/core.xml"
        );

        // documents specific parts
        write!(
            self.document,
            "word/document.xml",
            self.rels, SCHEMA_OFFICE_DOCUMENT, "word/document.xml"
        );
        option_write!(
            self.styles,
            "word/styles.xml",
            self.document_rels.get_or_insert(Relationships::default()),
            SCHEMA_STYLES,
            "styles.xml"
        );
        option_write!(
            self.font_table,
            "word/fontTable.xml",
            self.document_rels.get_or_insert(Relationships::default()),
            SCHEMA_FONT_TABLE,
            "fontTable.xml"
        );

        // relationships
        write!(self.rels, "_rels/.rels");
        option_write!(self.document_rels, "word/_rels/document.xml.rels");

        Ok(zip.finish()?)
    }

    pub fn write_file<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        let file = File::create(path)?;
        self.generate(file)
    }

    pub fn parse<T: Read + Seek>(reader: T) -> Result<Docx> {
        let mut zip = ZipArchive::new(reader)?;

        macro_rules! read {
            ($xml:tt, $name:expr) => {{
                let file = zip.by_name($name)?;
                let mut reader = XmlReader::from_reader(BufReader::new(file));
                $xml::read(&mut reader, None)?
            }};
        }

        macro_rules! option_read {
            ($xml:tt, $name:expr) => {
                match zip.by_name($name) {
                    Err(ZipError::FileNotFound) => None,
                    Err(e) => return Err(Error::Zip(e)),
                    Ok(file) => {
                        let mut reader = XmlReader::from_reader(BufReader::new(file));
                        Some($xml::read(&mut reader, None)?)
                    }
                };
            };
        }

        let app = option_read!(App, "docProps/app.xml");
        let content_types = read!(ContentTypes, "[Content_Types].xml");
        let core = option_read!(Core, "docProps/core.xml");
        let document_rels = option_read!(Relationships, "word/_rels/document.xml.rels");
        let document = read!(Document, "word/document.xml");
        let font_table = option_read!(FontTable, "word/fontTable.xml");
        let rels = read!(Relationships, "_rels/.rels");
        let styles = option_read!(Styles, "word/styles.xml");

        Ok(Docx {
            app,
            content_types,
            core,
            document_rels,
            document,
            font_table,
            rels,
            styles,
        })
    }

    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Docx> {
        let reader = BufReader::new(File::open(path)?);
        Docx::parse(reader)
    }

    #[inline]
    pub fn insert_para(&mut self, para: Para) -> &mut Self {
        self.document.body.content.push(BodyContent::Para(para));
        self
    }

    #[inline]
    pub fn insert_style(&mut self, style: Style) -> &mut Self {
        self.styles
            .get_or_insert(Styles::default())
            .styles
            .push(style);
        self
    }
}