# .spf package metadata

Metadata for a `.spf` package is stored on two separate occasions.

Those being:

- Inside a `.spf` package
- Used as a marker for the package installation.

## Metadata categories

To help identify the package, there are various categories that are read.

Those being:

| Category       | Purpose                                     |
| -------------- | ------------------------------------------- |
| `PROJECT_NAME` | The name of the package or your project     |
| `VERSION`      | The version of your package                 |
| `DESCRIPTION`  | Description of your project or package      |
| `LICENSE`      | What the code is licensed under             |
| `AUTHORS`      | The people who made the code or packaged it |
| `ARCH`         | The target system architecture for the code |

## Inside the .spf package

Inside the `.spf` package, the metadata is there which is later copied.

It only contains the metadata categories, like as seen above.

[See the available categories](#metadata-categories)

It looks like this:

```none
PROJECT_NAME = package_name
VERSION = v0.1.0
DESCRIPTION = Sample package
LICENSE = gplv3
AUTHORS = User Name
ARCH = x86_64
```

## Package marker

When a `.spf` package is installed, the metadata file inside is copied before
anything else.

After the metadata file is read, it is moved to `/usr/share/spf/packages/` to
serve as a marker to show a record that the package is installed and exists.

It's also relatively the same to the metadata stored inside the `.spf` package,
except it includes the locations of the installed files.

### Category Storage

The first half contains the metadata categories, like as seen above.

[See the available categories](#metadata-categories)

### Path storage

The defined paths are initialized by the header `:::PATH DEFINE START:::`.
Anything below it is a path to a copied file.

### Sample

Here is a sample metadata marker file:

```none
### PACKAGED WITH SPF v0.5.1 ###

PROJECT_NAME = package_name
VERSION = v0.1.0
DESCRIPTION = Sample package
LICENSE = gplv3
AUTHORS = User Name
ARCH = x86_64
:::PATH DEFINE START:::
/usr/bin/binary
/usr/share/licenses/package_name/LICENSE
```

## Others

There are also a couple other things to mention, such as the header, which
mentions which version of spf it was packaged with. It looks like this:

`### PACKAGED WITH SPF vX.X.X ###`

---

Last Updated: `August 29, 2026 @ 3:10 PM EST`
