# .spf package config

The custom configuration format is used to define the characteristics of the
package.

It's stored in two separate categories: - Package metadata - Defined paths to be
copied into the package.

## Define Metadata Values

> [!TIP]  
> See the [full metadata documentation](metadata.md) for more context on package
> metadata.

The package metadata is stored inside these two headers:

- `:::META DEFINE START:::`
- `:::META DEFINE END:::`

Any metadata found outside those two headers will likely be ignored, or have spf
fail to finish packaging.

### Sample Metadata Configuration

```none
:::META DEFINE START:::
PROJECT_NAME = package_name
VERSION = vX.X.X
DESCRIPTION = Sample package
LICENSE = gplv3
AUTHORS = User Name
ARCH = x86_64
:::META DEFINE END:::
```

## Define Path Values

These path values are determine which file gets packaged where.

Like the package metadata, the path defines are stored inside these two headers:

- `:::PATH DEFINE START:::`
- `:::PATH DEFINE END:::`

### Entry Formatting

The path entries have the original file (the one to be copied) on the right, and
the destination on the left.

The two paths are separated with a `:` (colon).

#### Example Path Entry Formatting

Here is an example path entry.

The original file (on the left) will be `./target/release/binary`, and the
destination of that file will be `/usr/bin/binary`.

`./target/release/binary:/usr/bin/binary`

### Sample Path Configuration

```none
:::PATH DEFINE START:::
./target/release/binary:/usr/bin/binary
LICENSE:/usr/share/licenses/package_name/LICENSE
:::PATH DEFINE END:::
```

## Sample Full Package Config

Here is a complete package config you are able to use as a template.

> [!TIP] You can also generate a package config by running
> `$ template <(optional) output location>`

```none
:::META DEFINE START:::
PROJECT_NAME = package_name
VERSION = vX.X.X
DESCRIPTION = Sample package
LICENSE = gplv3
AUTHORS = User Name
ARCH = x86_64
:::META DEFINE END:::

:::PATH DEFINE START:::
./target/release/binary:/usr/bin/binary
LICENSE:/usr/share/licenses/package_name/LICENSE
:::PATH DEFINE END:::
```

---

Last Updated: `August 29, 2026 @ 3:45 PM EST`
