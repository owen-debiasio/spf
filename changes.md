# spf changelog

## v0.1.0

- Initial release

## v0.2.0

- Add script to clean up generated packages in `./packages/`
- Add command `template`: Used to generate package metadata template
- Added safeguard when removing spf using spf.
  - If you want to remove spf using spf, you must use the standalone binary.
- Enhanced the cleaning functionality of `./dev/package.sh` and `./package/clean.bash`
- Enhanced error reporting
- Many, many code optimizations
- Fix files not being deleted when removing a package
- Add a header at the top of a packages META file displaying what version of spf
that package was packaged with
- Fix issues with file copying and installing

## v0.2.1

- Changes to command `list`:
  - If there are no commands installed, an error will now be thrown
  - Other tweaks
- Code optimizations
- Add command `inspect`: Inspect metadata of a package
  - Can inspect either a `.spf` package or an already installed package
  - Usage: `$ spf inspect <package to inspect>`
- Add support for `.deb` and `.rpm` packages
