# Converting To and From .spf files

The provided command `convert` allows you do convert `.spf` files to `.deb`
files, and vice versa.

The following is converted:

- Package formats
- Metadata

The way paths are copied remains the same for both directions.

## Conversion

Whether you are converting `.spf` to `.rpm`, or `.deb` -> `.rpm`, conversion
follows this flow:

```none
Path Evaluation -> Init Conversion -> Collect Metadata -> Build & Package
```

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

### Init conversion

There are few steps that must be followed before conversion is initialized.

> **TODO:** Make flowchart of events

---

Last Updated: `October 15, 2026 @ 5:05 PM EST`
