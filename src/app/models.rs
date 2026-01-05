#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Translation,
    Management,
    Download,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}
