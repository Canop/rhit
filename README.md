
[![Latest Version][s1]][l1] [![MIT][s2]][l2] [![Chat on Miaou][s3]][l3] [![Packaging status][srep]][lrep]

[s1]: https://img.shields.io/crates/v/rhit.svg
[l1]: https://crates.io/crates/rhit

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://miaou.dystroy.org/static/shields/room.svg
[l3]: https://miaou.dystroy.org/3768?rust

[srep]: https://repology.org/badge/tiny-repos/dysk.svg
[lrep]: https://repology.org/project/dysk/versions

![logo](doc/logo-rhit.png)

**[Rhit](https://dystroy.org/rhit)** reads your nginx log files in their standard location(even gzipped), does some analysis and tells you about it in pretty tables in your console, storing and polluting nothing.

It lets you filter hits by dates, status, referers or paths, and does trend analysis.

And it's fast enough (about one second per million lines) so you can iteratively try queries to build your insight.

Here's looking at dates and trends on January hits with status 2xx and 3xx:

![intro](doc/intro.png)


**[Installation instructions and documentation on Rhit's website](https://dystroy.org/rhit)**

