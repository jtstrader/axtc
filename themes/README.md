# Themes

Each file in this directory (except `template.toml`) is an axtc theme — a TOML file that
describes the visual appearance of your desktop.

## Creating a theme

Copy `template.toml` and fill in the values:

```sh
axtc new mytheme
```

This copies `template.toml` to `mytheme.toml` in your themes directory. Open it and customize.

## Applying a theme

```sh
axtc apply mytheme
```

axtc renders each app's config template with your theme values and writes the result to the
app's config location. Your previous config is automatically backed up to
`~/.config/axtc/backups/<app>/`.

## Listing themes

```sh
axtc list
```

## Schema reference

### Metadata

| Key           | Type   | Description                     |
|---------------|--------|---------------------------------|
| `name`        | String | Required. Identifier for the theme. |
| `description` | String | Optional. Human-readable description. |

---

### `[ansi.primary]` — Terminal foreground/background

| Key          | Type   | Example       | Description          |
|--------------|--------|---------------|----------------------|
| `foreground` | String | `"#e0e0e0"`   | Default text color   |
| `background` | String | `"#000000"`   | Terminal background  |
| `cursor`     | String | `"#ffffff"`   | Cursor color         |

---

### `[ansi.normal]` and `[ansi.bright]` — 8 ANSI colors

Both tables accept the same keys:

| Key       | ANSI index (normal / bright) |
|-----------|------------------------------|
| `black`   | 0 / 8                        |
| `red`     | 1 / 9                        |
| `green`   | 2 / 10                       |
| `yellow`  | 3 / 11                       |
| `blue`    | 4 / 12                       |
| `magenta` | 5 / 13                       |
| `cyan`    | 6 / 14                       |
| `white`   | 7 / 15                       |

---

### `[herbstluftwm]` — Window manager

| Key              | Type    | Default | Description                                          |
|------------------|---------|---------|------------------------------------------------------|
| `borders`        | Boolean | `true`  | Whether to render window/frame borders               |
| `transparency`   | Boolean | `false` | Enable window transparency via picom                 |
| `background_src` | String  | —       | Path to wallpaper image (passed to `nitrogen`)       |

---

### `[polybar]` — Status bar

| Key        | Type    | Default  | Description                              |
|------------|---------|----------|------------------------------------------|
| `position` | String  | `"top"`  | Bar position: `"top"` or `"bottom"`      |
| `height`   | Integer | `27`     | Bar height in pixels                     |
| `font`     | String  | —        | Font string in polybar format            |

---

### `[alacritty]` — Terminal emulator

| Key         | Type    | Default | Description                              |
|-------------|---------|---------|------------------------------------------|
| `font`      | String  | —       | Font family name                         |
| `font_size` | Float   | `12.0`  | Font size in points                      |
| `opacity`   | Float   | `1.0`   | Window opacity (0.0–1.0)                 |

---

### `[picom]` — Compositor

| Key             | Type    | Default | Description                              |
|-----------------|---------|---------|------------------------------------------|
| `transparency`  | Boolean | `false` | Enable per-window transparency           |
| `blur`          | Boolean | `false` | Enable background blur                   |
| `corner_radius` | Integer | `0`     | Window corner radius in pixels           |

---

## Template system

axtc uses [Tera](https://keats.github.io/tera/) (Jinja2-like) templates located in
`~/.config/axtc/templates/<app>/`. Variables map directly to the TOML keys:

```
{{ ansi.primary.foreground }}
{{ ansi.normal.blue }}
{% if herbstluftwm.borders %}...{% endif %}
{{ polybar.height | default(value=27) }}
```

See `../templates/` for the default templates for each supported app.
