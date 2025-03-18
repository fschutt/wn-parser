use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum PointerSymbol {
    Antonym,               // !
    Hypernym,              // @
    InstanceHypernym,      // @i
    Hyponym,               // ~
    InstanceHyponym,       // ~i
    Entailment,            // *
    SimilarTo,             // &
    MemberMeronym,         // #m
    SubstanceMeronym,      // #s
    PartMeronym,           // #p
    MemberHolonym,         // %m
    SubstanceHolonym,      // %s
    PartHolonym,           // %p
    Meronym,               // %
    Holonym,               // #
    CauseTo,               // >
    ParticipleOf,          // <
    SeeAlso,               // ^
    Pertainym,             // \
    Attribute,             // =
    VerbGroup,             // $
    DerivationallyRelated, // +
    Classification,        // ;
    ClassificationCategory,// ;c
    ClassificationUsage,   // ;u
    ClassificationRegional,// ;r
    Class,                 // -
    ClassCategory,         // -c
    ClassUsage,            // -u
    ClassRegional,         // -r
    Unknown(String),
}

impl From<&str> for PointerSymbol {
    fn from(s: &str) -> Self {
        match s {
            "!" => PointerSymbol::Antonym,
            "@" => PointerSymbol::Hypernym,
            "@i" => PointerSymbol::InstanceHypernym,
            "~" => PointerSymbol::Hyponym,
            "~i" => PointerSymbol::InstanceHyponym,
            "*" => PointerSymbol::Entailment,
            "&" => PointerSymbol::SimilarTo,
            "#m" => PointerSymbol::MemberMeronym,
            "#s" => PointerSymbol::SubstanceMeronym,
            "#p" => PointerSymbol::PartMeronym,
            "%m" => PointerSymbol::MemberHolonym,
            "%s" => PointerSymbol::SubstanceHolonym,
            "%p" => PointerSymbol::PartHolonym,
            "%" => PointerSymbol::Meronym,
            "#" => PointerSymbol::Holonym,
            ">" => PointerSymbol::CauseTo,
            "<" => PointerSymbol::ParticipleOf,
            "^" => PointerSymbol::SeeAlso,
            "\\" => PointerSymbol::Pertainym,
            "=" => PointerSymbol::Attribute,
            "$" => PointerSymbol::VerbGroup,
            "+" => PointerSymbol::DerivationallyRelated,
            ";" => PointerSymbol::Classification,
            ";c" => PointerSymbol::ClassificationCategory,
            ";u" => PointerSymbol::ClassificationUsage,
            ";r" => PointerSymbol::ClassificationRegional,
            "-" => PointerSymbol::Class,
            "-c" => PointerSymbol::ClassCategory,
            "-u" => PointerSymbol::ClassUsage,
            "-r" => PointerSymbol::ClassRegional,
            _ => PointerSymbol::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynsetType {
    Noun,         // n
    Verb,         // v
    Adjective,    // a
    Adverb,       // r
    AdjectiveSatellite, // s
    Unknown(String),
}

impl From<&str> for SynsetType {
    fn from(s: &str) -> Self {
        match s {
            "n" => SynsetType::Noun,
            "v" => SynsetType::Verb,
            "a" => SynsetType::Adjective,
            "r" => SynsetType::Adverb,
            "s" => SynsetType::AdjectiveSatellite,
            _ => SynsetType::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pointer {
    pub symbol: PointerSymbol,
    pub offset: u64,
    pub pos: SynsetType,
    pub source_target: (u16, u16), // Source and target word numbers
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word {
    pub word: String,
    pub lex_id: u8,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub frame_number: u16,
    pub word_number: u16, // 0 means all words in the synset
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Synset {
    pub offset: u64,
    pub lex_filenum: u8,
    pub ss_type: SynsetType,
    pub words: Vec<Word>,
    pub pointers: Vec<Pointer>,
    pub frames: Vec<Frame>,
    pub gloss: String,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    pub lemma: String,
    pub pos: SynsetType,
    pub synset_cnt: u32,
    pub ptr_symbols: Vec<PointerSymbol>,
    pub sense_cnt: u32,
    pub tagsense_cnt: u32,
    pub synset_offsets: Vec<u64>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenseEntry {
    pub sense_key: String,
    pub synset_offset: u64,
    pub sense_number: u32,
    pub tag_cnt: u32,
}