use super::models::{InputMode, Page};
use crate::config::{Config, DictConfig};
use crate::dictionary::{DictEntry, Dictionary};
use crate::download::FreeDictEntry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DownloadState {
    pub progress: (u64, u64),
    pub result: Option<Result<(String, PathBuf), String>>,
}

pub struct AppState {
    pub page: Page,
    pub exit: bool,

    // dictionary data
    pub config: Config,
    pub loaded_dictionaries: HashMap<String, Dictionary>,

    // translation page
    pub input: String,
    pub input_mode: InputMode,
    pub results: Vec<DictEntry>,
    pub selected_index: usize,
    pub active_dict_index: usize,

    // management page
    pub management_selected: usize,

    // download page state
    pub available_dicts: Option<Vec<FreeDictEntry>>,
    pub download_selected: usize,
    pub download_filter: String,
    pub download_input_mode: InputMode,
    pub download_status: Option<String>,
    pub loading_dicts: bool,
    pub download_progress: Option<(u64, u64)>,
    pub(super) download_state: Option<Arc<Mutex<DownloadState>>>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::load()?;

        let mut loaded_dictionaries = HashMap::new();
        let mut failed_dict_ids = Vec::new();

        for dict_config in config.get_active_dictionaries() {
            match load_dictionary(dict_config) {
                Ok(dict) => {
                    loaded_dictionaries.insert(dict_config.id.clone(), dict);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to load dictionary {}: {}",
                        dict_config.name, e
                    );
                    failed_dict_ids.push(dict_config.id.clone());
                }
            }
        }

        // deactivate failed dictionaries
        for dict_id in failed_dict_ids {
            if let Some(d) = config.dictionaries.iter_mut().find(|d| d.id == dict_id) {
                d.active = false;
            }
        }

        let _ = config.save();

        Ok(Self {
            page: Page::Translation,
            exit: false,
            input: String::new(),
            input_mode: InputMode::Editing,
            results: Vec::new(),
            selected_index: 0,
            active_dict_index: 0,
            config,
            loaded_dictionaries,
            management_selected: 0,
            available_dicts: None,
            download_selected: 0,
            download_filter: String::new(),
            download_input_mode: InputMode::Editing,
            download_status: None,
            loading_dicts: false,
            download_progress: None,
            download_state: None,
        })
    }
}

pub fn load_dictionary(dict_config: &DictConfig) -> anyhow::Result<Dictionary> {
    let index_path = dict_config.path.join(format!("{}.index", dict_config.id));
    let dict_path = dict_config.path.join(format!("{}.dict.dz", dict_config.id));

    Dictionary::new(&index_path, &dict_path)
}
