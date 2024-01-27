# wallpaper-randomizer

Small utility for randomly selecting a Gnome desktop wallpaper

## Build

```bash
cargo install --path .
```

## Run

```bash
Usage: wallpaper-randomizer --dir <DIR>

Options:
  -d, --dir <DIR>  
  -h, --help       Print help
  -V, --version    Print version
```

## Timer

This application can be run using systemd. Once you have built the executable, in your user systemd directory (check your distro docs on where this is) create a service file e.g. `$HOME/.local/share/systemd/user/wallpaper_randomizer.service`:

```bash
[Unit]
Description=Wallpaper randomizer service
After=network.target

[Service]
Type=oneshot
ExecStart=/home/USER/.cargo/bin/wallpaper-randomizer -d /home/USER/Pictures/Wallpapers

[Install]
WantedBy=default.target
```

Then add a timer file in the same directory e.g. `$HOME/.local/share/systemd/user/wallpaper_randomizer.timer`:

```
[Unit]
Description=Run Wallpaper randomizer every 5 minutes

[Timer]
OnBootSec=3min
OnUnitActiveSec=5min

[Install]
WantedBy=timers.target
```

Enable the service:

```bash
systemctl --user enable wallpaper_randomizer.service
systemctl --user enable --now wallpaper_randomizer.timer
```
