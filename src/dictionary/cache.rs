use super::trie::PrefixTrie;
use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

/// stores trie structure + dictionary
pub struct CacheManager;

impl CacheManager {
    fn cache_path(source_path: &Path) -> std::path::PathBuf {
        let mut path = source_path.to_path_buf();
        let mut extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        extension.push_str(".cache");
        path.set_extension(extension);
        path
    }

    fn is_cache_valid(cache_path: &Path, source_path: &Path) -> bool {
        if !cache_path.exists() {
            return false;
        }

        match (fs::metadata(cache_path), fs::metadata(source_path)) {
            (Ok(cache_meta), Ok(source_meta)) => {
                if let (Ok(cache_time), Ok(source_time)) =
                    (cache_meta.modified(), source_meta.modified())
                {
                    cache_time >= source_time
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn load_or_build_trie(index_path: &Path) -> Result<PrefixTrie> {
        let cache_path = Self::cache_path(index_path);

        if Self::is_cache_valid(&cache_path, index_path) {
            if let Ok(trie) = Self::load_trie_from_cache(&cache_path) {
                return Ok(trie);
            }
        }

        let trie = Self::build_trie_from_index(index_path)?;
        let _ = Self::save_trie_to_cache(&trie, &cache_path);
        Ok(trie)
    }

    pub fn load_or_decompress_dict(dict_path: &Path) -> Result<String> {
        let cache_path = Self::cache_path(dict_path);

        if Self::is_cache_valid(&cache_path, dict_path) {
            if let Ok(content) = Self::load_dict_from_cache(&cache_path) {
                return Ok(content);
            }
        }

        let content = Self::decompress_dict(dict_path)?;
        let _ = Self::save_dict_to_cache(&content, &cache_path);
        Ok(content)
    }

    fn build_trie_from_index(index_path: &Path) -> Result<PrefixTrie> {
        let index_file = File::open(index_path)
            .with_context(|| format!("Failed to open index file: {:?}", index_path))?;
        let reader = BufReader::new(index_file);
        let mut trie = PrefixTrie::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split('\t').collect();

            if parts.len() >= 3 {
                let word = parts[0];
                let offset = Self::decode_dict_number(parts[1])?;
                let length = Self::decode_dict_number(parts[2])?;
                trie.insert(word, offset, length);
            }
        }

        Ok(trie)
    }

    fn decompress_dict(dict_path: &Path) -> Result<String> {
        let dict_file = File::open(dict_path)
            .with_context(|| format!("Failed to open dict file: {:?}", dict_path))?;
        let mut decoder = flate2::read::GzDecoder::new(dict_file);
        let mut content = String::new();
        decoder
            .read_to_string(&mut content)
            .context("Failed to decompress dictionary data")?;
        Ok(content)
    }

    fn save_trie_to_cache(trie: &PrefixTrie, cache_path: &Path) -> Result<()> {
        let file = File::create(cache_path)
            .with_context(|| format!("Failed to create cache file: {:?}", cache_path))?;
        let mut writer = BufWriter::new(file);

        // format: count (8 bytes) + entries
        // each entry: key_len (8 bytes) + key + offset (8 bytes) + length (8 bytes)
        let entries: Vec<(Vec<u8>, (u64, u64))> = trie.to_entries();

        writer.write_all(&(entries.len() as u64).to_le_bytes())?;

        for (key, (offset, length)) in entries {
            writer.write_all(&(key.len() as u64).to_le_bytes())?;
            writer.write_all(&key)?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.write_all(&length.to_le_bytes())?;
        }

        writer.flush()?;
        Ok(())
    }

    fn load_trie_from_cache(cache_path: &Path) -> Result<PrefixTrie> {
        let file = File::open(cache_path)
            .with_context(|| format!("Failed to open cache file: {:?}", cache_path))?;
        let mut reader = BufReader::new(file);

        let mut count_buf = [0u8; 8];
        reader.read_exact(&mut count_buf)?;
        let count = u64::from_le_bytes(count_buf) as usize;

        let mut trie = PrefixTrie::new();

        for _ in 0..count {
            let mut len_buf = [0u8; 8];
            reader.read_exact(&mut len_buf)?;
            let key_len = u64::from_le_bytes(len_buf) as usize;

            let mut key = vec![0u8; key_len];
            reader.read_exact(&mut key)?;

            let mut offset_buf = [0u8; 8];
            reader.read_exact(&mut offset_buf)?;
            let offset = u64::from_le_bytes(offset_buf);

            let mut length_buf = [0u8; 8];
            reader.read_exact(&mut length_buf)?;
            let length = u64::from_le_bytes(length_buf);

            trie.insert_bytes(key, offset, length);
        }

        Ok(trie)
    }

    fn save_dict_to_cache(content: &str, cache_path: &Path) -> Result<()> {
        let mut file = File::create(cache_path)
            .with_context(|| format!("Failed to create cache file: {:?}", cache_path))?;
        file.write_all(content.as_bytes())
            .context("Failed to write cache file")?;
        Ok(())
    }

    fn load_dict_from_cache(cache_path: &Path) -> Result<String> {
        fs::read_to_string(cache_path)
            .with_context(|| format!("Failed to read cache file: {:?}", cache_path))
    }

    fn decode_dict_number(b64_str: &str) -> Result<u64> {
        const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut result: u64 = 0;
        for ch in b64_str.chars() {
            let digit = ALPHABET
                .find(ch)
                .with_context(|| format!("Invalid base64 character '{}' in '{}'", ch, b64_str))?;
            result = result
                .checked_mul(64)
                .and_then(|r| r.checked_add(digit as u64))
                .with_context(|| format!("Number overflow decoding '{}'", b64_str))?;
        }
        Ok(result)
    }
}
