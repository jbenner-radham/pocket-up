PocketUp
========
A Linux GUI app to download and update openFPGA cores on the Analogue Pocket.

<div align="center">
    <img src="assets/pocket-up.png" alt="PocketUp">
</div>

Build
-----
Prerequisites for Debian/Ubuntu:

```shell
sudo apt install libgtk-4-dev build-essential libadwaita-1-dev
```

Prerequisites for Fedora:

```shell
sudo dnf install gtk4-devel gcc libadwaita-devel
```

Prerequisites for Arch:

```shell
sudo pacman -S gtk4 base-devel libadwaita
```

Then to build:

```shell
cargo build --release
```

Package
-------
### Debian/Ubuntu
_Currently tested on Ubuntu 22.04 LTS._

Install `cargo-deb`:

```shell
rustup update
cargo install cargo-deb
```

Run `cargo-deb`:

```shell
cargo deb
```

This will produce a file in the form of `target/debian/pocket-up_<version>_<arch>.deb`.

Install
-------
```shell
# Install not yet available.
```

License
-------
The MIT License. See the [license file](LICENSE) for details.
