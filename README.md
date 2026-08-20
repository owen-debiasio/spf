# spf

Simple package format

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

Currently the only ways to install spf are:

- The following in the [Releases Page](https://github.com/owen-debiasio/spf/releases/latest)
  - Standalone binary (**x86_64 only** at the moment)
  - .spf package
- Building from source
  - Requires linux (unless if developing), rustup/cargo

## Disclaimers/Non-goals

- spf is not intended to be a common tool to use for everyone, it's only a
  project I've whipped up out of boredom. Please don't take this too
  seriously.
- This project is 100% human-written code. Either by me or from some forum or 10
  year old Stack Overflow post.
  
  ## Credits

- [Glob](https://github.com/rust-lang/glob) by rust-lang
