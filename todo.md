# spf TODO

> [!IMPORTANT]  
> Last updated: `September 24, 2026 @ 07:50 AM EST`

Related:

- [SPF Roadmap](roadmap.md)
- [SPF Changelog](changes.md)

---

- [src/convert.rs](src/convert.rs)
  - Finish adding conversions:
    - `.deb` -> `.spf` :check:
    - `.spf` -> `.rpm`
    - `rpm` -> `.spf`
  - Documents structs and enums and stuff
  - Fix Debian package metadata being collected as empty
- [src/main.rs](src/main.rs)
  - Fix command/argument/flag parsing
