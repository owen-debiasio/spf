# spf - Simple package format

spf is my small side project where I aim to create a packaging format similar to
`.deb` or `.rpm` packages.

![spf logo](assets/logo.png "spf logo")

![Badge Commits](https://img.shields.io/github/commit-activity/m/owen-debiasio/spf?label=Commits)
![Badge License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Badge Issues](https://img.shields.io/github/issues/owen-debiasio/spf)

[Codeberg Mirror](https://codeberg.org/owen-debiasio/spf)

## How to use

```none
create     <metadata file> <output directory>    Create package
install    <.spf package location>               Install package
remove     <package to uninstall>                Uninstall package
list       <(optional) string to match>          List installed packages
template   <(optional) output location>          Generate package metadata template
inspect    <package to inspect>                  Inspect metadata of a package
convert    <source package> <converted format>   Convert package formats
```

## Install

> [!IMPORTANT]  
> Right now, binaries and packages for spf are built for `x86_64` and `aarch64`
> systems. Look at [Building from source](#build-from-source)

### Debian/Ubuntu

You can find the `.deb` package here:
[Releases Page](https://github.com/owen-debiasio/spf/releases/latest)

### Fedora/SUSE

You can find the `.rpm` package here:
[Releases Page](https://github.com/owen-debiasio/spf/releases/latest)

### Universal

Here are ways that you can install spf on any distro

#### Standalone Binary

You can find the standalone binary here:
[Releases Page](https://github.com/owen-debiasio/spf/releases/latest)

#### .spf Installer package

You can find the `.spf` package here:
[Releases Page](https://github.com/owen-debiasio/spf/releases/latest)

#### Build from source

You can build from source if you wish, like if you want to use the latest git
build

##### Install Dependencies

- Rust (Cargo, Rustc)
- Git

###### Arch

```bash
sudo pacman -S --needed cargo git
```

###### Debian/Debian-based

```bash
sudo apt install git
```

> [!NOTE]  
> Install Rust from [rustup.rs](rustup.rs)

###### Fedora/SUSE-based

```bash
sudo dnf install git cargo rust
```

---

##### Building

Simply run:

```bash
git clone --depth=1 https://github.com/owen-debiasio/spf.git # Clone repo
cd spf # Navigate into cloned repo
cargo build --release # Build spf
sudo cp target/release/spf /usr/bin/spf # Install to location (like /usr/local/bin/spf)
# Optional: Clean up
# cd .. && rm -rf ./spf
```

## Disclaimers/Non-goals

- spf is not intended to be a common tool to use for everyone, it's only a
  project I've whipped up out of boredom. Please don't take this too seriously.
- The code in this project is either human written, or human verified. If any
  LLM is used, it's reviewed and verified by me. If I don't understand the code,
  I don't implement it. Even then, very little AI is used in this project. I
  take full responsibility of all code in spf.

## Credits

- [Glob](https://github.com/rust-lang/glob) by
  [@rust-lang](https://github.com/rust-lang)
- [self-replace](https://github.com/mitsuhiko/self-replace) by
  [@mitsuhiko](https://github.com/djc)
- [tar-rs](https://github.com/composefs/tar-rs) by
  [@composefs](https://github.com/composefs)
- [semver](https://github.com/dtolnay/semver) by
  [@dtolnay](https://github.com/dtolnay)
