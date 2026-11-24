# Install spf

Below are instructions to install spf.

## Packages and Binaries

> [!IMPORTANT]  
> Right now, binaries and packages for spf are built for `x86_64` and `aarch64`
> systems. Look at [Building from source](#build-from-source) if your system is
> not currently supported.

You can find packages and binaries over at
[the releases page](https://github.com/owen-debiasio/spf/releases/latest).

## Build from source

You can build from source if you wish, like if you want to use the latest git
build

You have two options to do this. You can either:

- [Use the builder script](#build-script) to generate an installable package
- [Use the installer script](#install-script) the package as a `.spf` package
- [Install the binary manually](#standalone-binary)

### Initializing

No matter which choice you make, clone and enter the repository first:

```bash
git clone --depth=1 https://github.com/owen-debiasio/spf.git
cd spf
```

### Build Script

This is a very powerful and hopefully user-friendly script that can help you
effortlessly build packages neccesary for your system.

> [!NOTE]  
> You can find the build script here:
>
> [./packages/build.py](./packages/build.py)

#### Dependencies

You need the following to build particular packages:

- Cargo
- `cargo-generate-rpm` for `.rpm` packages
- `cargo-deb` for `.deb` packages

For `.spf` packages and the binary, spf is already built, so no need for
external dependencies.

You can see how to install individual types below:

#### Debian/Ubuntu

```bash
./packages/build.py --version=auto --type=deb --arch=auto
sudo dpkg -i ./packages/artifacts/spf-v*
```

#### Fedora

```bash
./packages/build.py --version=auto --type=rpm --arch=auto
sudo rpm --install ./packages/artifacts/spf-v*
```

#### spf

```bash
./packages/build.py --version=auto --type=spf --arch=auto
sudo ./target/debug/spf install ./packages/artifacts/spf-v*
```

#### Binary

```bash
./packages/build.py --version=auto --type=bin --arch=auto
sudo mkdir -p /usr/local/bin
sudo cp ./packages/artifacts/spf-v* /usr/local/bin/spf
```

### Install Script

> [!NOTE]  
> You can find the installer script here:
>
> [./dev/install.bash](./dev/install.bash)

Use this step when you want spf installed as a spf package.

#### Dependencies

You only need Cargo. Nothing else.

#### Build and Install

```bash
./dev/install.bash
```

### Standalone Binary

This is when you just want the binary, and nothing else.

#### Dependencies

You only need Cargo. Nothing else.

#### Build

```bash
cargo build --release
sudo mkdir -p /usr/local/bin
sudo cp /target/release/spf /usr/local/bin/spf
```

---

Last Updated: `November 23, 2026 @ 08:39 PM EST`
