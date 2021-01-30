
[![Latest Version][s1]][l1] [![MIT][s2]][l2] [![Chat on Miaou][s3]][l3]

[s1]: https://img.shields.io/crates/v/rhit.svg
[l1]: https://crates.io/crates/rhit

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://miaou.dystroy.org/static/shields/room.svg
[l3]: https://miaou.dystroy.org/3768?rust


**Rhit** finds and reads nginx log files (even gzipped), does some basic analysis and tells you about it in pretty tables in your console, storing and polluting nothing.

It lets you filter hits by dates, or by patterns on referers and paths.

And it's fast enough (about one second per million lines) so you can iteratively try queries to build your insight.

![filtering](doc/download-filter.png)

# Installation

You need the [Rust](https://rustup.rs) toolchain. Do

```bash
cargo install rhit
```

Rhit is only tested on linux.

# Basic Usage

If rhit is on the server, and the logs are at their usual location:

```bash
rhit
```

(you may have to prefix with sudo to read the files in `/var/log`)

Tell rhit what files to open:

```bash
rhit ~/trav/nginx-logs
```

# Filtering

## Filter on paths

```bash
rhit -p download
```

## Filter on paths with a regular expression

```bash
rhit -p "^/blog/.*broot"
```

## Filter on referer

```bash
rhit -r reddit
```

## Only show a specific day

```bash
rhit -d 12/25
```
If the log contains several years, you need to precise it, eg `rhit -d 2020/12/25`.
Symmetrically, you may omit the month if it's not ambiguous: `rhit -d 25`.

## Only show a period:

```bash
rhit -d 2020/12/25-2021/01/03
```

## Combine filters

![mixed-filtering](doc/mixed-filter.png)

# Choose what to show

The displayed tables (all by default) can be chosen with the `-t` argument.

For example to only show remote adresses and paths, use:

```bash
rhit -t addr,paths
```
(use `rhit --help` for the complete list)

Table *lengths* is decided with the `-l` argument. Use `rhit -l 0` to have just a few lines in the various tables, and `rhit -l 5` for huge tables. Default value is `1`.



