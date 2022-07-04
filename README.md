[![Latest Version][crate-image]][crate-link]
[![Security Status][security-image]][security-link]
[![Build Status][pipeline-image]][pipeline-link]
![MSRV][rustc-image]
[![MIT licensed][license-image]][license-link]

# gitscanr
A simple git repository explorer.

```
USAGE:
    gitscanr [OPTIONS] [directory]

ARGS:
    <directory>    The directory to scan [default: ./]

OPTIONS:
    -h, --help         Print help information
    -r, --recursive    When [directory] is the root of multiple git reopsitories
    -V, --version      Print version information
```

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/gitscanr
[crate-link]: https://crates.io/crates/gitscanr
[security-image]: https://github.com/bilbu/gitscanr/actions/workflows/security.yml/badge.svg
[security-link]: https://github.com/bilbu/gitscanr/actions/workflows/security.yml/
[pipeline-image]: https://github.com/bilbu/gitscanr/actions/workflows/pipeline.yml/badge.svg
[pipeline-link]: https://github.com/bilbu/gitscanr/actions/workflows/pipeline.yml/
[license-image]: https://img.shields.io/crates/l/gitscanr
[rustc-image]: https://img.shields.io/badge/rustc-1.61.0+-blue.svg
[license-link]: https://raw.githubusercontent.com/bilbu/gitscanr/master/LICENSE
