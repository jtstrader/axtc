# AXTC Themes

This directory contains all supported AXTC themes. These themes are essentially snapshots of `~/.config`. Rather than using a color file and modifying the various config files in place, which could be cumbersome, we instead just take snapshots of the specific config and overwrite the files entirely when loading a new one. Post-installation, these themes exist in `~/.config/axtc/themes/`.

In order to preserve potentially an unsaved theme, themes are signed with a key in `~/.config/axtc/.axtc-theme-key`. If a user attempts to load a theme without a properly defined key, then AXTC will store the current theme in `~/.config/axtc/themes/.recover/`. You can load from these themes if needed, or save them to a specific profile. Once saved, or if you request to delete, the theme will be removed from the recovery list.
