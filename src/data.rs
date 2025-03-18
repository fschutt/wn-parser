use crate::common::{Frame, Pointer, PointerSymbol, Synset, SynsetType, Word};
use std::str::FromStr;

/// Parse a line from a data.* file into a Synset
pub fn parse_data_line(line: &str) -> Result<Synset, &'static str> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return Err("Invalid data line format: missing '|' separator");
    }

    let data_part = parts[0].trim();
    let gloss = parts[1].trim().to_string();

    let fields: Vec<&str> = data_part.split_whitespace().collect();
    if fields.len() < 6 {
        return Err("Invalid data line format: insufficient fields");
    }

    // Parse fixed fields
    let offset = u64::from_str(fields[0]).map_err(|_| "Invalid offset")?;
    let lex_filenum = u8::from_str(fields[1]).map_err(|_| "Invalid lex_filenum")?;
    let ss_type = SynsetType::from(fields[2]);
    
    // Parse word count (in hex)
    let w_cnt = u16::from_str_radix(fields[3], 16).map_err(|_| "Invalid word count")?;
    
    // Extract words
    let mut words = Vec::new();
    let mut current_index = 4;
    for _ in 0..w_cnt {
        let word_str = fields[current_index];
        current_index += 1;
        
        // Next field is lex_id (in hex)
        if current_index >= fields.len() {
            return Err("Invalid data line format: missing lex_id field");
        }
        let lex_id = u8::from_str_radix(fields[current_index], 16).map_err(|_| "Invalid lex_id")?;
        current_index += 1;
        
        words.push(Word {
            word: word_str.to_string(),
            lex_id,
        });
    }
    
    // Parse pointer count
    if current_index >= fields.len() {
        return Err("Invalid data line format: missing pointer count field");
    }
    let p_cnt = u16::from_str(fields[current_index]).map_err(|_| "Invalid pointer count")?;
    current_index += 1;
    
    // Extract pointers
    let mut pointers = Vec::new();
    for _ in 0..p_cnt {
        if current_index + 3 >= fields.len() {
            return Err("Invalid data line format: insufficient pointer fields");
        }
        
        let pointer_symbol = PointerSymbol::from(fields[current_index]);
        current_index += 1;
        
        let pointer_offset = u64::from_str(fields[current_index]).map_err(|_| "Invalid pointer offset")?;
        current_index += 1;
        
        let pos = SynsetType::from(fields[current_index]);
        current_index += 1;
        
        let source_target_str = fields[current_index];
        current_index += 1;
        
        if source_target_str.len() != 4 {
            return Err("Invalid source/target field length");
        }
        
        let source = u16::from_str_radix(&source_target_str[0..2], 16).map_err(|_| "Invalid source field")?;
        let target = u16::from_str_radix(&source_target_str[2..4], 16).map_err(|_| "Invalid target field")?;
        
        pointers.push(Pointer {
            symbol: pointer_symbol,
            offset: pointer_offset,
            pos,
            source_target: (source, target),
        });
    }
    
    // For verbs, parse frames
    let mut frames = Vec::new();
    if matches!(ss_type, SynsetType::Verb) && current_index < fields.len() {
        // Check if there's a frame count
        if let Ok(f_cnt) = u16::from_str(fields[current_index]) {
            current_index += 1;
            
            // Parse frames
            for _ in 0..f_cnt {
                if current_index + 2 >= fields.len() || fields[current_index] != "+" {
                    return Err("Invalid frame format");
                }
                
                current_index += 1; // Skip the '+'
                
                let f_num = u16::from_str(fields[current_index]).map_err(|_| "Invalid frame number")?;
                current_index += 1;
                
                let w_num = u16::from_str_radix(fields[current_index], 16).map_err(|_| "Invalid word number")?;
                current_index += 1;
                
                frames.push(Frame {
                    frame_number: f_num,
                    word_number: w_num,
                });
            }
        }
    }

    Ok(Synset {
        offset,
        lex_filenum,
        ss_type,
        words,
        pointers,
        frames,
        gloss,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_adjective_line() {
        let line = "00001740 00 a 01 able 0 005 = 05200169 n 0000 = 05616246 n 0000 + 05616246 n 0101 + 05200169 n 0101 ! 00002098 a 0101 | (usually followed by `to') having the necessary means or skill or know-how or authority to do something; \"able to swim\"; \"she was able to program her computer\"; \"we were at last able to buy a car\"; \"able to get a grant for the project\"";
        let synset = parse_data_line(line).unwrap();
        
        assert_eq!(synset.offset, 1740);
        assert_eq!(synset.lex_filenum, 0);
        assert!(matches!(synset.ss_type, SynsetType::Adjective));
        assert_eq!(synset.words.len(), 1);
        assert_eq!(synset.words[0].word, "able");
        assert_eq!(synset.words[0].lex_id, 0);
        assert_eq!(synset.pointers.len(), 5);
        assert!(matches!(synset.pointers[0].symbol, PointerSymbol::Attribute));
        assert_eq!(synset.pointers[0].offset, 5200169);
        assert!(matches!(synset.pointers[0].pos, SynsetType::Noun));
        assert_eq!(synset.pointers[0].source_target, (0, 0));
    }
    
    #[test]
    fn test_parse_verb_line() {
        let line = "02409148 41 v 02 overwork 0 exploit 0 008 @ 02407987 v 0000 + 01867768 a 0203 + 01867768 a 0201 + 01867768 a 0202 + 00948206 n 0201 + 00623370 n 0101 + 00623370 n 0102 ~ 02408530 v 0000 02 + 08 00 + 09 00 | work excessively hard; \"he is exploiting the students\"";
        let synset = parse_data_line(line).unwrap();
        
        assert_eq!(synset.offset, 2409148);
        assert_eq!(synset.lex_filenum, 41);
        assert!(matches!(synset.ss_type, SynsetType::Verb));
        assert_eq!(synset.words.len(), 2);
        assert_eq!(synset.words[0].word, "overwork");
        assert_eq!(synset.words[0].lex_id, 0);
        assert_eq!(synset.words[1].word, "exploit");
        assert_eq!(synset.words[1].lex_id, 0);
        assert_eq!(synset.pointers.len(), 8);
        assert_eq!(synset.frames.len(), 2);
        assert_eq!(synset.frames[0].frame_number, 8);
        assert_eq!(synset.frames[0].word_number, 0);
        assert_eq!(synset.frames[1].frame_number, 9);
        assert_eq!(synset.frames[1].word_number, 0);
    }
}