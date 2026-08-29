# spf changelog

## v0.1.0

- Initial release

## v0.2.0

<details>
<summary>Show details</summary>

- Add script to clean up generated packages in `./packages/`
- Add command `template`: Used to generate package metadata template
- Added safeguard when removing spf using spf.
  - If you want to remove spf using spf, you must use the standalone binary.
- Enhanced the cleaning functionality of `./dev/package.sh` and
  `./package/clean.bash`
- Enhanced error reporting
- Many, many code optimizations
- Fix files not being deleted when removing a package
- Add a header at the top of a packages META file displaying what version of spf
  that package was packaged with
- Fix issues with file copying and installing

</details>

## v0.3.0

<details>
<summary>Show details</summary>

- Changes to command `list`:
  - If there are no commands installed, an error will now be thrown
  - Other tweaks
- Code optimizations
- Add command `inspect`: Inspect metadata of a package
  - Can inspect either a `.spf` package or an already installed package
  - Usage: `$ spf inspect <package to inspect>`
- Add support for `.deb` and `.rpm` packages
- Add more suggested tools for linting/formatting

</details>

## v0.4.0

<details>
<summary>Show details</summary>

- Improve output when listing commands to now let you know that a command
  doesn't match the input string
- Code optimizations
- Enhanced and optimized errors.
  - Use native functions like `panic!()` or `.expect()` instead of custom error
    handlers to avoid complexity and improve code readability
- Add logo to readme, located at `assets/logo.png`
- You can now easily update spf from a `.spf` package
- `./dev/install.bash` now runs `./dev/format.bash`
- You can now remove spf using spf
- Add Github workflows for Bash scripts, Rust code, and Markdown files

</details>

## v0.5.0

- Improve code documentation
- Improve how metadata is retrieved using a more efficient method
- Add support for Linux `aarch64` systems
- Add flag `--ignore-args`: Allows you to the installation of a package with
  architecture
- You will be blocked from installing a `.spf` package with a different
  architecture unless you pass `--ignore-args`.
- Allow details to be hidden for older changelog entries

## v0.5.1

- Optimized the listing of packages when you are removing packages
- Code optimizations
- A list of supported architectures must be fulfilled when packaging.
- The existence of command `tar` is checked during the init process
