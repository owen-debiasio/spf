# spf - Simple package format

spf is my small side project where I aim to create a packaging format similar to
`.deb` or `.rpm` packages.

![spf logo](assets/logo.png "spf logo")

![Badge Commits](https://img.shields.io/github/commit-activity/m/owen-debiasio/spf?label=Commits)
![Badge License](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Badge Issues](https://img.shields.io/github/issues/owen-debiasio/spf)

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

##### Build prerequisites

- Rust (Cargo, Rustup, Rustc)
  - I recommend using the most recent stable release
- Git

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
- This project is 100% human-written code. Either by me or from some forum or 10
  year old Stack Overflow post.

## Credits

- [Glob](https://github.com/rust-lang/glob) and
  [flate2-rs](https://github.com/rust-lang/flate2-rs) by
  [rust-lang](https://github.com/rust-lang)
- [self-replace](https://github.com/mitsuhiko/self-replace) by
  [mitsuhiko](https://github.com/mitsuhiko)
- [file-diff](https://github.com/ethanpailes/file_diff-rs) by
  [ethanpailes](https://github.com/ethanpailes)
- [tar-rs](https://github.com/composefs/tar-rs) by
  [composefs](https://github.com/composefs)
- [xz2-rs](https://github.com/alexcrichton/xz2-rs) by
  [Alex Crichton](https://github.com/alexcrichton)
- [rust-ar](https://github.com/mdsteele/rust-ar) by
  [mdsteele](https://github.com/mdsteele)
- [deb-rust](https://codeberg.org/notsludgebomb/deb-rust) by
  [NotSludgeBomb](https://codeberg.org/NotSludgeBomb)
