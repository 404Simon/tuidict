use super::state::AppState;

impl AppState {
    pub fn perform_search(&mut self) {
        if self.loaded_dictionaries.is_empty() {
            self.results = Vec::new();
            return;
        }

        let active_configs: Vec<_> = self.config.get_active_dictionaries();
        if active_configs.is_empty() {
            self.results = Vec::new();
            return;
        }

        if self.active_dict_index >= active_configs.len() {
            self.active_dict_index = 0;
        }

        if let Some(dict_config) = active_configs.get(self.active_dict_index) {
            if let Some(dict) = self.loaded_dictionaries.get(&dict_config.id) {
                self.results = dict.lookup(&self.input);
                self.selected_index = 0;
            }
        }
    }

    pub fn cycle_dictionary(&mut self) {
        let active_count = self.config.get_active_dictionaries().len();
        if active_count > 0 {
            self.active_dict_index = (self.active_dict_index + 1) % active_count;
            self.perform_search();
        }
    }

    pub fn next_result(&mut self) {
        if !self.results.is_empty() && self.selected_index < self.results.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous_result(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn get_active_dict_name(&self) -> String {
        let active_configs: Vec<_> = self.config.get_active_dictionaries();
        if let Some(dict) = active_configs.get(self.active_dict_index) {
            format!("{} -> {}", dict.from_lang, dict.to_lang)
        } else {
            "No active dictionary".to_string()
        }
    }

    pub fn get_filtered_dicts<'a>(
        &self,
        dicts: &'a [crate::download::FreeDictEntry],
    ) -> Vec<&'a crate::download::FreeDictEntry> {
        if self.download_filter.is_empty() {
            dicts.iter().collect()
        } else {
            let filter_lower = self.download_filter.to_lowercase();
            dicts
                .iter()
                .filter(|d| d.name.to_lowercase().contains(&filter_lower))
                .collect()
        }
    }
}
