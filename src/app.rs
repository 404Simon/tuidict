use crate::dictionary::{DictEntry, Dictionary};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::path::Path;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub results: Vec<DictEntry>,
    pub selected_index: usize,
    pub dict_de_en: Dictionary,
    pub dict_en_de: Dictionary,
    pub active_dict_is_de_en: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let de_en = Dictionary::new(
            Path::new("assets/deu-eng/deu-eng.index"),
            Path::new("assets/deu-eng/deu-eng.dict.dz"),
        )?;

        let en_de = Dictionary::new(
            Path::new("assets/eng-deu/eng-deu.index"),
            Path::new("assets/eng-deu/eng-deu.dict.dz"),
        )?;

        Ok(Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            results: Vec::new(),
            selected_index: 0,
            dict_de_en: de_en,
            dict_en_de: en_de,
            active_dict_is_de_en: true,
            exit: false,
        })
    }

    pub fn handle_event(&mut self) -> anyhow::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit = true
                    }
                    KeyCode::Char('j') => self.next_result(),
                    KeyCode::Char('k') => self.previous_result(),
                    KeyCode::Char('/') => {
                        self.input.clear();
                        self.perform_search();
                        self.input_mode = InputMode::Editing;
                    }
                    KeyCode::Esc => self.input_mode = InputMode::Editing,
                    KeyCode::Tab => self.toggle_dictionary(),
                    KeyCode::Down => self.next_result(),
                    KeyCode::Up => self.previous_result(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => self.input_mode = InputMode::Normal,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.exit = true
                    }
                    KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.next_result()
                    }
                    KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.previous_result()
                    }
                    KeyCode::Char(c) => {
                        self.input.push(c);
                        self.perform_search();
                    }
                    KeyCode::Backspace => {
                        self.input.pop();
                        self.perform_search();
                    }
                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                    KeyCode::Tab => self.toggle_dictionary(),
                    KeyCode::Down => self.next_result(),
                    KeyCode::Up => self.previous_result(),
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn perform_search(&mut self) {
        let active_dict = if self.active_dict_is_de_en {
            &self.dict_de_en
        } else {
            &self.dict_en_de
        };
        self.results = active_dict.lookup(&self.input);
        self.selected_index = 0;
    }

    fn toggle_dictionary(&mut self) {
        self.active_dict_is_de_en = !self.active_dict_is_de_en;
        self.perform_search(); // Re-search with new dict
    }

    fn next_result(&mut self) {
        if !self.results.is_empty() && self.selected_index < self.results.len() - 1 {
            self.selected_index += 1;
        }
    }

    fn previous_result(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
