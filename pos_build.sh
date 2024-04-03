#!/bin/bash 
## doing this because th app image does not build automatically for some reason
WORKDIR=./src-tauri/target/release/bundle/appimage/wallpapercl.AppDir/
cp -i $WORKDIR/usr/bin/wallpapercl $HOME/.local/bin
appimagetool $WORKDIR
