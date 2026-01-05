use super::models::{InputMode, Page};
use super::state::{load_dictionary, AppState, DownloadState};
use crate::config::{Config, DictConfig};
use crate::download::{download_and_install, fetch_available_dictionaries, find_dict_files};
use crossterm::event::{self, KeyCode, KeyModifiers};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

impl AppState {
    pub fn handle_translation_event(&mut self, key: event::KeyEvent) -> anyhow::Result<()> {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('j') => self.next_result(),
                KeyCode::Char('k') => self.previous_result(),
                KeyCode::Char('/') => {
                    self.input.clear();
                    self.perform_search();
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Esc => self.input_mode = InputMode::Editing,
                KeyCode::Tab => self.cycle_dictionary(),
                KeyCode::Down => self.next_result(),
                KeyCode::Up => self.previous_result(),
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => self.input_mode = InputMode::Normal,
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.next_result()
                }
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.previous_result()
                }
                KeyCode::Char(c) if !c.is_numeric() || c == '0' => {
                    self.input.push(c);
                    self.perform_search();
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    self.perform_search();
                }
                KeyCode::Esc => self.input_mode = InputMode::Normal,
                KeyCode::Tab => self.cycle_dictionary(),
                KeyCode::Down => self.next_result(),
                KeyCode::Up => self.previous_result(),
                _ => {}
            },
        }
        Ok(())
    }

    pub fn handle_management_event(&mut self, key: event::KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Esc => self.page = Page::Translation,
            KeyCode::Down | KeyCode::Char('j') => {
                if self.management_selected < self.config.dictionaries.len().saturating_sub(1) {
                    self.management_selected += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.management_selected > 0 {
                    self.management_selected -= 1;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.toggle_selected_dictionary()?;
            }
            KeyCode::Char('d') => {
                self.delete_selected_dictionary()?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_download_event(&mut self, key: event::KeyEvent) -> anyhow::Result<()> {
        match self.download_input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Esc => self.page = Page::Translation,
                KeyCode::Char('j') | KeyCode::Down => {
                    if let Some(dicts) = &self.available_dicts {
                        let filtered = self.get_filtered_dicts(dicts);
                        if self.download_selected < filtered.len().saturating_sub(1) {
                            self.download_selected += 1;
                        }
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.download_selected > 0 {
                        self.download_selected -= 1;
                    }
                }
                KeyCode::Char('/') => {
                    self.download_filter.clear();
                    self.download_selected = 0;
                    self.download_input_mode = InputMode::Editing;
                }
                KeyCode::Enter => {
                    self.download_selected_dictionary();
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.exit = true
                }
                KeyCode::Enter => self.download_input_mode = InputMode::Normal,
                KeyCode::Esc => {
                    self.download_input_mode = InputMode::Normal;
                }
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if let Some(dicts) = &self.available_dicts {
                        let filtered = self.get_filtered_dicts(dicts);
                        if self.download_selected < filtered.len().saturating_sub(1) {
                            self.download_selected += 1;
                        }
                    }
                }
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if self.download_selected > 0 {
                        self.download_selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(dicts) = &self.available_dicts {
                        let filtered = self.get_filtered_dicts(dicts);
                        if self.download_selected < filtered.len().saturating_sub(1) {
                            self.download_selected += 1;
                        }
                    }
                }
                KeyCode::Up => {
                    if self.download_selected > 0 {
                        self.download_selected -= 1;
                    }
                }
                KeyCode::Char(c) if !c.is_numeric() || c == '0' => {
                    self.download_filter.push(c);
                    self.download_selected = 0;
                }
                KeyCode::Backspace => {
                    self.download_filter.pop();
                    self.download_selected = 0;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn toggle_selected_dictionary(&mut self) -> anyhow::Result<()> {
        if let Some(dict) = self.config.dictionaries.get(self.management_selected) {
            let dict_id = dict.id.clone();
            let was_active = dict.active;

            self.config.toggle_dictionary(&dict_id);
            let _ = self.config.save();

            // Load or unload dictionary
            if was_active {
                self.loaded_dictionaries.remove(&dict_id);
            } else {
                if let Some(dict_config) = self.config.dictionaries.iter().find(|d| d.id == dict_id)
                {
                    match load_dictionary(dict_config) {
                        Ok(dict) => {
                            self.loaded_dictionaries.insert(dict_id.clone(), dict);
                        }
                        Err(e) => {
                            self.download_status =
                                Some(format!("Failed to load dictionary: {}", e));
                            // Deactivate it again
                            self.config.toggle_dictionary(&dict_id);
                            let _ = self.config.save();
                        }
                    }
                }
            }

            let active_count = self.loaded_dictionaries.len();
            if active_count > 0 && self.active_dict_index >= active_count {
                self.active_dict_index = active_count - 1;
            }

            self.perform_search();
        }
        Ok(())
    }

    fn delete_selected_dictionary(&mut self) -> anyhow::Result<()> {
        if let Some(dict) = self
            .config
            .dictionaries
            .get(self.management_selected)
            .cloned()
        {
            let dict_id = dict.id.clone();

            self.config.remove_dictionary(&dict_id);
            let _ = self.config.save();

            self.loaded_dictionaries.remove(&dict_id);

            let _ = std::fs::remove_dir_all(&dict.path);

            if self.management_selected >= self.config.dictionaries.len()
                && self.management_selected > 0
            {
                self.management_selected -= 1;
            }

            let active_count = self.loaded_dictionaries.len();
            if active_count > 0 && self.active_dict_index >= active_count {
                self.active_dict_index = active_count - 1;
            }

            self.perform_search();
        }
        Ok(())
    }

    pub fn fetch_available_dictionaries(&mut self) {
        self.loading_dicts = true;
        self.download_status = Some("Loading dictionaries...".to_string());

        match fetch_available_dictionaries() {
            Ok(dicts) => {
                self.available_dicts = Some(dicts);
                self.download_status = None;
            }
            Err(e) => {
                self.download_status = Some(format!("Failed to load dictionaries: {}", e));
            }
        }
        self.loading_dicts = false;
    }

    fn download_selected_dictionary(&mut self) {
        if self.download_state.is_some() {
            return;
        }

        let (entry, download_name) = {
            if let Some(dicts) = &self.available_dicts {
                let filtered = self.get_filtered_dicts(dicts);
                if let Some(entry) = filtered.get(self.download_selected).cloned() {
                    let name = entry.name.clone();
                    (Some(entry), name)
                } else {
                    (None, String::new())
                }
            } else {
                (None, String::new())
            }
        };

        let Some(entry) = entry else {
            return;
        };

        self.download_status = Some(format!("Downloading {}...", download_name));
        self.download_progress = Some((0, 1));

        let data_dir = match Config::data_dir() {
            Ok(dir) => dir,
            Err(e) => {
                self.download_status = Some(format!("Failed to get data directory: {}", e));
                return;
            }
        };

        self.download_state = start_download_thread(entry.clone(), data_dir);
    }

    pub fn check_download_progress(&mut self) {
        let state = match &self.download_state {
            Some(s) => s.clone(),
            None => return,
        };

        let (progress, result_opt) = {
            if let Ok(s) = state.lock() {
                (s.progress, s.result.clone())
            } else {
                return;
            }
        };

        self.download_progress = Some(progress);

        if let Some(result) = result_opt {
            self.download_state = None;
            self.download_progress = None;

            match result {
                Ok((dict_name, dict_dir)) => {
                    self.handle_download_success(dict_name, dict_dir);
                }
                Err(e) => {
                    self.download_status = Some(format!("Download failed: {}", e));
                }
            }
        }
    }

    fn handle_download_success(&mut self, dict_name: String, dict_dir: PathBuf) {
        match find_dict_files(&dict_dir) {
            Ok((index_path, _dict_path)) => {
                let dict_id = dict_name.clone();

                // parse languages from dict_name
                let parts: Vec<&str> = dict_name.split('-').collect();
                let (from_lang, to_lang) = if parts.len() >= 2 {
                    (parts[0].to_uppercase(), parts[1].to_uppercase())
                } else {
                    ("UNK".to_string(), "UNK".to_string())
                };

                let dict_base_dir = index_path.parent().unwrap().to_path_buf();

                let dict_config = DictConfig {
                    id: dict_id.clone(),
                    name: dict_name.clone(),
                    from_lang,
                    to_lang,
                    path: dict_base_dir,
                    active: true,
                };

                self.config.add_dictionary(dict_config.clone());
                if let Err(e) = self.config.save() {
                    self.download_status = Some(format!("Failed to save config: {}", e));
                } else {
                    match load_dictionary(&dict_config) {
                        Ok(dict) => {
                            self.loaded_dictionaries.insert(dict_id, dict);
                            self.download_status =
                                Some(format!("Successfully installed {}", dict_name));
                        }
                        Err(e) => {
                            self.download_status =
                                Some(format!("Downloaded but failed to load: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                self.download_status = Some(format!("Failed to find dictionary files: {}", e));
            }
        }
    }
}

fn start_download_thread(
    entry: crate::download::FreeDictEntry,
    data_dir: PathBuf,
) -> Option<Arc<Mutex<DownloadState>>> {
    let state = Arc::new(Mutex::new(DownloadState {
        progress: (0, 1),
        result: None,
    }));

    let state_for_thread = Arc::clone(&state);

    thread::spawn(move || {
        let state_clone = Arc::clone(&state_for_thread);
        let dict_name = entry.name.clone();

        let result = download_and_install(&entry, &data_dir, move |downloaded, total| {
            if let Ok(mut s) = state_clone.lock() {
                s.progress = (downloaded, total);
            }
        });

        if let Ok(mut s) = state_for_thread.lock() {
            s.result = Some(match result {
                Ok(dict_dir) => Ok((dict_name, dict_dir)),
                Err(e) => Err(e.to_string()),
            });
        }
    });

    Some(state)
}
