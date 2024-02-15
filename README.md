# Wallpapercl
Wallpaper manager application made with tauri.

# Development
The commands you need to run (on Arch linux):
```bash
sudo pacman -S --needed webkit2gtk base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
git clone https://github.com/diegorezm/wallpapercl
cd wallpapercl && npm i && npm run tauri dev
```
Another requirement would be the script [changer](https://github.com/diegorezm/dotfiles-d/blob/master/.local/bin/changer) that i wrote, although
i plan on going for a Rust alternative eventually.
If you want to use this, the changer script must be under ```/home/$USER/.local/bin/scripts/changer```.

![print](./print.png)
