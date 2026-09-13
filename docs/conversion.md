# Converting To and From .spf files

The provided command `convert` allows you do convert `.spf` files to `.deb`
files, and vice versa.

The following is converted:

- Package formats
- Metadata

The way paths are copied remains the same for both directions.

## .deb -> .spf

It starts out by extracting the selected `.spf` file (seen in
[src/convert.rs](src/convert.rs) as `source_package_path`.)

It should look something like this:

```none
source_package.deb
    L control.tar.xz
    L data.tar.xz
    L debian-binary
```

Then `debian-binary` is deleted. Converting to `.spf` does not need that file.

### Extracting and Loading Metadata

When the source `.spf` file is extracted, extract `control.tar.xz`. This holds
the metadata inside the `.deb` file, and reveals the singular `control` file.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

#### Loading

The metadata inside `control` is loaded into memory first. After, the `control`
file is deleted.

#### Conversion

With the metadata loaded, the category headers are converted to the `.spf`
counterparts.

For example, `Package` is changed to `PROJECT_NAME`, `Version` is changed to
`VERSION`, and so on.

For example,

This...:

```none
Package: sample name
Version: sample version
Architecture: sample arch
Maintainer: sample name <sample email>
Description: sample desc
```

...is converted to this:

```none
PROJECT_NAME = sample name
VERSION = sample version
ARCH = sample arch
AUTHORS = sample name <sample email>
DESCRIPTION = sample desc
```

Because there is no proper `LICENSE` equivalent category, just drop it.

When the category `Architecture` is encountered, convert the `.deb` architecture
names to the `.spf` counterparts.

If something is encountered that isn't allowed or available, comment that line
with a `#`. After the metadata is finished converting, any line that starts with
`#` is removed.

### Extracting Data

This is a very quick process. Instead of copying files to a location, simply
rename the extracted `.deb` to the output `.spf` file name (minus the
extension).

Because `debian-binary` and `control.tar.xz` have already been deleted, there is
no problem with that.

### Packaging and Saving

The metadata collected earlier is saved into the now to-be-archived directory.

At this point, the contents of that directory is:

- The package filesystem
- The metadata file (`META`)

It looks something like this:

```none
package.spf
    L /usr
    L META
```

Finally, the directory is compressed into the file output, which serves as the
`.spf` package.

## .spf -> .deb

This process has much less steps due to the advantage of using external crates
and already-established libraries.

### Metadata Conversion and Loading

When the source `.spf` is extracted, the metadata is first loaded into memory.
Then the file is deleted.

Now with the metadata in memory, the only thing that has to be converted is the
package architecture.

The following metadata is loaded:

- Name
- Version
- Description
- Maintainer
- Homepage
- Architecture

The loaded metadata is then applied to the `.deb` package.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

### Paths

Paths located inside the extracted `.spf` file are cycled and added to the list
of what paths are to be added.

It's a quick and simple process.

### Building Package

The package is finally built to the output location.

## Risks

There are a few risks when installing an spf-converted package. Those could be:

- Lost files
- Potential incorrect path mapping

What is why before any package is converted, a disclaimer is shown to basically
say that `spf` has no warranty and is not responsible for any damage.

You should only install these converted packages if you know what you are doing.

---

Last Updated: `September 13, 2026 @ 1:54 PM EST`
