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
3. Copy Files & Convert Metadata
4. Build & Package

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

##### Metadata

The loaded metadata is no different from what's collected when installing an spf
package.

There isn't much to this step, the metadata is stored in a local variable,
rather than the struct.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

### Copy Files & Convert Metadata

TODO: Do this step

---

Last Updated: `October 17, 2026 @ 6:41 PM EST`
