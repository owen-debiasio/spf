# Setup dev environment for spf

Some stuff to help with development

## Target(s)

> [!NOTE]  
> You'll need to install the needed linkers. See
> [.cargo/config.toml](.cargo/config.toml) if you are using an `x86_64` system

- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu

## Linters

- `Clippy` For linting Rust code
- `rustfmt` / `cargo fmt` For formatting Rust code

- `Prettier` for formatting Markdown files
- `markdownlint-cli2` for linting markdown files

- `shfmt` for formatting shell files
- `shellcheck` for linting shell files

## Cargo packages

- `cargo-deb` for packaging .deb file
- `cargo-generate-rpm` for packaging .rpm file

## Distrobox

If you are using an immutable distro, you can use
[Distrobox](https://distrobox.it/) to run the development environment.

### Setup

spf already includes [distrobox.ini](distrobox.ini), so to set up the container,
run:

```bash
distrobox assemble create
```

### Notes

- The default shell is `/bin/bash`
- When using Zed, distrobox is opened automatically on integrated terminal
  startup
- Rustup is installed automatically from [rustup.rs](rustup.rs)
- The container is based on the latest Arch Linux release (`archlinux:latest`)
