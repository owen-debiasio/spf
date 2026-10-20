# Converting To and From .spf files

The provided command `convert` allows you do convert `.spf` files to `.deb`
files, and vice versa.

The following is converted:

- Package formats
- Metadata

The way paths are copied remains the same for both directions.

## Conversion

Whether you are converting `.spf` to `.rpm`, or `.deb` -> `.rpm`, conversion
follows these steps:

1. [Path Evaluation](#path-evaluation)
2. [Initialization: Collect Package Information & Metadata](#initialization-collect-package-information--metadata)
3. [Data Conversion](#data-conversion)
4. [Build & Package](#build--package)

### Path Evaluation

When the paths are being evaluated, the following conditions are met:

<!-- markdownlint-disable MD013 -->

```none

  ---------------------------------------------
  | Are the source and output paths provided? |
  ---------------------------------------------
      |       |
      v       v
    -----   ------           ----------------
    |yes|   | no | -------> | Exit w/ Error |
    -----   ------           ----------------
      |
      v
  -------------------------------
  | Does the source path exist? |
  -------------------------------
      |       |
      v       v
    -----   ------           ----------------
    |yes|   | no | -------> | Exit w/ Error |
    -----   ------           ----------------
      |
      v
  ------------------------------------
  | Collect provided file extensions |
  ------------------------------------
      |
      v
  -----------------------------------         ---------------------------------------
  | Are the source and output       | ------> | Are they .spf, .deb, or .rpm files? |
  | files supported package types?  |         ---------------------------------------
  -----------------------------------
      |       |
      v       v
    -----   ------           ----------------
    |yes|   | no | -------> | Exit w/ Error |
    -----   ------           ----------------
      |
      v
  ----------------------------------------------------------------
  | Is the user trying to convert two packages of the same type? |
  ----------------------------------------------------------------
      |       |
      v       v
    -----   ------           ----------------
    |yes|   | no | -------> | Exit w/ Error |
    -----   ------           ----------------
      |
      v
  ---------------------------------------------------------------
  | Is the user trying to convert .deb -> .rpm or .deb -> .rpm? |
  ---------------------------------------------------------------
      |       |
      v       v
    -----   ------           ----------------
    |yes|   | no | -------> | Exit w/ Error |
    -----   ------           ----------------
      |
      v
  -------------------------
  | Initialize Conversion |
  -------------------------
```

<!-- markdownlint-enable MD013 -->

### Initialization: Collect Package Information & Metadata

There are a few things spf needs to do before the conversion process starts.

#### Package Type Determination

Before the conversion begins, spf needs to know what package is being converted
to what format.

To do this, the package type is determined via the file extension.

For example, lets say we have a file named `test.spf`. Using
`PackageType::get_from_extension()`, it reads the part of the file name that
says `.spf`, which then returns its corresponding type as `PackageType::Spf`.

#### Loading the Package

The following pieces of information of the package are loaded:

- Location
- Type
- Metadata

The package location and type are stored in struct `Converter`, while metadata
is retrieved separately.

##### Metadata Loading

The loaded metadata is no different from what's collected when installing an spf
package.

There isn't much to this step, the metadata is stored in a local variable,
rather than the struct.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

### Data Conversion

The package metadata is the only thing "converted", while the actual program
files are copied. No conversion needed.

#### Metadata Conversion

Below is a chart showing the metadata categories and their equivalents. Compares
`.spf`, `.rpm`, and `.deb` packages.

<!-- markdownlint-disable MD013 -->

| Category        | .spf Package | .deb Package                                                           | .rpm Package           |
| --------------- | ------------ | ---------------------------------------------------------------------- | ---------------------- |
| Name            | PROJECT_NAME | Package                                                                | Name                   |
| Version         | VERSION      | Version                                                                | Version                |
| Description     | DESCRIPTION  | Description                                                            | Architecture / Summary |
| Download Source | REPOSITORY   | Homepage                                                               | URL                    |
| License         | LICENSE      | **Not Applicable! See [the quirks of the license category](#license)** | License                |
| Maintainer(s)   | AUTHORS      | Maintainer                                                             | Packager               |
| Architecture    | ARCH         | Architecture                                                           | Architecture           |

<!-- markdownlint-enable MD013 -->

You can read more about the package metadata at
[the metadata documentation](metadata.md)

##### Weird Quirks

Due to packaging differences and other limitations, some improvising has to be
done. This specifically affects the license and architecture metadata fields.

###### License

Converting the license category works a bit differently. Since `.deb` packages
do not have a native field that identifies the license, we have to improvise.

When converting `.spf` to `.deb`, the `LICENSE` field will be dropped, as there
is no place for it.

On the other hand, when converting `.deb` to `.spf`, the `LICENSE` field will
contain a placeholder along the lines of:

`Not Applicable (converted using spf)`.

###### Architecture

Between `.spf`, `.rpm`, and `.deb` packages, they have different methods of
identifying architectures. Below is a table comparing them.

| Architecture | .spf Package | .deb Package | .rpm Package       |
| ------------ | ------------ | ------------ | ------------------ |
| Universal    | universal    | all          | noarch, src, nosrc |
| 64-bit       | x86_64       | amd64        | x86_64             |
| 64-bit ARM   | aarch64      | arm64        | aarch64            |
| 32-bit       | x86          | i386         | i686               |
| 32-bit ARM   | arm          | armhf        | armv7hl            |

##### Post-Conversion

After the metadata is converted, it is loaded into memory. After, the original
metadata file is deleted (if applicable).

This becomes helpful when file copying begins.

#### File Copying

This is a really short (depending on the package size) and simple step.

Once the metadata file is gone (if applicable), the rest of the paths inside of
the extracted package are files for the package itself.

Because of that, the extracted package is renamed to whatever the output file
name is (minus the extension).

That's the entire step. No files are actually "copied".

### Build & Package

This is the simplest step throughout the conversion process.

Once all the files are in the correct locations and the metadata has been
converted, the packages can be built.

When converting in any direction, there are specialized functions provided
through a provided crate for `.deb` and `.rpm` packages, while `.spf` packages
simply just create an archive from the directory. See
[`.spf` package creation](./creation.md/#creating-a-spf-package) for more
details.

If needed, the archive is renamed to the output location provided at the
beginnning at the process. See the [path evaluation](#path-evaluation) for more.

---

Last Updated: `October 20, 2026 @ 10:23 AM EST`
