mod cache;
mod models;
mod trie;

pub use cache::CacheManager;
pub use models::DictEntry;
pub use trie::PrefixTrie;

use anyhow::Result;
use std::path::Path;

const MAX_RESULTS: usize = 50;

pub struct Dictionary {
    index: PrefixTrie,
    data_content: String,
}

impl Dictionary {
    pub fn new(index_path: &Path, dict_path: &Path) -> Result<Self> {
        let index = CacheManager::load_or_build_trie(index_path)?;
        let data_content = CacheManager::load_or_decompress_dict(dict_path)?;

        Ok(Self {
            index,
            data_content,
        })
    }

    pub fn lookup(&self, query: &str) -> Vec<DictEntry> {
        if query.is_empty() {
            return Vec::new();
        }

        let matches = self.index.search_prefix(query, MAX_RESULTS);

        matches
            .into_iter()
            .filter_map(|(headword, offset, length)| {
                self.extract_definition(offset, length)
                    .map(|definition| DictEntry {
                        headword,
                        definition,
                    })
            })
            .collect()
    }

    fn extract_definition(&self, offset: u64, length: u64) -> Option<String> {
        let start = offset as usize;
        let end = start.checked_add(length as usize)?;

        if end <= self.data_content.len() {
            self.data_content
                .get(start..end)
                .map(|s| s.trim().to_string())
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn entry_count(&self) -> usize {
        self.index.len()
    }

    #[allow(dead_code)]
    pub fn data_size(&self) -> usize {
        self.data_content.len()
    }
}
