//! Data structures for parsing CSN (Core Schema Notation) files into Rust using serde.
//! See csn.md for specification.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap};

/// The top-level CSN model structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnModel {
    /// Imported models (optional)
    pub requires: Option<Vec<String>>,
    /// Dictionary of named definitions (entities, types, services, aspects, etc.)
    pub definitions: BTreeMap<String, CsnDefinition>,
    /// Unapplied extensions or annotations (optional)
    pub extensions: Option<Vec<CsnExtension>>,
    /// i18n translations (optional)
    pub i18n: Option<BTreeMap<String, BTreeMap<String, String>>>,
    /// Any other top-level properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// A CSN definition: entity, type, service, aspect, or other.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum CsnDefinition {
    /// Entity definition (structured type with key(s), elements, etc.)
    #[serde(rename = "entity")]
    Entity(CsnEntity),
    /// Type definition (scalar, structured, arrayed, enum, etc.)
    #[serde(rename = "type")]
    Type(CsnType),
    /// Service definition
    #[serde(rename = "service")]
    Service(CsnService),
    /// Aspect definition (reusable structure)
    #[serde(rename = "aspect")]
    Aspect(CsnAspect),
    /// Context definition
    #[serde(rename = "context")]
    Context(CsnContext),
    /// Action or function definition
    #[serde(rename = "action")]
    Action(CsnAction),
    #[serde(rename = "function")]
    Function(CsnAction),
    /// Fallback for unknown or untyped definitions
    #[serde(other)]
    Other,
}

/// Entity definition: structured type with elements, keys, queries, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnEntity {
    /// Dictionary of elements (fields, associations, etc.)
    pub elements: Option<BTreeMap<String, CsnElement>>,
    /// Optional key(s) for the entity
    pub key: Option<bool>,
    /// Optional query (for views)
    pub query: Option<serde_json::Value>,
    /// Optional projection (for views)
    pub projection: Option<serde_json::Value>,
    /// Optional parameters (for views with params)
    pub params: Option<BTreeMap<String, CsnType>>,
    /// Optional includes (for aspects, inheritance)
    pub includes: Option<Vec<String>>,
    /// Optional actions/functions bound to the entity
    pub actions: Option<BTreeMap<String, CsnAction>>,
    /// Any other entity-level properties (annotations, etc.)
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Type definition: scalar, structured, arrayed, enum, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnType {
    /// The base type (e.g., cds.String, cds.Association, etc.)
    #[serde(rename = "type")]
    pub base_type: Option<String>,
    /// Structured type elements
    pub elements: Option<BTreeMap<String, CsnElement>>,
    /// Arrayed type items
    pub items: Option<Box<CsnType>>,
    /// Enumeration members
    #[serde(rename = "enum")]
    pub members: Option<BTreeMap<String, CsnEnumMember>>,
    /// Constant literal value or calculation expression
    pub value: Option<serde_json::Value>,
    /// Default value or expression
    pub default: Option<serde_json::Value>,
    /// Localized flag (for i18n)
    pub localized: Option<bool>,
    /// Any other type-specific properties (length, precision, scale, etc.)
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Service definition
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnService {
    /// Any service-level properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Aspect definition (reusable structure)
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnAspect {
    /// Dictionary of elements
    pub elements: Option<BTreeMap<String, CsnElement>>,
    /// Any other aspect-level properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Context definition (grouping)
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnContext {
    /// Any context-level properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Action or function definition
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnAction {
    /// Parameters for the action/function
    pub params: Option<BTreeMap<String, CsnType>>,
    /// Return type definition
    pub returns: Option<Box<CsnType>>,
    /// Any other properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Element definition: field, association, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnElement {
    /// The type of the element (scalar, association, etc.)
    #[serde(rename = "type")]
    pub datatype: Option<String>,
    /// Is this element a key?
    pub key: Option<bool>,
    /// Not null constraint
    pub not_null: Option<bool>,
    /// Virtual field (ignored in persistence)
    pub virtual_: Option<bool>,
    /// Association target
    pub target: Option<String>,
    /// Cardinality for associations
    pub cardinality: Option<CsnCardinality>,
    /// On condition for unmanaged associations
    pub on: Option<serde_json::Value>,
    /// Keys for managed associations
    pub keys: Option<Vec<serde_json::Value>>,
    /// Enumeration members
    #[serde(rename = "enum")]
    pub enum_: Option<BTreeMap<String, CsnEnumMember>>,
    /// Constant literal value or calculation expression
    pub value: Option<serde_json::Value>,
    /// Default value or expression
    pub default: Option<serde_json::Value>,
    /// Arrayed type items
    pub items: Option<Box<CsnType>>,
    /// Structured type elements
    pub elements: Option<BTreeMap<String, CsnElement>>,
    /// Any other element-level properties (length, precision, scale, annotations, etc.)
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Enumeration member definition
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnEnumMember {
    /// The literal value of the enum member
    pub val: Option<serde_json::Value>,
    /// Any other enum member properties (annotations, etc.)
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Extension or annotation definition
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnExtension {
    /// Target to extend
    pub extend: Option<String>,
    /// Target to annotate
    pub annotate: Option<String>,
    /// Elements to add or override
    pub elements: Option<BTreeMap<String, CsnElement>>,
    /// Any other extension-level properties
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

/// Cardinality for associations
#[derive(Debug, Serialize, Deserialize)]
pub struct CsnCardinality {
    /// Source cardinality
    pub src: Option<u32>,
    /// Minimum cardinality
    pub min: Option<u32>,
    /// Maximum cardinality (can be "*" for many)
    pub max: Option<String>,
}
