use crate::common::SenseEntry;
use std::str::FromStr;

/// Parse a line from the index.sense file into a SenseEntry
pub fn parse_sense_line(line: &str) -> Result<SenseEntry, &'static str> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 4 {
        return Err("Invalid sense index line format: insufficient fields");
    }

    let sense_key = fields[0].to_string();
    let synset_offset = u64::from_str(fields[1]).map_err(|_| "Invalid synset offset")?;
    let sense_number = u32::from_str(fields[2]).map_err(|_| "Invalid sense number")?;
    let tag_cnt = u32::from_str(fields[3]).map_err(|_| "Invalid tag count")?;

    Ok(SenseEntry {
        sense_key,
        synset_offset,
        sense_number,
        tag_cnt,
    })
}

/// Utility function to parse a sense key into its components
pub fn parse_sense_key(sense_key: &str) -> Result<(String, String), &'static str> {
    let parts: Vec<&str> = sense_key.split('%').collect();
    if parts.len() != 2 {
        return Err("Invalid sense key format: missing '%' separator");
    }

    let lemma = parts[0].to_string();
    let lex_sense = parts[1].to_string();

    Ok((lemma, lex_sense))
}

/// Utility function to further parse the lex_sense part of a sense key
pub fn parse_lex_sense(lex_sense: &str) -> Result<(u8, u8, u8, String, u8), &'static str> {
    let parts: Vec<&str> = lex_sense.split(':').collect();
    if parts.len() != 5 {
        return Err("Invalid lex_sense format: should have 5 colon-separated parts");
    }

    let ss_type = u8::from_str(parts[0]).map_err(|_| "Invalid ss_type")?;
    let lex_filenum = u8::from_str(parts[1]).map_err(|_| "Invalid lex_filenum")?;
    let lex_id = u8::from_str(parts[2]).map_err(|_| "Invalid lex_id")?;
    let head_word = parts[3].to_string();
    let head_id = if parts[4].is_empty() {
        0
    } else {
        u8::from_str(parts[4]).map_err(|_| "Invalid head_id")?
    };

    Ok((ss_type, lex_filenum, lex_id, head_word, head_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_sense_line() {
        let line = "abandonment%1:04:03:: 00204439 1 3";
        let sense_entry = parse_sense_line(line).unwrap();
        
        assert_eq!(sense_entry.sense_key, "abandonment%1:04:03::");
        assert_eq!(sense_entry.synset_offset, 204439);
        assert_eq!(sense_entry.sense_number, 1);
        assert_eq!(sense_entry.tag_cnt, 3);
    }
    
    #[test]
    fn test_parse_sense_key() {
        let (lemma, lex_sense) = parse_sense_key("abandonment%1:04:03::").unwrap();
        assert_eq!(lemma, "abandonment");
        assert_eq!(lex_sense, "1:04:03::");
    }
    
    #[test]
    fn test_parse_lex_sense() {
        let (ss_type, lex_filenum, lex_id, head_word, head_id) = 
            parse_lex_sense("1:04:03::").unwrap();
        
        assert_eq!(ss_type, 1);
        assert_eq!(lex_filenum, 4);
        assert_eq!(lex_id, 3);
        assert_eq!(head_word, "");
        assert_eq!(head_id, 0);
    }
    
    #[test]
    fn test_parse_lex_sense_with_head() {
        let (ss_type, lex_filenum, lex_id, head_word, head_id) = 
            parse_lex_sense("5:00:00:discomposed:00").unwrap();
        
        assert_eq!(ss_type, 5);
        assert_eq!(lex_filenum, 0);
        assert_eq!(lex_id, 0);
        assert_eq!(head_word, "discomposed");
        assert_eq!(head_id, 0);
    }
}