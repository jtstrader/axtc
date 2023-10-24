#!/bin/bash

HERBSTLUFT_SRC="$HOME/.config/herbstluftwm/autostart"
HERBSTLUFT_DST="./backup/herbstluftwm/autostart"

POLYBAR_SRC="$HOME/.config/polybar/config.ini"
POLYBAR_DST="./backup/polybar/config.ini"

ALACRITTY_SRC="$HOME/.config/alacritty/alacritty.yml"
ALACRITTY_DST="./backup/alacritty/alacritty.yml"

backup() {
    # Make backup directories
    mkdir -p \
        ./backup/herbstluftwm \
        ./backup/polybar \
        ./backup/alacritty \
        ./backup/nvim

    # Copy files
    cp $HERBSTLUFT_SRC $HERBSTLUFT_DST || err_on_cpy
    cp $POLYBAR_SRC $POLYBAR_DST || err_on_cpy
    cp $ALACRITTY_SRC $ALACRITTY_DST || err_on_cpy
}

restore() {
    # Copy files
    cp $HERBSTLUFT_DST $HERBSTLUFT_SRC || err_on_cpy
    cp $POLYBAR_DST $POLYBAR_SRC || err_on_cpy
    cp $ALACRITTY_DST $ALACRITTY_SRC || err_on_cpy
}

err_on_cpy() {
    echo "axtc-backup: failed to copy"
    exit 1
}

if [[ $# == 0 ]]; then
    echo "axtc-backup: no arg provided"
    exit 1 
fi

if [[ $1 == "--save" ]]; then
    backup
elif [[ $1 == "--restore" ]]; then
    restore 
else
    echo "axtc-backup: bad argument '$1'"
    exit 1
fi
