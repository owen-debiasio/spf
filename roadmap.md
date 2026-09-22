# spf Roadmap

> [!IMPORTANT]  
> Last updated: `September 22, 2026 @ 05:46 PM EST`

Here's a todo list that I've thrown together of things I want to add/change over
time.

Related:

- [TODO list](todo.md)
- [SPF Changelog](changes.md)

## Work in progress

- Add converting `.spf` packages -> `.rpm` packages (and vice versa)

## List of proposed features

- Pre/Post install/uninstall jobs
  - Such as running scripts, or handling things spf doesn't
- Intel x86_64 and M-series Apple Silicon (aarch64) Mac support
- Command `verify`
  - Verifies one of the following:
    - `.spf` package metadata
    - Package config
    - Installed package metadata

## Previously Completed

Here are some features that were previously on here, and are now completed.

| Feature                                              | Version Completed |
| ---------------------------------------------------- | ----------------- |
| Converting `.spf` and `.deb` packages back and forth | v0.6.0            |
