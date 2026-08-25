# spf - Simple package format

spf is my small side project where I aim to create a packaging format similar to
`.deb` or `.rpm` packages.

![spf logo](assets/logo.png "spf logo")

## Prerequisites

- `tar` (GNU version) (Needed for creating packages)

## How to use

```none
create     <metadata file> <output directory>   Create package
install    <.spf package location>              Install package
remove     <package to uninstall>               Uninstall package
list       <(optional) string to match>         List installed packages
template   <(optional) output location>         Generate package metadata template
inspect    <package to inspect>                 Inspect metadata of a package
```

### Examples

#### Create package

> [!IMPORTANT]  
> The metadata config file must NOT have a file extension

`$ spf create config package.spf`

#### Install package

> [!WARNING]  
> Using spf along with other package managers could cause issues

`$ sudo spf create install package.spf`

#### Remove package

`$ sudo spf remove package`

#### List packages

> [!NOTE]  
> You could also just run `spf list` to list all packages

`$ spf list pac`

#### Generate template package config

> [!NOTE]  
> The output location must be to a directory.

`$ spf template ~/Downloads/`

#### Inspect metadata of .spf package

##### Installer package (`.spf` package)

`$ spf inspect installer_package.spf`

##### Already installed package

`$ spf inspect installed_package`

## Install

> [!IMPORTANT]  
> Right now, binaries and packages for spf are built for `x86_64` systems. Look
> at
> [Building from source](https://github.com/owen-debiasio/spf#build-from-source)

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

You can find the `.rpm` package here:
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

- [Glob](https://github.com/rust-lang/glob) by
  [rust-lang](https://github.com/rust-lang)
- [self-replace](https://github.com/mitsuhiko/self-replace) by
  [mitsuhiko](https://github.com/mitsuhiko)
