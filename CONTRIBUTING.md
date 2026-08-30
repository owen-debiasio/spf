# Contributing

Hi! If you're looking for resources to help with the development of spf, this is
the place for you!

Please note that this project is released with a
[Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this
project you agree to abide by its terms.

## Issues and PRs

If you have suggestions for how this project could be improved, or want to
report a bug, open an issue! I'd love all and any contributions. If you have
questions, too, I'd love to hear them.

I'd also love PRs. If you're thinking of a large PR, I advise opening up an
issue first to talk about it, though! Look at the links below if you're not sure
how to open a PR.

## Submitting a pull request

1. [Fork](/fork) and clone the repository.
2. Configure and install the dependencies: `$ cargo build`.
3. Make sure the code is linted and formatted by running
   [`$ ./dev/format.bash`](dev/format.bash)
4. Create a new branch: `$ git checkout -b my-branch-name`.
5. Make your change, add tests, and make sure the tests still pass.
6. Push to your fork and [submit a pull request](/compare).
7. Pat your self on the back and wait for your pull request to be reviewed and
   merged.

Here are a few things you can do that will increase the likelihood of your pull
request being accepted:

- Follow the style guide ([Rust](https://doc.rust-lang.org/style-guide/),
  [Markdown](https://www.markdownguide.org/basic-syntax/),
  [Bash](https://google.github.io/styleguide/shellguide.html)). Any linting
  errors should be shown when running [`$ ./dev/format.bash`](dev/format.bash).
- Write and update tests.
- Keep your changes as focused as possible. If there are multiple changes you
  would like to make that are not dependent upon each other, consider submitting
  them as separate pull requests.
- Write a
  [good commit message](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html).

Work in Progress pull requests are also welcome to get feedback early on, or if
there is something blocked you.

## Resources

- [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/)
- [Using Pull Requests](https://help.github.com/articles/about-pull-requests/)
- [GitHub Help](https://help.github.com)
