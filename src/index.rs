use crate::common::{IndexEntry, PointerSymbol, SynsetType};
use std::str::FromStr;

/// Parse a line from an index.* file into an IndexEntry
pub fn parse_index_line(line: &str) -> Result<IndexEntry, &'static str> {
    // Skip lines that begin with space (copyright notices)
    if line.starts_with(' ') {
        return Err("Skipping copyright notice line");
    }

    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 6 {
        return Err("Invalid index line format: insufficient fields");
    }

    // Parse the fixed fields
    let lemma = fields[0].to_string();
    let pos = SynsetType::from(fields[1]);
    let synset_cnt = u32::from_str(fields[2]).map_err(|_| "Invalid synset count")?;
    let p_cnt = u32::from_str(fields[3]).map_err(|_| "Invalid pointer count")?;

    // Parse pointer symbols
    let mut ptr_symbols = Vec::new();
    let mut current_index = 4;
    for _ in 0..p_cnt {
        if current_index >= fields.len() {
            return Err("Invalid index line format: missing pointer symbol fields");
        }
        ptr_symbols.push(PointerSymbol::from(fields[current_index]));
        current_index += 1;
    }

    // Next fields are sense_cnt and tagsense_cnt
    if current_index + 1 >= fields.len() {
        return Err("Invalid index line format: missing sense count fields");
    }
    let sense_cnt = u32::from_str(fields[current_index]).map_err(|_| "Invalid sense count")?;
    current_index += 1;
    
    let tagsense_cnt = u32::from_str(fields[current_index]).map_err(|_| "Invalid tagged sense count")?;
    current_index += 1;

    // Parse synset offsets
    let mut synset_offsets = Vec::new();
    while current_index < fields.len() {
        let offset = u64::from_str(fields[current_index]).map_err(|_| "Invalid synset offset")?;
        synset_offsets.push(offset);
        current_index += 1;
    }

    // Validation - number of offsets should match synset_cnt
    if synset_offsets.len() != synset_cnt as usize {
        return Err("Invalid index line: synset count doesn't match number of offsets");
    }

    Ok(IndexEntry {
        lemma,
        pos,
        synset_cnt,
        ptr_symbols,
        sense_cnt,
        tagsense_cnt,
        synset_offsets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_noun_index_line() {
        let line = "abductor_muscle n 1 2 @ ~ 1 0 05291010";
        let index_entry = parse_index_line(line).unwrap();
        
        assert_eq!(index_entry.lemma, "abductor_muscle");
        assert!(matches!(index_entry.pos, SynsetType::Noun));
        assert_eq!(index_entry.synset_cnt, 1);
        assert_eq!(index_entry.ptr_symbols.len(), 2);
        assert!(matches!(index_entry.ptr_symbols[0], PointerSymbol::Hypernym));
        assert!(matches!(index_entry.ptr_symbols[1], PointerSymbol::Hyponym));
        assert_eq!(index_entry.sense_cnt, 1);
        assert_eq!(index_entry.tagsense_cnt, 0);
        assert_eq!(index_entry.synset_offsets.len(), 1);
        assert_eq!(index_entry.synset_offsets[0], 5291010);
    }
    
    #[test]
    fn test_parse_verb_index_line() {
        let line = "abduct v 2 5 ! @ ~ + ; 2 0 01471043 01449427";
        let index_entry = parse_index_line(line).unwrap();
        
        assert_eq!(index_entry.lemma, "abduct");
        assert!(matches!(index_entry.pos, SynsetType::Verb));
        assert_eq!(index_entry.synset_cnt, 2);
        assert_eq!(index_entry.ptr_symbols.len(), 5);
        assert!(matches!(index_entry.ptr_symbols[0], PointerSymbol::Antonym));
        assert_eq!(index_entry.sense_cnt, 2);
        assert_eq!(index_entry.tagsense_cnt, 0);
        assert_eq!(index_entry.synset_offsets.len(), 2);
        assert_eq!(index_entry.synset_offsets[0], 1471043);
        assert_eq!(index_entry.synset_offsets[1], 1449427);
    }
    
    #[test]
    fn test_parse_adj_index_line() {
        let line = ".22-caliber a 1 1 \\ 1 0 03146310";
        let index_entry = parse_index_line(line).unwrap();
        
        assert_eq!(index_entry.lemma, ".22-caliber");
        assert!(matches!(index_entry.pos, SynsetType::Adjective));
        assert_eq!(index_entry.synset_cnt, 1);
        assert_eq!(index_entry.ptr_symbols.len(), 1);
        assert!(matches!(index_entry.ptr_symbols[0], PointerSymbol::Pertainym));
        assert_eq!(index_entry.sense_cnt, 1);
        assert_eq!(index_entry.tagsense_cnt, 0);
        assert_eq!(index_entry.synset_offsets.len(), 1);
        assert_eq!(index_entry.synset_offsets[0], 3146310);
    }
}