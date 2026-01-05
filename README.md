# tuidict

Bidirectional German-English TUI dictionary.
Tools uses FreeDict data (not included in this Repo).

## Features

- Bidirectional German - English translation
- live search-as-you-type
- prefix search using Trie data structure (O(k) lookups)
- binary caching for instant startup times

## Usage

```bash
./tuidict
```

**Keybindings:**
- Type to search (live results)
- `Tab` - Switch between DE -> EN and EN -> DE
- `j/k` - Navigate results in Normal Mode
- `Ctrl+p/Ctrl+n` or `↑/↓` - Navigate results whilest in editing mode
- `Enter` - Enter normal mode
- `Esc` - go back to editing mode
- `/` - clear input and go to editing mode
- `q` or `Ctrl+C` - Quit

