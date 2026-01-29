# tuidict

TUI dictionary with support for multiple language pairs from FreeDict.

## Features

- Multi-language support - Download and use any dictionary from the FreeDict database
- Live search-as-you-type with instant results
- Fast prefix search using Trie data structure (O(k) lookups)
- Binary caching for instant startup times
- In-app dictionary downloads and management

## Quick Start

```bash
cargo install tuidict
```

1. Launch the application
2. Press `3` to go to the Download page
3. Browse available dictionaries and press `Enter` to download
4. Press `1` to return to the Translation page
5. Start typing to search!

## Keybindings

### Global
- `1` - Translation page
- `2` - Dictionary management page
- `3` - Download dictionaries page
- `q` or `Ctrl+C` - Quit

### Translation Page (Page 1)
- Type to search (live results)
- `Tab` - Cycle through active dictionaries
- `j/k` or `↑/↓` - Navigate results
- `Ctrl+n/Ctrl+p` - Navigate results while editing
- `Enter` - Enter normal mode
- `Esc` - Go back to editing mode
- `/` - Clear input and go to editing mode

### Dictionary Management (Page 2)
- `j/k` or `↑/↓` - Navigate dictionary list
- `Space` or `Enter` - Toggle dictionary active/inactive
- `d` - Delete dictionary (removes files)

### Download Page (Page 3)
- Type to filter dictionaries (live search)
- `Ctrl+n/Ctrl+p` - Navigate while filtering
- `j/k` or `↑/↓` - Navigate dictionary list
- `Esc` - Enter normal mode (stops editing filter)
- `Enter` - Download and install selected dictionary
- `/` - Clear filter and enter editing mode

## Storage

- Configuration: `~/.config/tuidict/config.json`
- Dictionaries: `~/.local/share/tuidict/dictionaries/`
- Cache files: Stored alongside dictionary files for fast loading

## Dictionary Source

Dictionaries are sourced from [FreeDict](https://freedict.org/), a free and open-source dictionary project supporting numerous language pairs.
