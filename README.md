# Wallpapercl

Wallpaper manager application made with tauri.

# Install

- Install dedependencies

```bash
sudo pacman -S feh
```

- Download from the releases page
  [diegorezm/wallpapercl](https://github.com/diegorezm/wallpapercl/releases)

# Development

The commands you need to run (on Arch linux):

```bash
sudo pacman -Syu
sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips \
    feh
git clone https://github.com/diegorezm/wallpapercl
cd wallpapercl && npm i && npm run tauri dev
```

![print](./showcase.png)
