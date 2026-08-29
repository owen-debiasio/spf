# Creating a .spf package

`.spf` packages are created by collecting a tree of directories, and a metadata
file to it's own directory.

If copying finishes, it moves on to creating the actual `.spf` package, which is
basically a renamed `.tar` file.

## Initializing the Creation

To create a `.spf` package you need a
[package configuration file](package_config.md).

If the format of the config file is correct, you will be able to run the
creation command:

`create <metadata file> <output directory>`

### Example Packaging Command

This loads the configuration file `package_config`, and the output package name
will be `package_linux_x86_64.spf`.

```bash
spf create package_config package_linux_x86_64.spf
```

If created correctly, you should be able to find the file named:
`package_linux_x86_64.spf`

## Structure of a .spf Package

The only file that is absolutely needed by spf is the metadata file, named as
`META`.

Otherwise, the package root is like the root of the filesystem. It can contain
directories such as `/var`, `/usr`, and others.

### Sample Package Contents

A basic graph or a `.spf` package is shown below

```none
package_name-v0.1.0-x86_64
    L /usr
        L /bin
            L binary
        L /share
            L /package_name
                L /licenses
                    L LICENSE
    L META
```

## Recording the Installation

Once installed, the packages metadata is stored at a special local repository
containing the metadata of installed packages.

They are moved to `/usr/share/spf/packages`, where they are used as markers to
basically say _"Hey, I exist!"_.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

---

Last Updated: `August 29, 2026 @ 3:45 PM EST`
