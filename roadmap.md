# spf Roadmap

> [!IMPORTANT]  
> Last updated: `November 07, 2026 @ 02:51 PM EST`

Here's an idea list that I've thrown together of things I want to add/change
over time. These are only IDEAS and are not guaranteed to be implemented.

Related:

- [TODO list](todo.md)
- [SPF Changelog](changes.md)

## Work in progress

These are some features that are currently being worked on. Things may be
delayed, removed, or added to the list.

| Feature                           | Priority | Due Date (version) |
| --------------------------------- | -------- | ------------------ |
| Restructure and clean up codebase | Medium   | v1.0.0             |

## List of Proposed Features

This is an ambitious list of features I want to add to spf. Entries can be
dropped out of the list at any point.

> [!NOTE]  
> If the entry due date is `NaN`, this means that it is not guaranteed, and are
> more likely drop out of the list at any point.

<!-- markdownlint-disable MD013 -->

| Feature                                                               | Priority  | Due Date (version) |
| --------------------------------------------------------------------- | --------- | ------------------ |
| Command `verify`: Verify `.spf`/installed package metadata and config | Low       | v0.8.0             |
| Pre/Post install/uninstall jobs                                       | Undecided | v0.9.0             |
| Unit Tests (via `cargo test`)                                         | Undecided | v0.9.0             |
| Move argument parsing system to `clap`, rather than my custom system. | Undecided | v1.0.0             |
| Add a `.toml` configuration system for spf cli                        | Undecided | v1.0.0             |
| Ability to install packages through a remote repository               | Undecided | v1.0.0             |
| Move to a `.toml`-based package configuration/metadata                | Undecided | v1.0.0             |
| Remote package repository support (Ubuntu PPA, AUR, Cargo)            | NaN       | NaN                |
| Intel x86_64 and M-series Apple Silicon (aarch64) MacOS support       | NaN       | NaN                |
| GUI Package install tool (GTK4)                                       | NaN       | NaN                |

<!-- markdownlint-enable MD013 -->

## Previously Completed

Here are some features that were previously on here, and are now completed.

| Feature                                          | Version Completed |
| ------------------------------------------------ | ----------------- |
| Conversion in between `.spf` and `.deb` packages | v0.6.0            |
| Conversion in between `.spf` and `.rpm` packages | v0.7.0            |
