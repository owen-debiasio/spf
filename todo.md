# spf TODO

> [!IMPORTANT]  
> Last updated: `September 26, 2026 @ 07:20 PM EST`

Related:

- [SPF Roadmap](roadmap.md)
- [SPF Changelog](changes.md)

---

## Current goals for v0.7.0

- [src/convert.rs](src/convert.rs)
  - Finish adding conversions:
    - `.deb` -> `.spf` :check:
    - `.spf` -> `.rpm`
    - `rpm` -> `.spf` (work in progress)
  - Document structs and enums and stuff
  - Fix `spf` packages being archived with wrong name when being converted from
    `.rpm` to `.spf`
- [src/main.rs](src/main.rs)
  - Fix command/argument/flag parsing
