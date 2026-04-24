# axtc

**Arch/X theme changer** — a CLI tool for applying themes to X11 systems.

`axtc` reads a theme defined in a single TOML file and renders config files for each supported application via [Tera](https://keats.github.io/tera/) templates, backing up any existing configs before overwriting them.

## Supported applications

| Application | Config written |
|---|---|
| [Alacritty](https://alacritty.org/) | `~/.config/alacritty/alacritty.toml` |
| [herbstluftwm](https://herbstluftwm.org/) | `~/.config/herbstluftwm/autostart` |
| [picom](https://github.com/yshui/picom) | `~/.config/picom/picom.conf` |
| [polybar](https://polybar.github.io/) | `~/.config/polybar/config.ini` |

## Usage

```sh
# Apply a theme by name
axtc apply <theme>

# List available themes
axtc list

# Create a new theme from the base template
axtc new <name>
```

Themes live in `~/.config/axtc/themes/`. Each theme is a TOML file — see `themes/template.toml` in this repo for the full reference with all available fields and documentation.

## Building

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for task automation.

### Common tasks

| Command | Description |
|---|---|
| `cargo make` | Run the full dev flow (build → lint → test → doc) |
| `cargo make build` | Debug build |
| `cargo make build-release` | Release build |
| `cargo make check` | Check for errors without producing a binary |
| `cargo make lint` | Run Clippy |
| `cargo make test` | Run all tests |
| `cargo make doc` | Build documentation |

