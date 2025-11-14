//! Library for reading and writing Pandoc JSON documents.
//! Provides basic Pandoc AST types and serde (de)serialization support.

use std::vec;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pandoc {
    #[serde(rename = "pandoc-api-version")]
    version: Vec<i32>,
    pub meta: serde_json::Value,
    pub blocks: Vec<Block>,
}

impl Pandoc {
    pub fn new(blocks: Vec<Block>) -> Self {
        let version = vec![1, 23, 1];
        let meta = serde_json::json!({});
        Self {
            version,
            meta,
            blocks,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Block {
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    CodeBlock((Attr, String)),
    RawBlock((String, String)),
    BlockQuote(Vec<Block>),
    OrderedList((ListAttributes, Vec<Vec<Block>>)),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header((i32, Attr, Vec<Inline>)),
    HorizontalRule,
    Table(
        (
            Attr,
            Caption,
            ColSpecs,
            TableHead,
            Vec<TableBody>,
            TableFoot,
        ),
    ),
    Div((Attr, Vec<Block>)),
    Null,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Inline {
    Str(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    Quoted((QuoteType, Vec<Inline>)),
    Cite((Vec<Citation>, Vec<Inline>)),
    Code((Attr, String)),
    Space,
    SoftBreak,
    LineBreak,
    Math((MathType, String)),
    RawInline((String, String)),
    Link((Attr, Vec<Inline>, Target)),
    Image((Attr, Vec<Inline>, Target)),
    Note(Vec<Block>),
    Span((Attr, Vec<Inline>)),
}

pub type Attr = (String, Vec<String>, Vec<AttrKeyValue>);
pub type AttrKeyValue = (String, String);
pub type ListAttributes = (i32, ListNumberStyle, ListNumberDelim);
pub type Caption = (Option<ShortCaption>, Vec<Block>);
pub type ShortCaption = Vec<Inline>;
pub type ColSpecs = Vec<ColSpec>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColSpec(pub ColWidth, pub Alignment);

#[derive(Debug, Serialize, Deserialize)]
pub enum ColWidth {
    ColWidthDefault,
    ColWidthDouble(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableHead(pub Attr, pub Vec<Row>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TableBody(pub Attr, pub RowHeadColumns, pub Vec<Row>, pub Vec<Row>);
pub type RowHeadColumns = i32;
#[derive(Debug, Serialize, Deserialize)]
pub struct TableFoot(pub Attr, pub Vec<Row>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Row(pub Attr, pub Vec<Cell>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Cell(
    pub Attr,
    pub Alignment,
    pub RowSpan,
    pub ColSpan,
    pub Vec<Block>,
);
pub type RowSpan = i32;
pub type ColSpan = i32;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Citation {
    #[serde(rename = "citationId")]
    pub citation_id: String,
    #[serde(rename = "citationPrefix")]
    pub citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")]
    pub citation_suffix: Vec<Inline>,
    #[serde(rename = "citationMode")]
    pub citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    pub citation_note_num: i32,
    #[serde(rename = "citationHash")]
    pub citation_hash: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum CitationMode {
    AuthorInText,
    NormalCitation,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

pub type Target = (String, String);

#[derive(Debug, Serialize, Deserialize)]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

// ...extend as needed for your use case...
