use qp_trie::Trie;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct PrefixTrie {
    trie: Trie<Vec<u8>, (u64, u64)>,
}

impl Serialize for PrefixTrie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<(Vec<u8>, (u64, u64))> =
            self.trie.iter().map(|(k, v)| (k.to_vec(), *v)).collect();
        entries.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrefixTrie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries: Vec<(Vec<u8>, (u64, u64))> = Vec::deserialize(deserializer)?;
        let mut trie = Trie::new();
        for (key, value) in entries {
            trie.insert(key, value);
        }
        Ok(PrefixTrie { trie })
    }
}

impl PrefixTrie {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }

    pub fn insert(&mut self, word: &str, offset: u64, length: u64) {
        let key = word.to_lowercase().into_bytes();
        self.trie.insert(key, (offset, length));
    }

    pub fn search_prefix(&self, prefix: &str, limit: usize) -> Vec<(String, u64, u64)> {
        if prefix.is_empty() {
            return Vec::new();
        }

        let prefix_lower = prefix.to_lowercase();
        let prefix_bytes = prefix_lower.as_bytes();

        let mut results: Vec<(String, u64, u64)> = self
            .trie
            .iter_prefix(prefix_bytes)
            .map(|(key, &(offset, length))| {
                let word = String::from_utf8_lossy(key).into_owned();
                (word, offset, length)
            })
            .collect();

        results.sort_by(|a, b| {
            let a_exact = a.0 == prefix_lower;
            let b_exact = b.0 == prefix_lower;

            if a_exact && !b_exact {
                return std::cmp::Ordering::Less;
            }
            if !a_exact && b_exact {
                return std::cmp::Ordering::Greater;
            }

            match a.0.len().cmp(&b.0.len()) {
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            }
        });

        results.truncate(limit);
        results
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.trie.count()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }
}

impl Default for PrefixTrie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insert_and_search() {
        let mut trie = PrefixTrie::new();
        trie.insert("hello", 0, 10);
        trie.insert("help", 10, 8);
        trie.insert("hero", 18, 12);

        let results = trie.search_prefix("hel", 10);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "help");
        assert_eq!(results[1].0, "hello");
    }

    #[test]
    fn test_case_insensitive() {
        let mut trie = PrefixTrie::new();
        trie.insert("Hello", 0, 10);

        let results = trie.search_prefix("hel", 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "hello");
    }

    #[test]
    fn test_exact_match_priority() {
        let mut trie = PrefixTrie::new();
        trie.insert("test", 0, 10);
        trie.insert("testing", 10, 15);
        trie.insert("testament", 25, 20);

        let results = trie.search_prefix("test", 10);
        assert_eq!(results[0].0, "test"); // Exact match first
    }

    #[test]
    fn test_limit() {
        let mut trie = PrefixTrie::new();
        trie.insert("apple", 0, 10);
        trie.insert("application", 10, 15);
        trie.insert("apply", 25, 8);

        let results = trie.search_prefix("app", 2);
        assert_eq!(results.len(), 2);
    }
}
