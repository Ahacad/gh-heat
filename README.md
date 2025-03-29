# gh-heat 0.1.0

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

### For Arch Linux Users (via AUR)

```bash
yay -S gh-heat
# or
paru -S gh-heat
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

## Development

### Automated Release Process

This project uses:
- `cargo-release` to manage version bumping and tag creation
- GitHub Actions to automatically build binaries for multiple platforms
- Automated deployment to the Arch User Repository (AUR)

To create a new release:

```bash
./release.sh 0.1.1  # Replace with the new version number
```

This will:
1. Update version numbers in Cargo.toml and README.md
2. Create a Git commit and tag
3. Push to GitHub, triggering automated builds
4. Deploy the new version to AUR automatically

### Setting Up AUR Deployment

For AUR deployment to work, you need to add these GitHub Secrets:

- `AUR_USERNAME`: Your AUR username
- `AUR_EMAIL`: Email associated with your AUR account
- `AUR_SSH_PRIVATE_KEY`: SSH private key for AUR access

## License

MIT
