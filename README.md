# gh-heat

A terminal GitHub contribution heatmap generator written in Rust. View anyone's GitHub contribution activity right in your terminal!

## Features

- Generate GitHub-style contribution heatmaps for any user
- Customize the look with different color schemes (light/dark)
- Display using symbols instead of colors for terminals with limited color support
- Show numeric contribution counts
- View contribution statistics

## Installation

### Using Cargo

```bash
cargo install gh-heat
```

### Building from source

```bash
git clone https://github.com/yourusername/gh-heat.git
cd gh-heat
cargo build --release
```

The executable will be located at `target/release/gh-heat`.

## Usage

```bash
# Basic usage - show heatmap for a GitHub user
gh-heat username

# Show a full year of contributions with dark theme
gh-heat username --dark-mode

# Show contributions using symbols instead of colors
gh-heat username --symbols

# Show numeric contribution counts
gh-heat username --numbers

# Show statistics about contributions
gh-heat username --totals

# Show only the last 30 days of contributions
gh-heat username --days 30
```

## GitHub Authentication

The tool works for public GitHub profiles without authentication. For private repositories or to avoid rate limits, set your GitHub token as an environment variable:

```bash
export GITHUB_TOKEN=your_token_here
```

## Requirements

- Rust 1.56 or later
- A terminal with RGB color support (for color mode)

## License

MIT
