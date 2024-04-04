#!/bin/bash 
## doing this because th app image does not build automatically for some reason
WORKDIR=./src-tauri/target/release/bundle/appimage/wallpapercl.AppDir/
echo "tauri build..."
npm run tauri build
echo "Moving binary to local/bin..." 
cp -i $WORKDIR/usr/bin/wallpapercl $HOME/.local/bin 
echo "Creating appimage..." 
appimagetool $WORKDIR
