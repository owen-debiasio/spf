# Creating a .spf package

`.spf` packages are created by collecting a tree of directories, and a metadata
file to it's own directory.

If copying finishes, it moves on to creating the actual `.spf` package, which is
basically a renamed `.tar` file.

## Structure of a .spf package

The only file that is absolutely needed by spf is the metadata file, named as
`META`.

Otherwise, the package root is like the root of the filesystem. It can contain
directories such as `/var`, `/usr`, and others.

### Sample package contents

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

## Recording the installation

Once installed, the packages metadata is stored at a special local repository
containing the metadata of installed packages.

They are moved to `/usr/share/spf/packages`, where they are used as markers to
basically say _"Hey, I exist!"_.

You can read more about the package metadata at
[the metadata documentation](metadata.md)

---

Last Updated: `August 29, 2026 @ 3:10 PM EST`
