mod api;
mod installer;

pub use api::{fetch_available_dictionaries, FreeDictEntry};
pub use installer::{download_and_install, find_dict_files};
